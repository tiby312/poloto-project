use super::*;

pub fn line_fill<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
    base_line: f64,
    add_start_end_base: bool,
) -> fmt::Result {
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
                        if add_start_end_base {
                            path.put(M(last[0], base_line))?;
                            path.put(L(last[0], last[1]))?;
                        } else {
                            path.put(M(last[0], last[1]))?;
                        }
                        first = false;
                    }
                    last_finite = Some([newx, newy]);
                    path.put(L(newx, newy))?;
                }
                (true, false) => {
                    path.put(M(newx, newy))?;
                }
                (false, true) => {
                    path.put(L(last[0], base_line))?;
                }
                _ => {}
            };
            last = [newx, newy];
        }
        if let Some([x, _]) = last_finite {
            if add_start_end_base {
                path.put(L(x, base_line))?;
            }
            path.put(Z(""))?;
        }
    }
    Ok(())
}

pub fn line<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
) -> fmt::Result {
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
                        path.put(M(last[0], last[1]))?;
                        first = false;
                    }
                    path.put(L(newx, newy))?;
                }
                (true, false) => {
                    path.put(M(newx, newy))?;
                }
                _ => {}
            };
            last = [newx, newy];
        }
    }
    Ok(())
}

///
/// Draw the axis lines, and tick intervals
///
pub(super) fn draw_base<X: PlotNum, Y: PlotNum, T: fmt::Write>(
    plotter: &mut Plotter<X, Y>,
    writer: &mut tagger::ElemWriter<T>,
    dd: DrawData,
    sd: ScaleData<X, Y>,
) -> fmt::Result {
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
            d.attr("class", "poloto_labels poloto_text poloto_title")?;
            d.attr("alignment-baseline", "start")?;
            d.attr("text-anchor", "middle")?;
            d.attr("font-size", "x-large")?;
            d.attr("x", width / 2.0)?;
            d.attr("y", padding / 4.0)
        })?
        .build(|w| w.put_raw(&plotter.title))?;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_labels poloto_text poloto_xname")?;
            d.attr("alignment-baseline", "start")?;
            d.attr("text-anchor", "middle")?;
            d.attr("font-size", "x-large")?;
            d.attr("x", width / 2.0)?;
            d.attr("y", height - padding / 8.)
        })?
        .build(|w| w.put_raw(&plotter.xname))?;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_labels poloto_text poloto_yname")?;
            d.attr("alignment-baseline", "start")?;
            d.attr("text-anchor", "middle")?;
            d.attr("font-size", "x-large")?;
            d.attr(
                "transform",
                format_args!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
            )?;
            d.attr("x", padding / 4.0)?;
            d.attr("y", height / 2.0)
        })?
        .build(|w| w.put_raw(&plotter.yname))?;

    {
        //Draw step lines
        //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

        //let ideal_num_xsteps = if preserve_aspect { 5 } else { 9 };
        let ideal_num_xsteps = if preserve_aspect { 4 } else { 6 };

        //let ideal_num_ysteps = 7;
        let ideal_num_ysteps = 5;

        let texty_padding = paddingy * 0.3;
        let textx_padding = padding * 0.1;

        let xtick_info = PlotNum::compute_ticks(
            ideal_num_xsteps,
            [minx, maxx],
            DashInfo {
                ideal_dash_size: 30.0,
                max: scalex,
            },
        );
        let ytick_info = PlotNum::compute_ticks(
            ideal_num_ysteps,
            [miny, maxy],
            DashInfo {
                ideal_dash_size: 30.0,
                max: scaley,
            },
        );

        let xdash_size = xtick_info.dash_size; //PlotNum::dash_size(30.0, &xtick_info, [minx, maxx], scalex);
        let ydash_size = ytick_info.dash_size; //PlotNum::dash_size(30.0, &ytick_info, [miny, maxy], scaley);

        use tagger::PathCommand::*;

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(base) = xtick_info.display_relative {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "middle")?;
                        d.attr("text-anchor", "start")?;
                        d.attr("x", width * 0.55)?;
                        d.attr("y", paddingy * 0.7)
                    })?
                    .build(|d| {
                        d.put_raw(format_args!(
                            "Where j = {}",
                            DisplayableClosure::new(|w| base.fmt_tick(
                                w,
                                xtick_info.unit_data,
                                FmtFull::Full
                            ))
                        ))
                    })?;

                "j+"
            } else {
                ""
            };

            //Draw interva`l x text
            for &Tick { position, value } in xtick_info.ticks.iter() {
                let xx = (position.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex))
                    + padding;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", aspect_offset + xx)?;
                    d.attr("x2", aspect_offset + xx)?;
                    d.attr("y1", height - paddingy)?;
                    d.attr("y2", height - paddingy * 0.95)
                })?;

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "start")?;
                        d.attr("text-anchor", "middle")?;
                        d.attr("x", aspect_offset + xx)?;
                        d.attr("y", height - paddingy + texty_padding)
                    })?
                    .build(|w| {
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.xtick_fmt.write(
                                w,
                                value,
                                xtick_info.unit_data,
                                FmtFull::Tick
                            ))
                        ))
                    })?;
            }
        }

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(base) = ytick_info.display_relative {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "middle")?;
                        d.attr("text-anchor", "start")?;
                        d.attr("x", padding)?;
                        d.attr("y", paddingy * 0.7)
                    })?
                    .build(|w| {
                        w.put_raw(format_args!(
                            "Where k = {}",
                            DisplayableClosure::new(|w| base.fmt_tick(
                                w,
                                ytick_info.unit_data,
                                FmtFull::Full
                            ))
                        ))
                    })?;

                "k+"
            } else {
                ""
            };

            //Draw interval y text
            for &Tick { position, value } in ytick_info.ticks.iter() {
                let yy = height
                    - (position.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley))
                    - paddingy;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", aspect_offset + padding)?;
                    d.attr("x2", aspect_offset + padding * 0.96)?;
                    d.attr("y1", yy)?;
                    d.attr("y2", yy)
                })?;

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "middle")?;
                        d.attr("text-anchor", "end")?;
                        d.attr("x", aspect_offset + padding - textx_padding)?;
                        d.attr("y", yy)
                    })?
                    .build(|w| {
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.ytick_fmt.write(
                                w,
                                value,
                                ytick_info.unit_data,
                                FmtFull::Tick
                            )) //TODO need a way to communicate writing base
                        ))
                    })?;
            }
        }

        let d1 = minx.scale([minx, maxx], scalex);
        let d2 = xtick_info.ticks[0].position.scale([minx, maxx], scalex);
        let distance_to_firstx = d2 - d1;

        writer.single("path", |d| {
            d.attr("stroke", "black")?;
            d.attr("fill", "none")?;
            d.attr("class", "poloto_axis_lines")?;
            if let Some(xdash_size) = xdash_size {
                d.attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        xdash_size / 2.0,
                        -distance_to_firstx
                    ),
                )?;
            }
            d.path(|p| {
                p.put(M(padding + aspect_offset, height - paddingy))?;
                if preserve_aspect {
                    p.put(L(
                        height - paddingy / 2.0 + aspect_offset,
                        height - paddingy,
                    ))
                } else {
                    p.put(L(width - padding + aspect_offset, height - paddingy))
                }
            })
        })?;

        let d1 = miny.scale([miny, maxy], scaley);
        let d2 = ytick_info.ticks[0].position.scale([miny, maxy], scaley);
        let distance_to_firsty = d2 - d1;

        writer.single("path", |d| {
            d.attr("stroke", "black")?;
            d.attr("fill", "none")?;
            d.attr("class", "poloto_axis_lines")?;
            if let Some(ydash_size) = ydash_size {
                d.attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        ydash_size / 2.0,
                        -distance_to_firsty
                    ),
                )?;
            }
            d.path(|p| {
                p.put(M(aspect_offset + padding, height - paddingy))?;
                p.put(L(aspect_offset + padding, paddingy))
            })
        })?;
    }
    Ok(())
}
