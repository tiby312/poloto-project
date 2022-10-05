use super::*;

pub(super) fn render_base<X: PlotNum, Y: PlotNum>(
    writer: &mut hypermelon::ElemWrite,
    xticksg: impl TickDist<Num = X>,
    yticksg: impl TickDist<Num = Y>,
    boundx: &ticks::DataBound<X>,
    boundy: &ticks::DataBound<Y>,
    plot_fmt: &mut dyn BaseFmt,
    canvas: &RenderOptionsResult,
) -> std::fmt::Result {
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

    let text = hbuild::elem("text").with(attrs!(
        ("class", "poloto_labels poloto_text poloto_title"),
        ("x", width / 2.0),
        ("y", padding / 4.0)
    ));

    let title = hbuild::from_closure(|w| plot_fmt.write_title(&mut w.writer()));

    writer.render(text.append(title))?;

    let text = hbuild::elem("text").with(attrs!(
        ("class", "poloto_labels poloto_text poloto_xname"),
        ("x", width / 2.0),
        ("y", height - padding / 8.)
    ));

    let xname = hbuild::from_closure(|w| plot_fmt.write_xname(&mut w.writer()));

    writer.render(text.append(xname))?;

    let text = hbuild::elem("text").with(attrs!(
        ("class", "poloto_labels poloto_text poloto_yname"),
        (
            "transform",
            format_move!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
        ),
        ("x", padding / 4.0),
        ("y", height / 2.0)
    ));

    let yname = hbuild::from_closure(|w| plot_fmt.write_yname(&mut w.writer()));

    writer.render(text.append(yname))?;

    let xdash_size = xticksg.res.dash_size;
    let ydash_size = yticksg.res.dash_size;

    let mut xticks = xticksg
        .iter
        .into_iter()
        .skip_while(|&x| x < boundx[0])
        .take_while(|&x| x <= boundx[1]);

    let xticks = {
        let a = xticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        let b = xticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        vec![a, b].into_iter().chain(xticks)
    };

    //TODO get rid of collecting ticks upfront.
    let mut xticks = xticks.collect::<Vec<_>>().into_iter();

    let mut yticks = yticksg
        .iter
        .into_iter()
        .skip_while(|&x| x < boundy[0])
        .take_while(|&x| x <= boundy[1]);

    let yticks = {
        let a = yticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        let b = yticks
            .next()
            .expect("There must be atleast two ticks for each axis");
        vec![a, b].into_iter().chain(yticks)
    };

    //TODO get rid of collecting ticks upfront.
    let mut yticks = yticks.collect::<Vec<_>>().into_iter();

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
        let text = hbuild::elem("text").with(attrs!(
            ("class", "poloto_tick_labels poloto_text"),
            ("dominant-baseline", "middle"),
            ("text-anchor", "start"),
            ("x", padding),
            ("y", paddingy * 0.7)
        ));

        let ywher = hbuild::from_closure(|w| yticksg.fmt.write_where(&mut w.writer()));

        writer.render(text.append(ywher))?;

        //Draw interval y text
        for val in std::iter::once(first_ticky).chain(yticks) {
            let yy = height
                - (val.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley))
                - paddingy;

            writer.render(hbuild::single("line").with(attrs!(
                ("class", "poloto_axis_lines"),
                ("stroke", "black"),
                ("x1", xaspect_offset + padding),
                ("x2", xaspect_offset + padding * 0.96),
                ("y1", yaspect_offset + yy),
                ("y2", yaspect_offset + yy)
            )))?;

            if canvas.ytick_lines {
                writer.render(hbuild::single("line").with(attrs!(
                    ("class", "poloto_tick_line"),
                    ("stroke", "black"),
                    ("x1", xaspect_offset + padding),
                    ("x2", padding + xaspect_offset + distancex_min_to_max),
                    ("y1", yaspect_offset + yy),
                    ("y2", yaspect_offset + yy)
                )))?;
            }

            let text = hbuild::elem("text").with(attrs!(
                ("class", "poloto_tick_labels poloto_text"),
                ("dominant-baseline", "middle"),
                ("text-anchor", "end"),
                ("x", xaspect_offset + padding - textx_padding),
                ("y", yaspect_offset + yy)
            ));

            let ytick = hbuild::from_closure(|w| yticksg.fmt.write_tick(&mut w.writer(), &val));

            writer.render(text.append(ytick))?;
        }
    }

    {
        let text = hbuild::elem("text").with(attrs!(
            ("class", "poloto_tick_labels poloto_text"),
            ("dominant-baseline", "middle"),
            ("text-anchor", "start"),
            ("x", width * 0.55),
            ("y", paddingy * 0.7)
        ));

        let xwher = hbuild::from_closure(|w| xticksg.fmt.write_where(&mut w.writer()));

        writer.render(text.append(xwher))?;

        //Draw interva`l x text
        for val in std::iter::once(first_tickx).chain(xticks) {
            let xx = (val.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex)) + padding;

            writer.render(hbuild::single("line").with(attrs!(
                ("class", "poloto_axis_lines"),
                ("stroke", "black"),
                ("x1", xaspect_offset + xx),
                ("x2", xaspect_offset + xx),
                ("y1", yaspect_offset + height - paddingy),
                ("y2", yaspect_offset + height - paddingy * 0.95)
            )))?;

            if canvas.xtick_lines {
                writer.render(hbuild::single("line").with(attrs!(
                    ("class", "poloto_tick_line"),
                    ("stroke", "black"),
                    ("x1", xaspect_offset + xx),
                    ("x2", xaspect_offset + xx),
                    ("y1", yaspect_offset + height - paddingy),
                    (
                        "y2",
                        yaspect_offset + height - paddingy - distancey_min_to_max,
                    )
                )))?;
            }

            let text = hbuild::elem("text").with(attrs!(
                ("class", "poloto_tick_labels poloto_text"),
                ("dominant-baseline", "start"),
                ("text-anchor", "middle"),
                ("x", xaspect_offset + xx),
                ("y", yaspect_offset + height - paddingy + texty_padding)
            ));

            let xtick = hbuild::from_closure(|w| xticksg.fmt.write_tick(&mut w.writer(), &val));

            writer.render(text.append(xtick))?;
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

    use hbuild::PathCommand::*;
    writer.render(hbuild::single("path").with(attrs!(
        ("stroke", "black"),
        ("fill", "none"),
        ("class", "poloto_axis_lines"),
        xclosure,
        hbuild::path([
            M(padding + xaspect_offset, height - paddingy + yaspect_offset,),
            L(
                padding + xaspect_offset + distancex_min_to_max,
                height - paddingy + yaspect_offset,
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
            M(xaspect_offset + padding, yaspect_offset + height - paddingy,),
            L(
                xaspect_offset + padding,
                yaspect_offset + height - paddingy - distancey_min_to_max,
            )
        ])
    )))?;

    Ok(())
}
