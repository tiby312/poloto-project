use super::*;

pub(super) fn render_base<X: PlotNum, Y: PlotNum>(
    writer: &mut elem::ElemWrite,
    xticksg: impl TickDist<Num = X>,
    yticksg: impl TickDist<Num = Y>,
    boundx: &ticks::DataBound<X>,
    boundy: &ticks::DataBound<Y>,
    plot_fmt: &mut dyn BaseFmt,
    canvas: &RenderOptionsResult,
) -> std::fmt::Result {
    let ffmt = FloatFmt::new(canvas.precision);

    use crate::ticks::tick_fmt::TickFmt;

    let mut xticksg = xticksg.unwrap();
    let mut yticksg = yticksg.unwrap();

    let RenderOptionsResult {
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

    let g = hbuild::elem("g").with(("class", "poloto_labels poloto_text"));

    writer.session(g).build(|w| {
        let text = hbuild::elem("text")
            .with(attrs!(
                ("class", "poloto_title"),
                ("x", ffmt.disp(width / 2.0)),
                ("y", ffmt.disp(padding / 4.0))
            ))
            .inline();

        let title = hbuild::from_closure(|w| plot_fmt.write_title(&mut w.writer()));
        let title = text.append(title);
        w.render(title)?;

        let text = hbuild::elem("text")
            .with(attrs!(
                ("class", "poloto_xname"),
                ("x", ffmt.disp(width / 2.0)),
                ("y", ffmt.disp(height - padding / 8.))
            ))
            .inline();

        let xname = hbuild::from_closure(|w| plot_fmt.write_xname(&mut w.writer()));
        let xname = text.append(xname);
        w.render(xname)?;

        let text = hbuild::elem("text")
            .with(attrs!(
                ("class", "poloto_yname"),
                (
                    "transform",
                    format_move!(
                        "rotate(-90,{},{})",
                        ffmt.disp(padding / 4.0),
                        ffmt.disp(height / 2.0)
                    ),
                ),
                ("x", ffmt.disp(padding / 4.0)),
                ("y", ffmt.disp(height / 2.0))
            ))
            .inline();

        let yname = hbuild::from_closure(|w| plot_fmt.write_yname(&mut w.writer()));
        let yname = text.append(yname);
        w.render(yname)?;
        Ok(())
    })?;

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
        let d1 = minx.scale([minx, maxx], scalex);
        let d2 = first_tickx.scale([minx, maxx], scalex);
        let distance_to_firstx = d2 - d1;
        let distancex_min_to_max = maxx.scale([minx, maxx], scalex) - d1;
        (distance_to_firstx, distancex_min_to_max)
    };

    let (distance_to_firsty, distancey_min_to_max) = {
        let d1 = miny.scale([miny, maxy], scaley);
        let d2 = first_ticky.scale([miny, maxy], scaley);
        let distance_to_firsty = d2 - d1;
        let distancey_min_to_max = maxy.scale([miny, maxy], scaley) - d1;
        (distance_to_firsty, distancey_min_to_max)
    };

    {
        let mut ywher = String::new();
        yticksg.fmt.write_where(&mut ywher)?;

        if !ywher.is_empty() {
            let text = hbuild::elem("text")
                .with(attrs!(
                    ("class", "poloto_tick_labels poloto_text"),
                    ("dominant-baseline", "middle"),
                    ("text-anchor", "start"),
                    ("x", ffmt.disp(*padding)),
                    ("y", ffmt.disp(paddingy * 0.7))
                ))
                .inline();

            writer.render(text.append(ywher))?;
        }

        let ticks: Vec<_> = std::iter::once(first_ticky)
            .chain(yticks)
            .map(|val| {
                let yy = height
                    - (val.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley))
                    - paddingy;
                (val, yy)
            })
            .collect();

        let j = hbuild::from_closure(|w| {
            for (val, yy) in ticks.iter() {
                let text = hbuild::elem("text")
                    .with(attrs!(
                        ("x", ffmt.disp(xaspect_offset + padding - textx_padding)),
                        ("y", ffmt.disp(yaspect_offset + yy))
                    ))
                    .inline();

                let ytick = hbuild::from_closure(|w| yticksg.fmt.write_tick(&mut w.writer(), val));

                w.render(text.append(ytick))?;
            }
            Ok(())
        });

        let g = hbuild::elem("g").with(attrs!(
            ("class", "poloto_tick_labels poloto_text"),
            ("dominant-baseline", "middle"),
            ("text-anchor", "end")
        ));

        writer.render(g.append(j))?;

        let j = hbuild::from_closure(|w| {
            for (_, yy) in ticks.iter() {
                w.render(hbuild::single("line").with(attrs!(
                    ("x1", ffmt.disp(xaspect_offset + padding)),
                    ("x2", ffmt.disp(xaspect_offset + padding * 0.96)),
                    ("y1", ffmt.disp(yaspect_offset + yy)),
                    ("y2", ffmt.disp(yaspect_offset + yy))
                )))?;
            }
            Ok(())
        });

        let g = hbuild::elem("g").with(attrs!(("class", "poloto_axis_lines"), ("stroke", "black")));

        writer.render(g.append(j))?;

        //Draw interval y text
        for (_, yy) in ticks.iter() {
            if canvas.ytick_lines {
                writer.render(hbuild::single("line").with(attrs!(
                    ("class", "poloto_tick_line"),
                    ("stroke", "black"),
                    ("x1", ffmt.disp(xaspect_offset + padding)),
                    (
                        "x2",
                        ffmt.disp(padding + xaspect_offset + distancex_min_to_max)
                    ),
                    ("y1", ffmt.disp(yaspect_offset + yy)),
                    ("y2", ffmt.disp(yaspect_offset + yy))
                )))?;
            }
        }
    }

    {
        let mut xwher = String::new();
        xticksg.fmt.write_where(&mut xwher)?;

        if !xwher.is_empty() {
            let text = hbuild::elem("text")
                .with(attrs!(
                    ("class", "poloto_tick_labels poloto_text"),
                    ("dominant-baseline", "middle"),
                    ("text-anchor", "start"),
                    ("x", ffmt.disp(width * 0.55)),
                    ("y", ffmt.disp(paddingy * 0.7))
                ))
                .inline();

            writer.render(text.append(xwher))?;
        }

        let ticks: Vec<_> = std::iter::once(first_tickx)
            .chain(xticks)
            .map(|val| {
                let xx =
                    (val.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex)) + padding;
                (val, xx)
            })
            .collect();

        let j = hbuild::from_closure(|w| {
            for (val, xx) in ticks.iter() {
                let text = hbuild::elem("text")
                    .with(attrs!(
                        ("x", ffmt.disp(xaspect_offset + xx)),
                        (
                            "y",
                            ffmt.disp(yaspect_offset + height - paddingy + texty_padding)
                        )
                    ))
                    .inline();

                let xtick = hbuild::from_closure(|w| xticksg.fmt.write_tick(&mut w.writer(), val));
                w.render(text.append(xtick))?;
            }
            Ok(())
        });

        let g = hbuild::elem("g").with(attrs!(
            ("class", "poloto_tick_labels poloto_text"),
            ("dominant-baseline", "start"),
            ("text-anchor", "middle")
        ));
        writer.render(g.append(j))?;

        let j = hbuild::from_closure(|w| {
            for (_, xx) in ticks.iter() {
                w.render(hbuild::single("line").with(attrs!(
                    ("x1", ffmt.disp(xaspect_offset + xx)),
                    ("x2", ffmt.disp(xaspect_offset + xx)),
                    ("y1", ffmt.disp(yaspect_offset + height - paddingy)),
                    ("y2", ffmt.disp(yaspect_offset + height - paddingy * 0.95))
                )))?;
            }
            Ok(())
        });

        let g = hbuild::elem("g").with(attrs!(("class", "poloto_axis_lines"), ("stroke", "black")));

        writer.render(g.append(j))?;

        //Draw interva`l x text
        for (_, xx) in ticks.iter() {
            if canvas.xtick_lines {
                writer.render(hbuild::single("line").with(attrs!(
                    ("class", "poloto_tick_line"),
                    ("stroke", "black"),
                    ("x1", ffmt.disp(xaspect_offset + xx)),
                    ("x2", ffmt.disp(xaspect_offset + xx)),
                    ("y1", ffmt.disp(yaspect_offset + height - paddingy)),
                    (
                        "y2",
                        ffmt.disp(yaspect_offset + height - paddingy - distancey_min_to_max),
                    )
                )))?;
            }
        }
    }

    let xclosure = hbuild::attr_from_closure(|w| {
        if let Some(xdash_size) = xdash_size {
            w.render((
                "style",
                format_move!(
                    "stroke-dasharray:{};stroke-dashoffset:{};",
                    xdash_size / 2.0,
                    -distance_to_firstx
                ),
            ))?;
        }
        Ok(())
    });

    use attr::PathCommand::*;
    writer.render(hbuild::single("path").with(attrs!(
        ("stroke", "black"),
        ("fill", "none"),
        ("class", "poloto_axis_lines"),
        xclosure,
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
    )))?;

    let yclosure = hbuild::attr_from_closure(|w| {
        if let Some(ydash_size) = ydash_size {
            w.render((
                "style",
                format_move!(
                    "stroke-dasharray:{};stroke-dashoffset:{};",
                    ydash_size / 2.0,
                    -distance_to_firsty
                ),
            ))?;
        }
        Ok(())
    });
    writer.render(hbuild::single("path").with(attrs!(
        ("stroke", "black"),
        ("fill", "none"),
        ("class", "poloto_axis_lines"),
        yclosure,
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
    )))?;

    Ok(())
}
