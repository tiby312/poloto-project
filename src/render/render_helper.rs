use super::*;

pub fn line_fill<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
    base_line: f64,
) {
    if let Some([startx, starty]) = it.next() {
        use tagger::PathCommand::*;

        let mut last = [startx, starty];
        let mut last_finite = None;
        let mut first = true;
        for [newx, newy] in it {
            match (
                newx.is_finite() && newy.is_finite(),
                last[0].is_finite() && last[1].is_finite(),
            ) {
                (true, true) => {
                    if first {
                        path.put(M(last[0], base_line));
                        path.put(L(last[0], last[1]));

                        first = false;
                    }
                    last_finite = Some([newx, newy]);
                    path.put(L(newx, newy));
                }
                (true, false) => {
                    path.put(M(newx, newy));
                }
                (false, true) => {
                    path.put(L(last[0], base_line));
                }
                _ => {}
            };
            last = [newx, newy];
        }
        if let Some([x, _]) = last_finite {
            path.put(L(x, base_line));
            path.put(Z(""));
        }
    }
}

pub fn line<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
) {
    if let Some([startx, starty]) = it.next() {
        use tagger::PathCommand::*;

        let mut last = [startx, starty];
        let mut first = true;
        for [newx, newy] in it {
            match (
                newx.is_finite() && newy.is_finite(),
                last[0].is_finite() && last[1].is_finite(),
            ) {
                (true, true) => {
                    if first {
                        path.put(M(last[0], last[1]));
                        first = false;
                    }
                    path.put(L(newx, newy));
                }
                (true, false) => {
                    path.put(M(newx, newy));
                }
                _ => {}
            };
            last = [newx, newy];
        }
    }
}

///
/// Draw the axis lines, and tick intervals
///
pub(super) fn draw_base<X: PlotNumber, Y: PlotNumber, T: fmt::Write>(
    plotter: &mut Plotter<X, Y>,
    writer: &mut tagger::ElemWriter<T>,
    dd: DrawData,
    sd: ScaleData<X, Y>,
) {
    let DrawData {
        width,
        height,
        padding,
        paddingy,
    } = dd;
    let ScaleData {
        minx,
        maxx,
        miny,
        maxy,
        scalex,
        scaley,
        preserve_aspect,
        aspect_offset,
    } = sd;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_text")
                .attr("alignment-baseline", "start")
                .attr("text-anchor", "middle")
                .attr("font-size", "x-large")
                .attr("x", width / 2.0)
                .attr("y", padding / 4.0);
        })
        .build(|w| {
            write!(w.writer(), "{}", plotter.title).unwrap();
        });

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_text")
                .attr("alignment-baseline", "start")
                .attr("text-anchor", "middle")
                .attr("font-size", "x-large")
                .attr("x", width / 2.0)
                .attr("y", height - padding / 8.);
        })
        .build(|w| {
            write!(w.writer(), "{}", plotter.xname).unwrap();
        });

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_text")
                .attr("alignment-baseline", "start")
                .attr("text-anchor", "middle")
                .attr("font-size", "x-large")
                .attr(
                    "transform",
                    format_args!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
                )
                .attr("x", padding / 4.0)
                .attr("y", height / 2.0);
        })
        .build(|w| {
            write!(w.writer(), "{}", plotter.yname).unwrap();
        });

    {
        //Draw step lines
        //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

        let ideal_num_xsteps = if preserve_aspect { 5 } else { 9 };

        let ideal_num_ysteps = 7;

        let texty_padding = paddingy * 0.3;
        let textx_padding = padding * 0.1;

        let xtick_info = PlotNumber::compute_ticks(ideal_num_xsteps, [minx, maxx]);
        let ytick_info = PlotNumber::compute_ticks(ideal_num_ysteps, [miny, maxy]);

        let xdash_size = PlotNumber::tick_size(20.0, &xtick_info, [minx, maxx], scalex);
        let ydash_size = PlotNumber::tick_size(20.0, &ytick_info, [miny, maxy], scaley);

        use tagger::PathCommand::*;

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(base) = xtick_info.display_relative {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "middle")
                            .attr("text-anchor", "start")
                            .attr("x", width * 0.55)
                            .attr("y", paddingy * 0.7);
                    })
                    .build(|d| {
                        d.put_raw(format_args!(
                            "Where j = {}",
                            DisplayableClosure::new(|w| base.fmt_tick(w, None))
                        ));
                    });

                "j+"
            } else {
                ""
            };

            //Draw interva`l x text
            for Tick { position, value } in xtick_info.ticks {
                let xx = (position.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex))
                    + padding;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")
                        .attr("stroke", "black")
                        .attr("x1", aspect_offset + xx)
                        .attr("x2", aspect_offset + xx)
                        .attr("y1", height - paddingy)
                        .attr("y2", height - paddingy * 0.95);
                });

                let s = xtick_info.step;
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "start")
                            .attr("text-anchor", "middle")
                            .attr("x", aspect_offset + xx)
                            .attr("y", height - paddingy + texty_padding);
                    })
                    .build(|w| {
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.xtick_fmt.write(w,value,Some(s)))
                        ));
                    });
            }
        }

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(base) = ytick_info.display_relative {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "middle")
                            .attr("text-anchor", "start")
                            .attr("x", padding)
                            .attr("y", paddingy * 0.7);
                    })
                    .build(|w| {
                        w.put_raw(format_args!(
                            "Where k = {}",
                            DisplayableClosure::new(|w| base.fmt_tick(w, None))
                        ));
                    });

                "k+"
            } else {
                ""
            };

            //Draw interval y text
            for Tick { position, value } in ytick_info.ticks {
                let yy = height
                    - (position.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley))
                    - paddingy;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")
                        .attr("stroke", "black")
                        .attr("x1", aspect_offset + padding)
                        .attr("x2", aspect_offset + padding * 0.96)
                        .attr("y1", yy)
                        .attr("y2", yy);
                });

                let s = ytick_info.step;

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "middle")
                            .attr("text-anchor", "end")
                            .attr("x", aspect_offset + padding - textx_padding)
                            .attr("y", yy);
                    })
                    .build(|w| {
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w|  plotter.ytick_fmt.write(w,value,Some(s)))
                        ));
                    });
            }
        }

        let d1 = minx.scale([minx, maxx], scalex);
        let d2 = xtick_info.start_step.scale([minx, maxx], scalex);
        let distance_to_firstx = d2 - d1;

        writer.single("path", |d| {
            d.attr("stroke", "black")
                .attr("fill", "none")
                .attr("class", "poloto_axis_lines");
            if let Some(xdash_size) = xdash_size {
                d.attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        xdash_size / 2.0,
                        //-(distance_to_firstx).scale2([minx,maxx],scalex)
                        -distance_to_firstx
                    ),
                );
            }
            d.path(|p| {
                p.put(M(padding + aspect_offset, height - paddingy));
                if preserve_aspect {
                    p.put(L(
                        height - paddingy / 2.0 + aspect_offset,
                        height - paddingy,
                    ));
                } else {
                    p.put(L(width - padding + aspect_offset, height - paddingy));
                }
            });
        });

        let d1 = miny.scale([miny, maxy], scaley);
        let d2 = ytick_info.start_step.scale([miny, maxy], scaley);
        let distance_to_firsty = d2 - d1;

        writer.single("path", |d| {
            d.attr("stroke", "black")
                .attr("fill", "none")
                .attr("class", "poloto_axis_lines");
            if let Some(ydash_size) = ydash_size {
                d.attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        ydash_size / 2.0,
                        //-(distance_to_firsty).scale2([miny,maxy],scaley)
                        -distance_to_firsty
                    ),
                );
            }
            d.path(|p| {
                p.put(M(aspect_offset + padding, height - paddingy));
                p.put(L(aspect_offset + padding, paddingy));
            });
        });
    }
}
