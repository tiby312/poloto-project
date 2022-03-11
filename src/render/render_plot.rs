use super::*;

pub(crate) fn render_plot<X: PlotNum, Y: PlotNum>(
    writer: impl std::fmt::Write,
    extra: &Extra<X, Y>,
    plots_all: impl AllPlotFmt<Item2 = (X, Y)>,
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
    } = extra.canvas;

    let boundx = [extra.boundx.min, extra.boundx.max];
    let boundy = [extra.boundy.min, extra.boundy.max];

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

    for (i, mut ppp) in plots_all.iter().enumerate() {
        let legendy1 = paddingy - yaspect_offset - padding / 8.0 + (i as f64) * spacing;

        let name_exists = writer
            .elem("text", |d| {
                d.attr("class", "poloto_text poloto_legend_text")?;
                d.attr("x", width - padding / 1.2)?;
                d.attr("y", paddingy - yaspect_offset + (i as f64) * spacing)
            })?
            .build(|d| {
                let mut wc = util::WriteCounter::new(d.writer_safe());
                ppp.fmt(&mut wc)?;
                //p.write_name(&mut wc)?;
                Ok(wc.get_counter() != 0)
            })?;

        let aa = minx.scale([minx, maxx], scalex);
        let bb = miny.scale([miny, maxy], scaley);

        match ppp.plot_type() {
            PlotMetaType::Text => {
                // don't need to render any legend or plots
            }
            PlotMetaType::Plot(p_type) => {
                let colori = color_iter.next().unwrap();

                let it = {
                    let basex_ii = xaspect_offset + padding - aa;
                    let basey_ii = yaspect_offset + height - paddingy + bb;
                    let rangex_ii = [minx, maxx];
                    let rangey_ii = [miny, maxy];
                    let maxx_ii = scalex;
                    let maxy_ii = scaley;

                    ppp.get_iter().map(move |(x, y)| {
                        [
                            basex_ii + x.scale(rangex_ii, maxx_ii),
                            basey_ii - y.scale(rangey_ii, maxy_ii),
                        ]
                    })
                };

                render(
                    &mut writer,
                    it,
                    PlotRenderInfo {
                        canvas: &extra.canvas,
                        p_type,
                        name_exists,
                        colori,
                        legendy1,
                    },
                )?;
            }
        }
    }

    Ok(())
}

struct PlotRenderInfo<'a> {
    canvas: &'a Canvas,
    p_type: PlotType,
    name_exists: bool,
    colori: usize,
    legendy1: f64,
}

fn render<W: fmt::Write>(
    writer: &mut tagger::ElemWriter<W>,
    it: impl Iterator<Item = [f64; 2]>,
    info: PlotRenderInfo,
) -> fmt::Result {
    let PlotRenderInfo {
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
                d.path(|a| render::line(a, it))
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
                    for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
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
                    for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
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
                d.path(|path| render::line_fill(path, it, height - paddingy, true))
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
                d.path(|path| render::line_fill(path, it, height - paddingy, false))
            })?;
        }
    }
    Ok(())
}
