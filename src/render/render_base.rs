use super::*;

pub fn render_base<X: PlotNum, Y: PlotNum>(
    writer: impl std::fmt::Write,
    plotter: &mut Plotter<X, Y>,
) -> std::fmt::Result {
    let mut writer = tagger::new(writer);

    let Canvas {
        width,
        height,
        padding,
        paddingy,
        xaspect_offset,
        yaspect_offset,
        scalex,
        scaley,
        ..
    } = plotter.plots.canvas;

    let boundx = [plotter.plots.boundx.min, plotter.plots.boundx.max];
    let boundy = [plotter.plots.boundy.min, plotter.plots.boundy.max];

    let [minx, maxx] = boundx;
    let [miny, maxy] = boundy;

    let xtick_info = &mut plotter.tickx;
    let ytick_info = &mut plotter.ticky;

    let texty_padding = paddingy * 0.3;
    let textx_padding = padding * 0.1;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_labels poloto_text poloto_title")?;
            d.attr("alignment-baseline", "start")?;
            d.attr("text-anchor", "middle")?;
            d.attr("font-size", "x-large")?;
            d.attr("x", width / 2.0)?;
            d.attr("y", padding / 4.0)
        })?
        .build(|w| plotter.plot_fmt.write_title(&mut w.writer_safe()))?;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_labels poloto_text poloto_xname")?;
            d.attr("alignment-baseline", "start")?;
            d.attr("text-anchor", "middle")?;
            d.attr("font-size", "x-large")?;
            d.attr("x", width / 2.0)?;
            d.attr("y", height - padding / 8.)
        })?
        .build(|w| plotter.plot_fmt.write_xname(&mut w.writer_safe()))?;

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
        .build(|w| plotter.plot_fmt.write_yname(&mut w.writer_safe()))?;

    let xdash_size = xtick_info.dash_size;
    let ydash_size = ytick_info.dash_size;

    use tagger::PathCommand::*;

    let first_tickx = xtick_info.ticks[0];

    let first_ticky = ytick_info.ticks[0];

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
        //step num is assured to be atleast 1.
        writer
            .elem("text", |d| {
                d.attr("class", "poloto_tick_labels poloto_text")?;
                d.attr("alignment-baseline", "middle")?;
                d.attr("text-anchor", "start")?;
                d.attr("x", width * 0.55)?;
                d.attr("y", paddingy * 0.7)
            })?
            .build(|d| plotter.plot_fmt.write_xwher(&mut d.writer_safe()))?;

        //Draw interva`l x text
        for &val in xtick_info.ticks.iter() {
            let xx = (val.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex)) + padding;

            writer.single("line", |d| {
                d.attr("class", "poloto_axis_lines")?;
                d.attr("stroke", "black")?;
                d.attr("x1", xaspect_offset + xx)?;
                d.attr("x2", xaspect_offset + xx)?;
                d.attr("y1", yaspect_offset + height - paddingy)?;
                d.attr("y2", yaspect_offset + height - paddingy * 0.95)
            })?;

            if plotter.plots.xtick_lines {
                writer.single("line", |d| {
                    d.attr("class", "poloto_tick_line")?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", xaspect_offset + xx)?;
                    d.attr("x2", xaspect_offset + xx)?;
                    d.attr("y1", yaspect_offset + height - paddingy)?;
                    d.attr(
                        "y2",
                        yaspect_offset + height - paddingy - distancey_min_to_max,
                    )
                })?;
            }

            writer
                .elem("text", |d| {
                    d.attr("class", "poloto_tick_labels poloto_text")?;
                    d.attr("alignment-baseline", "start")?;
                    d.attr("text-anchor", "middle")?;
                    d.attr("x", xaspect_offset + xx)?;
                    d.attr("y", yaspect_offset + height - paddingy + texty_padding)
                })?
                .build(|w| plotter.plot_fmt.write_xtick(&mut w.writer_safe(), &val))?;
        }
    }

    {
        //step num is assured to be atleast 1.
        writer
            .elem("text", |d| {
                d.attr("class", "poloto_tick_labels poloto_text")?;
                d.attr("alignment-baseline", "middle")?;
                d.attr("text-anchor", "start")?;
                d.attr("x", padding)?;
                d.attr("y", paddingy * 0.7)
            })?
            .build(|w| plotter.plot_fmt.write_ywher(&mut w.writer_safe()))?;

        //Draw interval y text
        for &val in ytick_info.ticks.iter() {
            let yy = height
                - (val.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley))
                - paddingy;

            writer.single("line", |d| {
                d.attr("class", "poloto_axis_lines")?;
                d.attr("stroke", "black")?;
                d.attr("x1", xaspect_offset + padding)?;
                d.attr("x2", xaspect_offset + padding * 0.96)?;
                d.attr("y1", yaspect_offset + yy)?;
                d.attr("y2", yaspect_offset + yy)
            })?;

            if plotter.plots.ytick_lines {
                writer.single("line", |d| {
                    d.attr("class", "poloto_tick_line")?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", xaspect_offset + padding)?;
                    d.attr("x2", padding + xaspect_offset + distancex_min_to_max)?;
                    d.attr("y1", yaspect_offset + yy)?;
                    d.attr("y2", yaspect_offset + yy)
                })?;
            }

            writer
                .elem("text", |d| {
                    d.attr("class", "poloto_tick_labels poloto_text")?;
                    d.attr("alignment-baseline", "middle")?;
                    d.attr("text-anchor", "end")?;
                    d.attr("x", xaspect_offset + padding - textx_padding)?;
                    d.attr("y", yaspect_offset + yy)
                })?
                .build(|w| plotter.plot_fmt.write_ytick(&mut w.writer_safe(), &val))?;
        }
    }

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
            p.put(M(
                padding + xaspect_offset,
                height - paddingy + yaspect_offset,
            ))?;
            p.put(L(
                padding + xaspect_offset + distancex_min_to_max,
                height - paddingy + yaspect_offset,
            ))
        })
    })?;

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
            p.put(M(
                xaspect_offset + padding,
                yaspect_offset + height - paddingy,
            ))?;
            p.put(L(
                xaspect_offset + padding,
                yaspect_offset + height - paddingy - distancey_min_to_max,
            ))
        })
    })?;

    Ok(())
}
