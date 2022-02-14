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
        aspect_offset,
        scalex,
        scaley,
        preserve_aspect,
        ..
    } = plotter.plots.canvas;

    let boundx = [plotter.plots.boundx.min, plotter.plots.boundx.max];
    let boundy = [plotter.plots.boundy.min, plotter.plots.boundy.max];
    let [minx, maxx] = boundx;
    let [miny, maxy] = boundy;

    let xtick_info = &mut plotter.tickx;
    let ytick_info = &mut plotter.ticky;

    let xcontext = &mut *plotter.xcontext;
    let ycontext = &mut *plotter.ycontext;

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
        .build(|w| {
            use std::fmt::Write;
            write!(w.writer_safe(), "{}", plotter.title)
        })?;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_labels poloto_text poloto_xname")?;
            d.attr("alignment-baseline", "start")?;
            d.attr("text-anchor", "middle")?;
            d.attr("font-size", "x-large")?;
            d.attr("x", width / 2.0)?;
            d.attr("y", height - padding / 8.)
        })?
        .build(|w| {
            use std::fmt::Write;
            write!(w.writer_safe(), "{}", plotter.xname)
        })?;

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
        .build(|w| {
            use std::fmt::Write;
            write!(w.writer_safe(), "{}", plotter.yname)
        })?;

    let xdash_size = xtick_info.dash_size;
    let ydash_size = ytick_info.dash_size;

    use tagger::PathCommand::*;

    let first_tickx = xtick_info.ticks[0];

    let first_ticky = ytick_info.ticks[0];

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
                    let mut w = d.writer_safe();
                    use std::fmt::Write;
                    write!(w, "Where j = ")?;

                    xcontext.write_where(&mut w, &base)
                })?;

            "j+"
        } else {
            ""
        };

        //Draw interva`l x text
        for &Tick { position, value } in xtick_info.ticks.iter() {
            let xx =
                (position.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex)) + padding;

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
                    let mut w = w.writer_safe();
                    use std::fmt::Write;
                    write!(w, "{}", extra)?;

                    xcontext.write_tick(&mut w, &value)
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
                    use std::fmt::Write;
                    let mut w = w.writer_safe();
                    write!(w, "Where k = ")?;

                    ycontext.write_where(&mut w, &base)
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
                    let mut w = w.writer_safe();
                    use std::fmt::Write;
                    write!(w, "{}", extra)?;

                    ycontext.write_tick(&mut w, &value)
                })?;
        }
    }

    let d1 = minx.scale([minx, maxx], scalex);
    let d2 = first_tickx.position.scale([minx, maxx], scalex);
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
    let d2 = first_ticky.position.scale([miny, maxy], scaley);
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

    Ok(())
}
