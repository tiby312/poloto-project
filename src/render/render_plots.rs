use super::*;

pub fn render_plots<X: PlotNum, Y: PlotNum>(
    writer: impl std::fmt::Write,
    plots: &mut DataResult<X, Y>,
) -> std::fmt::Result {
    let Canvas {
        width,
        height,
        padding,
        paddingy,
        xaspect_offset,
        yaspect_offset,
        scalex,
        scaley,
        spacing,
        num_css_classes,
        ..
    } = plots.canvas;

    let boundx = [plots.boundx.min, plots.boundx.max];
    let boundy = [plots.boundy.min, plots.boundy.max];

    let [minx, maxx] = boundx;
    let [miny, maxy] = boundy;

    let mut writer = tagger::new(writer);

    let mut color_iter = {
        let max = if let Some(nn) = num_css_classes {
            nn
        } else {
            usize::MAX
        };

        (0..max).cycle()
    };

    for (i, mut p) in plots.plots.drain(..).enumerate() {
        let legendy1 = paddingy - yaspect_offset - padding / 8.0 + (i as f64) * spacing;

        let name_exists = writer
            .elem("text", |d| {
                d.attr("class", "poloto_text poloto_legend_text")?;
                d.attr("alignment-baseline", "middle")?;
                d.attr("text-anchor", "start")?;
                d.attr("font-size", "large")?;
                d.attr("x", width - padding / 1.2)?;
                d.attr("y", paddingy - yaspect_offset + (i as f64) * spacing)
            })?
            .build(|d| {
                let mut wc = util::WriteCounter::new(d.writer_safe());
                p.write_name(&mut wc)?;
                Ok(wc.get_counter() != 0)
            })?;

        let aa = minx.scale([minx, maxx], scalex);
        let bb = miny.scale([miny, maxy], scaley);

        match p.plot_type() {
            PlotMetaType::Text => {
                // don't need to render any legend or plots
            }
            PlotMetaType::Plot(p_type) => {
                let colori = color_iter.next().unwrap();

                let plot_iter = PlotIter {
                    basex_ii: xaspect_offset + padding - aa,
                    basey_ii: yaspect_offset + height - paddingy + bb,
                    rangex_ii: [minx, maxx],
                    rangey_ii: [miny, maxy],
                    maxx_ii: scalex,
                    maxy_ii: scaley,
                };

                render(PlotRenderInfo {
                    writer: &mut writer,
                    p: p.as_mut(),
                    plot_iter,
                    canvas: &plots.canvas,
                    p_type,
                    name_exists,
                    colori,
                    legendy1,
                })?;
            }
        }
    }

    Ok(())
}

struct PlotIter<X: PlotNum, Y: PlotNum> {
    basex_ii: f64,
    basey_ii: f64,
    rangex_ii: [X; 2],
    rangey_ii: [Y; 2],
    maxx_ii: f64,
    maxy_ii: f64,
}
impl<X: PlotNum, Y: PlotNum> PlotIter<X, Y> {
    fn gen_iter<'a>(
        &'a self,
        p: &'a mut dyn PlotTrait<Item = (X, Y)>,
    ) -> impl Iterator<Item = [f64; 2]> + 'a {
        p.iter_second().map(move |(x, y)| {
            [
                self.basex_ii + x.scale(self.rangex_ii, self.maxx_ii),
                self.basey_ii - y.scale(self.rangey_ii, self.maxy_ii),
            ]
        })
    }
}

struct PlotRenderInfo<'a, W: fmt::Write, X: PlotNum, Y: PlotNum> {
    writer: &'a mut tagger::ElemWriter<W>,
    p: &'a mut dyn PlotTrait<Item = (X, Y)>,
    plot_iter: PlotIter<X, Y>,
    canvas: &'a Canvas,
    p_type: PlotType,
    name_exists: bool,
    colori: usize,
    legendy1: f64,
}

fn render<W: fmt::Write, X: PlotNum, Y: PlotNum>(info: PlotRenderInfo<W, X, Y>) -> fmt::Result {
    let PlotRenderInfo {
        writer,
        p,
        plot_iter,
        canvas,
        p_type,
        name_exists,
        colori,
        legendy1,
    } = info;

    let Canvas {
        height,
        padding,
        paddingy,
        legendx1,
        ..
    } = *canvas;

    match p_type {
        PlotType::Line => {
            if name_exists {
                writer.single("line", |d| {
                    d.attr(
                        "class",
                        format_args!(
                            "poloto_line poloto_legend_icon poloto{}stroke poloto{}legend",
                            colori, colori
                        ),
                    )?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", legendx1)?;
                    d.attr("x2", legendx1 + padding / 3.0)?;
                    d.attr("y1", legendy1)?;
                    d.attr("y2", legendy1)
                })?;
            }

            writer.single("path", |d| {
                d.attr("class", format_args!("poloto_line poloto{}stroke", colori))?;
                d.attr("fill", "none")?;
                d.attr("stroke", "black")?;
                d.path(|a| render::line(a, plot_iter.gen_iter(p)))
            })?;
        }
        PlotType::Scatter => {
            if name_exists {
                writer.single("line", |d| {
                    d.attr(
                        "class",
                        format_args!(
                            "poloto_scatter poloto_legend_icon poloto{}stroke poloto{}legend",
                            colori, colori
                        ),
                    )?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", legendx1 + padding / 30.0)?;
                    d.attr("x2", legendx1 + padding / 30.0)?;
                    d.attr("y1", legendy1)?;
                    d.attr("y2", legendy1)
                })?;
            }

            writer.single("path", |d| {
                d.attr(
                    "class",
                    format_args!("poloto_scatter poloto{}stroke", colori),
                )?;
                d.path(|a| {
                    use tagger::PathCommand::*;
                    for [x, y] in plot_iter
                        .gen_iter(p)
                        .filter(|&[x, y]| x.is_finite() && y.is_finite())
                    {
                        a.put(M(x, y))?;
                        a.put(H_(0))?;
                    }
                    Ok(())
                })
            })?;
        }
        PlotType::Histo => {
            if name_exists {
                writer.single("rect", |d| {
                    d.attr(
                        "class",
                        format_args!(
                            "poloto_histo poloto_legend_icon poloto{}fill poloto{}legend",
                            colori, colori
                        ),
                    )?;
                    d.attr("x", legendx1)?;
                    d.attr("y", legendy1 - padding / 30.0)?;
                    d.attr("width", padding / 3.0)?;
                    d.attr("height", padding / 20.0)?;
                    d.attr("rx", padding / 30.0)?;
                    d.attr("ry", padding / 30.0)
                })?;
            }

            writer
                .elem("g", |d| {
                    d.attr("class", format_args!("poloto_histo poloto{}fill", colori))
                })?
                .build(|writer| {
                    let mut last = None;
                    for [x, y] in plot_iter
                        .gen_iter(p)
                        .filter(|&[x, y]| x.is_finite() && y.is_finite())
                    {
                        if let Some((lx, ly)) = last {
                            writer.single("rect", |d| {
                                d.attr("x", lx)?;
                                d.attr("y", ly)?;
                                d.attr("width", (padding * 0.02).max((x - lx) - (padding * 0.02)))?;
                                d.attr("height", height - paddingy - ly)
                            })?;
                        }
                        last = Some((x, y))
                    }
                    Ok(())
                })?;
        }
        PlotType::LineFill => {
            if name_exists {
                writer.single("rect", |d| {
                    d.attr(
                        "class",
                        format_args!(
                            "poloto_linefill poloto_legend_icon poloto{}fill poloto{}legend",
                            colori, colori
                        ),
                    )?;
                    d.attr("x", legendx1)?;
                    d.attr("y", legendy1 - padding / 30.0)?;
                    d.attr("width", padding / 3.0)?;
                    d.attr("height", padding / 20.0)?;
                    d.attr("rx", padding / 30.0)?;
                    d.attr("ry", padding / 30.0)
                })?;
            }

            writer.single("path", |d| {
                d.attr(
                    "class",
                    format_args!("poloto_linefill poloto{}fill", colori),
                )?;
                d.path(|path| {
                    render::line_fill(path, plot_iter.gen_iter(p), height - paddingy, true)
                })
            })?;
        }
        PlotType::LineFillRaw => {
            if name_exists {
                writer.single("rect", |d| {
                    d.attr(
                        "class",
                        format_args!(
                            "poloto_linefillraw poloto_legend_icon poloto{}fill poloto{}legend",
                            colori, colori
                        ),
                    )?;
                    d.attr("x", legendx1)?;
                    d.attr("y", legendy1 - padding / 30.0)?;
                    d.attr("width", padding / 3.0)?;
                    d.attr("height", padding / 20.0)?;
                    d.attr("rx", padding / 30.0)?;
                    d.attr("ry", padding / 30.0)
                })?;
            }

            writer.single("path", |d| {
                d.attr(
                    "class",
                    format_args!("poloto_linefillraw poloto{}fill", colori),
                )?;
                d.path(|path| {
                    render::line_fill(path, plot_iter.gen_iter(p), height - paddingy, false)
                })
            })?;
        }
    }
    Ok(())
}
