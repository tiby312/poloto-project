use super::*;

pub(super) fn render_base<'b, 'a, X: PlotNum + 'a, Y: PlotNum + 'a>(
    mut writer: ElemStack<'b, Sentinel>,
    xticksg: impl TickDist<Num = X> + 'a,
    yticksg: impl TickDist<Num = Y> + 'a,
    boundx: &'a ticks::DataBound<X>,
    boundy: &'a ticks::DataBound<Y>,
    plot_fmt: &'a mut dyn BaseFmt,
    canvas: &'a RenderFrame,
) -> Result<ElemStack<'b, Sentinel>, fmt::Error> {
    let ffmt = FloatFmt::new(canvas.precision);

    use crate::ticks::tick_fmt::TickFmt;

    let xticksg = xticksg.unwrap();
    let yticksg = yticksg.unwrap();

    let RenderFrame {
        width,
        height,
        padding,
        paddingy,
        xaspect_offset,
        yaspect_offset,
        ..
    } = canvas;

    let scalex = canvas.boundx.max;
    let scaley = canvas.boundy.max;

    let boundx = [boundx.min, boundx.max];
    let boundy = [boundy.min, boundy.max];

    let [minx, maxx] = boundx;
    let [miny, maxy] = boundy;

    let texty_padding = paddingy * 0.3;
    let textx_padding = padding * 0.1;

    let title = {
        let text = hbuild::elem("text")
            .with(attrs!(
                ("class", "poloto_text poloto_name poloto_title"),
                ("x", ffmt.disp(width / 2.0)),
                ("y", ffmt.disp(padding / 4.0))
            ))
            .inline();

        let title = hbuild::from_stack(|mut w| plot_fmt.write_title(&mut w.writer()).map(|_| w));
        text.append(title)
    };

    let xname = {
        let text = hbuild::elem("text")
            .with(attrs!(
                ("class", "poloto_text poloto_name poloto_x"),
                ("x", ffmt.disp(width / 2.0)),
                ("y", ffmt.disp(height - padding / 8.))
            ))
            .inline();

        let xname = hbuild::from_stack(|mut w| plot_fmt.write_xname(&mut w.writer()).map(|_| w));
        text.append(xname)
    };

    let yname = {
        let text = hbuild::elem("text")
            .with(attrs!(
                ("class", "poloto_text poloto_name poloto_y"),
                ("x", ffmt.disp(padding / 4.0)),
                ("y", ffmt.disp(height / 2.0)),
                (
                    "transform",
                    format_move!(
                        "rotate(-90,{},{})",
                        ffmt.disp(padding / 4.0),
                        ffmt.disp(height / 2.0)
                    ),
                )
            ))
            .inline();

        let yname = hbuild::from_stack(|mut w| plot_fmt.write_yname(&mut w.writer()).map(|_| w));
        text.append(yname)
    };

    let title_xname_yname = title.chain(xname).chain(yname);

    //writer.render(g)?;

    let ywher = {
        let mut ywher = String::new();
        yticksg.fmt.write_where(&mut ywher)?;

        (!ywher.is_empty()).then(|| {
            let text = hbuild::elem("text")
                .with(attrs!(
                    ("class", "poloto_text poloto_where poloto_y"),
                    ("x", ffmt.disp(*padding)),
                    ("y", ffmt.disp(paddingy * 0.7))
                ))
                .inline();

            text.append(hbuild::raw(ywher))
        })
    };

    let xwher = {
        let mut xwher = String::new();
        xticksg.fmt.write_where(&mut xwher)?;

        (!xwher.is_empty()).then(|| {
            let text = hbuild::elem("text")
                .with(attrs!(
                    ("class", "poloto_text poloto_where poloto_x"),
                    ("x", ffmt.disp(width * 0.55)),
                    ("y", ffmt.disp(paddingy * 0.7))
                ))
                .inline();

            text.append(hbuild::raw(xwher))
        })
    };

    let xdash_size = xticksg.res.dash_size;
    let ydash_size = yticksg.res.dash_size;

    let mut xticks = xticksg
        .iter
        .into_iter()
        .skip_while(|&x| x < boundx[0])
        .take_while(|&x| x <= boundx[1]);

    let mut xticks = {
        let a = xticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        let b = xticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        [a, b].into_iter().chain(xticks)
    };

    let mut yticks = yticksg
        .iter
        .into_iter()
        .skip_while(|&x| x < boundy[0])
        .take_while(|&x| x <= boundy[1]);

    let mut yticks = {
        let a = yticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        let b = yticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        [a, b].into_iter().chain(yticks)
    };

    let first_tickx = xticks.next().unwrap();

    let first_ticky = yticks.next().unwrap();

    let (distance_to_firstx, distancex_min_to_max) = {
        let d1 = minx.scale(&[minx, maxx], scalex);
        let d2 = first_tickx.scale(&[minx, maxx], scalex);
        let distance_to_firstx = d2 - d1;
        let distancex_min_to_max = maxx.scale(&[minx, maxx], scalex) - d1;
        (distance_to_firstx, distancex_min_to_max)
    };

    let (distance_to_firsty, distancey_min_to_max) = {
        let d1 = miny.scale(&[miny, maxy], scaley);
        let d2 = first_ticky.scale(&[miny, maxy], scaley);
        let distance_to_firsty = d2 - d1;
        let distancey_min_to_max = maxy.scale(&[miny, maxy], scaley) - d1;
        (distance_to_firsty, distancey_min_to_max)
    };

    let ticks: Vec<_> = std::iter::once(first_ticky)
        .chain(yticks)
        .map(|val| {
            let yy = height
                - (val.scale(&[miny, maxy], scaley) - miny.scale(&[miny, maxy], scaley))
                - paddingy;
            (val, yy)
        })
        .collect();

    let ytick_elems = {
        let tick_text = {
            let g =
                hbuild::elem("text").with(attrs!(("class", "poloto_text poloto_ticks poloto_y")));

            let j = hbuild::from_iter(ticks.iter().map(|(val, yy)| {
                let text = hbuild::elem("tspan")
                    .with(attrs!(
                        ("x", ffmt.disp(xaspect_offset + padding - textx_padding)),
                        ("y", ffmt.disp(yaspect_offset + yy))
                    ))
                    .inline();

                let ytick = hbuild::from_stack(|mut w| {
                    yticksg.fmt.write_tick(&mut w.writer(), val).map(|_| w)
                });
                text.append(ytick)
            }));
            g.append(j)
        };

        let tick_short_lines = {
            let g = hbuild::elem("g").with(attrs!(
                ("class", "poloto_imgs poloto_ticks poloto_y"),
                ("stroke", "black")
            ));

            let j = hbuild::from_iter(ticks.iter().map(|(_, yy)| {
                hbuild::single("line").with(attrs!(
                    ("x1", ffmt.disp(xaspect_offset + padding)),
                    ("x2", ffmt.disp(xaspect_offset + padding * 0.96)),
                    ("y1", ffmt.disp(yaspect_offset + yy)),
                    ("y2", ffmt.disp(yaspect_offset + yy))
                ))
            }));
            g.append(j)
        };

        let tick_long_lines = {
            canvas.ytick_lines.then(|| {
                let g = hbuild::elem("g").with(attrs!(
                    ("class", "poloto_grid poloto_y"),
                    ("stroke", "black")
                ));

                let j = hbuild::from_iter(ticks.iter().map(|(_, yy)| {
                    hbuild::single("line").with(attrs!(
                        ("x1", ffmt.disp(xaspect_offset + padding)),
                        (
                            "x2",
                            ffmt.disp(padding + xaspect_offset + distancex_min_to_max)
                        ),
                        ("y1", ffmt.disp(yaspect_offset + yy)),
                        ("y2", ffmt.disp(yaspect_offset + yy))
                    ))
                }));

                g.append(j)
            })
        };

        tick_text.chain(tick_short_lines).chain(tick_long_lines)
    };

    let ticks: Vec<_> = std::iter::once(first_tickx)
        .chain(xticks)
        .map(|val| {
            let xx =
                (val.scale(&[minx, maxx], scalex) - minx.scale(&[minx, maxx], scalex)) + padding;
            (val, xx)
        })
        .collect();

    let xtick_elems = {
        let tick_text = {
            let g =
                hbuild::elem("text").with(attrs!(("class", "poloto_text poloto_ticks poloto_x")));

            let j = hbuild::from_iter(ticks.iter().map(|(val, xx)| {
                let text = hbuild::elem("tspan")
                    .with(attrs!(
                        ("x", ffmt.disp(xaspect_offset + xx)),
                        (
                            "y",
                            ffmt.disp(yaspect_offset + height - paddingy + texty_padding)
                        )
                    ))
                    .inline();

                let xtick = hbuild::from_stack(|mut w| {
                    xticksg.fmt.write_tick(&mut w.writer(), val).map(|_| w)
                });
                text.append(xtick)
            }));

            g.append(j)
        };

        let tick_short_lines = {
            let g = hbuild::elem("g").with(attrs!(
                ("class", "poloto_imgs poloto_ticks poloto_x"),
                ("stroke", "black")
            ));

            let j = hbuild::from_iter(ticks.iter().map(|(_, xx)| {
                hbuild::single("line").with(attrs!(
                    ("x1", ffmt.disp(xaspect_offset + xx)),
                    ("x2", ffmt.disp(xaspect_offset + xx)),
                    ("y1", ffmt.disp(yaspect_offset + height - paddingy)),
                    ("y2", ffmt.disp(yaspect_offset + height - paddingy * 0.95))
                ))
            }));

            g.append(j)
        };

        let tick_long_lines = {
            canvas.xtick_lines.then(|| {
                let g = hbuild::elem("g").with(attrs!(
                    ("class", "poloto_grid poloto_x"),
                    ("stroke", "black")
                ));

                let j = hbuild::from_iter(ticks.iter().map(|(_, xx)| {
                    hbuild::single("line").with(attrs!(
                        ("x1", ffmt.disp(xaspect_offset + xx)),
                        ("x2", ffmt.disp(xaspect_offset + xx)),
                        ("y1", ffmt.disp(yaspect_offset + height - paddingy)),
                        (
                            "y2",
                            ffmt.disp(yaspect_offset + height - paddingy - distancey_min_to_max),
                        )
                    ))
                }));

                g.append(j)
            })
        };
        tick_text.chain(tick_short_lines).chain(tick_long_lines)
    };

    use attr::PathCommand::*;

    let xline = hbuild::single("path").with(attrs!(
        ("class", "poloto_imgs poloto_ticks poloto_x"),
        ("stroke", "black"),
        xdash_size.map(|xdash_size| {
            (
                "style",
                format_move!(
                    "stroke-dasharray:{};stroke-dashoffset:{};",
                    xdash_size / 2.0,
                    -distance_to_firstx
                ),
            )
        }),
        hbuild::path([
            M(
                ffmt.disp(padding + xaspect_offset),
                ffmt.disp(height - paddingy + yaspect_offset)
            ),
            L(
                ffmt.disp(padding + xaspect_offset + distancex_min_to_max),
                ffmt.disp(height - paddingy + yaspect_offset),
            )
        ])
    ));

    let yline = hbuild::single("path").with(attrs!(
        ("class", "poloto_imgs poloto_ticks poloto_y"),
        ("stroke", "black"),
        ydash_size.map(|ydash_size| {
            (
                "style",
                format_move!(
                    "stroke-dasharray:{};stroke-dashoffset:{};",
                    ydash_size / 2.0,
                    -distance_to_firsty
                ),
            )
        }),
        hbuild::path([
            M(
                ffmt.disp(xaspect_offset + padding),
                ffmt.disp(yaspect_offset + height - paddingy)
            ),
            L(
                ffmt.disp(xaspect_offset + padding),
                ffmt.disp(yaspect_offset + height - paddingy - distancey_min_to_max),
            )
        ])
    ));

    //TODO replace with a element chaining macro?
    writer.put(
        title_xname_yname
            .chain(ywher)
            .chain(xwher)
            .chain(ytick_elems)
            .chain(xtick_elems)
            .chain(xline)
            .chain(yline),
    )?;
    Ok(writer)
}
