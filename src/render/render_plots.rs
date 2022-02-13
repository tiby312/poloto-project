use super::*;

pub fn render_plots<X: PlotNumContext, Y: PlotNumContext>(
    canvas: &Canvas,
    writer: impl std::fmt::Write,
    plotter: &mut Plotter<X, Y>,
    data: &Data<X::Num, Y::Num, X::StepInfo, Y::StepInfo>,
) -> std::fmt::Result {
    let Canvas {
        width,
        height,
        padding,
        paddingy,
        aspect_offset,
        scalex,
        scaley,
        spacing,
        legendx1,
        num_css_classes,
        ..
    } = *canvas;

    let [minx, maxx] = data.boundx;
    let [miny, maxy] = data.boundy;

    let mut writer = tagger::new(writer);

    let xcontext = plotter.xcontext.as_mut().unwrap();
    let ycontext = plotter.ycontext.as_mut().unwrap();

    let mut color_iter = {
        let max = if let Some(nn) = num_css_classes {
            nn
        } else {
            usize::MAX
        };

        (0..max).cycle()
    };

    for (i, mut p) in plotter.plots.plots.drain(..).enumerate() {
        let legendy1 = paddingy - padding / 8.0 + (i as f64) * spacing;

        let name_exists = writer
            .elem("text", |d| {
                d.attr("class", "poloto_text poloto_legend_text")?;
                d.attr("alignment-baseline", "middle")?;
                d.attr("text-anchor", "start")?;
                d.attr("font-size", "large")?;
                d.attr("x", width - padding / 1.2)?;
                d.attr("y", paddingy + (i as f64) * spacing)
            })?
            .build(|d| {
                let mut wc = util::WriteCounter::new(d.writer_safe());
                p.plots.write_name(&mut wc)?;
                Ok(wc.get_counter() != 0)
            })?;

        let aa = minx.default_scale([minx, maxx], scalex);
        let bb = miny.default_scale([miny, maxy], scaley);

        struct PlotIter<X: PlotNumContext, Y: PlotNumContext> {
            basex_ii: f64,
            basey_ii: f64,
            rangex_ii: [X::Num; 2],
            rangey_ii: [Y::Num; 2],
            maxx_ii: f64,
            maxy_ii: f64,
        }
        impl<X: PlotNumContext, Y: PlotNumContext> PlotIter<X, Y> {
            fn gen_iter<'a>(
                &'a self,
                p: &'a mut Plot<X::Num, Y::Num>,
                xcontext: &'a mut X,
                ycontext: &'a mut Y,
            ) -> impl Iterator<Item = [f64; 2]> + 'a {
                p.plots.iter_second().map(move |(mut x, mut y)| {
                    [
                        self.basex_ii + x.default_scale(self.rangex_ii, self.maxx_ii),
                        self.basey_ii - y.default_scale(self.rangey_ii, self.maxy_ii),
                    ]
                })
            }
        }

        let plot_iter = PlotIter {
            basex_ii: aspect_offset + padding - aa,
            basey_ii: height - paddingy + bb,
            rangex_ii: [minx, maxx],
            rangey_ii: [miny, maxy],
            maxx_ii: scalex,
            maxy_ii: scaley,
        };

        let colori = color_iter.next().unwrap();

        match p.plot_type {
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
                    d.path(|a| render::line(a, plot_iter.gen_iter(&mut p, xcontext, ycontext)))
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
                            .gen_iter(&mut p, xcontext, ycontext)
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
                            .gen_iter(&mut p, xcontext, ycontext)
                            .filter(|&[x, y]| x.is_finite() && y.is_finite())
                        {
                            if let Some((lx, ly)) = last {
                                writer.single("rect", |d| {
                                    d.attr("x", lx)?;
                                    d.attr("y", ly)?;
                                    d.attr(
                                        "width",
                                        (padding * 0.02).max((x - lx) - (padding * 0.02)),
                                    )?;
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
                        render::line_fill(
                            path,
                            plot_iter.gen_iter(&mut p, xcontext, ycontext),
                            height - paddingy,
                            true,
                        )
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
                        render::line_fill(
                            path,
                            plot_iter.gen_iter(&mut p, xcontext, ycontext),
                            height - paddingy,
                            false,
                        )
                    })
                })?;
            }
        }
    }

    Ok(())
}
