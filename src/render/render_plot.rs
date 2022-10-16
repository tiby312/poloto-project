use super::*;

use crate::build::*;

pub(super) fn render_plot<P: build::PlotIterator>(
    writer: &mut elem::ElemWrite,
    boundx: &ticks::DataBound<P::X>,
    boundy: &ticks::DataBound<P::Y>,
    canvas: &RenderOptionsResult,
    plots_all: &mut P,
) -> std::fmt::Result {
    let RenderOptionsResult {
        width,
        height,
        padding,
        paddingy,
        xaspect_offset,
        yaspect_offset,
        spacing,
        num_css_classes,
        ..
    } = canvas;

    let scalex = canvas.boundx.max;
    let scaley = canvas.boundy.max;

    let boundx = [boundx.min, boundx.max];
    let boundy = [boundy.min, boundy.max];

    let [minx, maxx] = boundx;
    let [miny, maxy] = boundy;

    let mut color_iter = {
        let max = if let Some(nn) = *num_css_classes {
            nn
        } else {
            usize::MAX
        };

        (0..max).cycle()
    };

    let mut color_iter2 = color_iter.clone();

    let mut f = crate::build::RenderablePlotIter::new(plots_all);

    let mut names = vec![];

    for i in 0.. {
        let mut ppp = if let Some(ppp) = f.next_plot() {
            ppp
        } else {
            break;
        };

        let typ = ppp.typ();

        let mut name = String::new();
        ppp.name(&mut name).unwrap()?;

        let name_exists = !name.is_empty();

        if name_exists {
            names.push((typ, name, i));
        }

        let aa = minx.scale([minx, maxx], scalex);
        let bb = miny.scale([miny, maxy], scaley);

        match typ {
            PlotMetaType::Text => {
                assert_eq!(ppp.plots().count(), 0);

                // don't need to render any legend or plots
            }
            PlotMetaType::Plot(p_type) => {
                let colori = color_iter.next().unwrap();

                let mut it = {
                    let basex_ii = xaspect_offset + padding - aa;
                    let basey_ii = yaspect_offset + height - paddingy + bb;
                    let rangex_ii = [minx, maxx];
                    let rangey_ii = [miny, maxy];
                    let maxx_ii = scalex;
                    let maxy_ii = scaley;

                    ppp.plots().map(move |(x, y)| {
                        [
                            basex_ii + x.scale(rangex_ii, maxx_ii),
                            basey_ii - y.scale(rangey_ii, maxy_ii),
                        ]
                    })
                };

                //
                // Using `cargo bloat` determined that these lines reduces alot of code bloat.
                // in debug builds.
                //
                let it: &mut dyn Iterator<Item = [f64; 2]> = &mut it;

                let precision = canvas.precision;
                render(
                    writer,
                    it,
                    PlotRenderInfo {
                        canvas,
                        p_type,
                        colori,
                        precision,
                        bar_width: canvas.bar_width,
                    },
                )?;
            }
        }
    }

    if !names.is_empty() {
        let j = hbuild::from_closure(|w| {
            //TODO redesign so that not all names need to be written to memory at once
            for (typ, name, i) in names.iter() {
                match typ {
                    PlotMetaType::Text => {
                        //assert_eq!(ppp.plots().count(), 0);

                        // don't need to render any legend or plots
                    }
                    &PlotMetaType::Plot(p_type) => {
                        let colori = color_iter2.next().unwrap();
                        let legendy1 =
                            paddingy - yaspect_offset - padding / 8.0 + (*i as f64) * spacing;

                        if !name.is_empty() {
                            render_label(
                                w,
                                PlotRenderInfo2 {
                                    canvas,
                                    p_type,
                                    colori,
                                    legendy1,
                                },
                            )?;
                        }
                    }
                }
            }
            Ok(())
        });

        writer.render(j)?;

        let j = hbuild::from_closure(|w| {
            for (typ, name, i) in names.into_iter() {

                let class=match typ{
                    PlotMetaType::Plot(e) => {
                        match e{
                            PlotType::Scatter => "poloto_scatter",
                            PlotType::Line => "poloto_line",
                            PlotType::Histo => "poloto_histo",
                            PlotType::LineFill => "poloto_linefill",
                            PlotType::LineFillRaw => "poloto_linefillraw",
                            PlotType::Bars => "poloto_bars",
                        }
                    },
                    PlotMetaType::Text => "",
                };

                let text = hbuild::elem("text")
                    .with(attrs!(
                        ("class", format_move!("poloto_legend poloto_text {} poloto{}",class,i)),
                        ("x", width - padding / 1.2),
                        ("y", paddingy - yaspect_offset + (i as f64) * spacing)
                    ))
                    .inline();

                w.render(text.append(name))?;
            }
            Ok(())
        });
        writer.render(j)?;
    }
    Ok(())
}

struct PlotRenderInfo2<'a> {
    canvas: &'a RenderOptionsResult,
    p_type: PlotType,
    colori: usize,
    legendy1: f64,
}

struct PlotRenderInfo<'a> {
    canvas: &'a RenderOptionsResult,
    p_type: PlotType,
    colori: usize,
    precision: usize,
    bar_width: f64,
}

fn render_label(writer: &mut elem::ElemWrite, info: PlotRenderInfo2) -> fmt::Result {
    let PlotRenderInfo2 {
        canvas,
        p_type,
        colori,
        legendy1,
        ..
    } = info;

    let RenderOptionsResult {
        padding, legendx1, ..
    } = *canvas;

    match p_type {
        PlotType::Line => {
            writer.render(hbuild::single("line").with(attrs!(
                (
                    "class",
                    format_move!(
                        "poloto_legend poloto_imgs poloto_line poloto{} poloto_stroke",
                        colori
                    )
                ),
                ("x1", legendx1),
                ("x2", legendx1 + padding / 3.0),
                ("y1", legendy1),
                ("y2", legendy1)
            )))?;
        }
        PlotType::Scatter => {
            writer.render(hbuild::single("line").with(attrs!(
                (
                    "class",
                    format_move!(
                        "poloto_legend poloto_imgs poloto_scatter poloto{} poloto_stroke",
                        colori,
                    ),
                ),
                ("x1", legendx1 + padding / 30.0),
                ("x2", legendx1 + padding / 30.0),
                ("y1", legendy1),
                ("y2", legendy1)
            )))?;
        }
        PlotType::Histo => {
            //let g=hbuild::elem("g").with();

            let g=hbuild::single("rect").with(attrs!(
                (
                    "class",
                    format_move!(
                        "poloto_legend poloto_imgs poloto_histo poloto{} poloto_fill",
                        colori,
                    ),
                ),
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            ));

            writer.render(g)?;
        }
        PlotType::LineFill => {
            writer.render(hbuild::single("rect").with(attrs!(
                (
                    "class",
                    format_move!(
                        "poloto_legend poloto_imgs poloto_linefill poloto{} poloto_fill",
                        colori,
                    ),
                ),
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )))?;
        }

        PlotType::LineFillRaw => {
            writer.render(hbuild::single("rect").with(attrs!(
                (
                    "class",
                    format_move!(
                        "poloto_legend poloto_imgs poloto_linefillraw poloto{} poloto_fill",
                        colori,
                    ),
                ),
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )))?;
        }

        PlotType::Bars => {
            writer.render(hbuild::single("rect").with(attrs!(
                (
                    "class",
                    format_move!(
                        "poloto_legend poloto_imgs poloto_bars poloto{} poloto_fill",
                        colori
                    ),
                ),
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )))?;
        }
    }

    Ok(())
}

fn render(
    writer: &mut elem::ElemWrite,
    it: impl Iterator<Item = [f64; 2]>,
    info: PlotRenderInfo,
) -> fmt::Result {
    let PlotRenderInfo {
        canvas,
        p_type,
        colori,
        precision,
        bar_width,
        ..
    } = info;

    let RenderOptionsResult {
        height,
        padding,
        paddingy,
        ..
    } = *canvas;

    let ffmt = FloatFmt::new(precision);

    match p_type {
        PlotType::Line => {
            writer.render(hbuild::single("path").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!("poloto_plot poloto_imgs poloto_line poloto{} poloto_stroke", colori)
                ),
                ("fill", "none"),
                ("stroke", "black"),
                Line::new(it, ffmt)
            )))?;
        }
        PlotType::Scatter => {
            writer.render(hbuild::single("path").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!("poloto_plot poloto_imgs poloto_scatter poloto{} poloto_stroke", colori),
                ),
                hbuild::path_from_closure(|w| {
                    let mut w = w.start();
                    use hypermelon::attr::PathCommand::*;
                    for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                        w.put(M(ffmt.disp(x), ffmt.disp(y)))?;
                        w.put(H_(ffmt.disp(0.0)))?;
                    }
                    Ok(())
                })
            )))?;
        }
        PlotType::Histo => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!("poloto_plot poloto_imgs poloto_histo poloto{} poloto_fill", colori),
                )
            ));

            let h = hbuild::from_closure(|w| {
                let mut last = None;
                for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                    if let Some((lx, ly)) = last {
                        w.render(hbuild::single("rect").with(attrs!(
                            ("x", ffmt.disp(lx)),
                            ("y", ffmt.disp(ly)),
                            ("width", (padding * 0.02).max((x - lx) - (padding * 0.02))),
                            ("height", height - paddingy - ly)
                        )))?;
                    }
                    last = Some((x, y))
                }
                Ok(())
            });

            writer.render(g.append(h))?;
        }
        PlotType::LineFill => {
            writer.render(hbuild::single("path").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!("poloto_plot poloto_imgs poloto_linefill poloto{} poloto_fill", colori),
                ),
                LineFill::new(it, ffmt, height - paddingy, true)
            )))?;
        }
        PlotType::LineFillRaw => {
            writer.render(hbuild::single("path").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!("poloto_plot poloto_imgs poloto_linefill poloto{} poloto_fill", colori),
                ),
                LineFill::new(it, ffmt, height - paddingy, false)
            )))?;
        }
        PlotType::Bars => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!("poloto_plot poloto_imgs poloto_histo poloto{} poloto_fill", colori),
                )
            ));

            let h = hbuild::from_closure(|w| {
                for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                    w.render(hbuild::single("rect").with(attrs!(
                        ("x", ffmt.disp(padding)),
                        ("y", ffmt.disp(y - bar_width / 2.0)),
                        ("width", x - padding),
                        ("height", bar_width)
                    )))?;
                }
                Ok(())
            });

            writer.render(g.append(h))?;
        }
    }
    Ok(())
}

struct LineFill<I> {
    it: I,
    fmt: FloatFmt,
    base_line: f64,
    add_start_end_base: bool,
}
impl<I: Iterator<Item = [f64; 2]>> LineFill<I> {
    pub fn new(it: I, fmt: FloatFmt, base_line: f64, add_start_end_base: bool) -> Self {
        LineFill {
            it,
            fmt,
            base_line,
            add_start_end_base,
        }
    }
}
impl<I: Iterator<Item = [f64; 2]>> attr::Attr for LineFill<I> {
    fn render(self, w: &mut attr::AttrWrite) -> fmt::Result {
        let LineFill {
            mut it,
            fmt,
            base_line,
            add_start_end_base,
        } = self;

        w.render(hypermelon::build::path_from_closure(|w| {
            let mut w = w.start();

            if let Some([startx, starty]) = it.next() {
                use hypermelon::attr::PathCommand::*;

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
                                    w.put(M(fmt.disp(last[0]), fmt.disp(base_line)))?;
                                    w.put(L(fmt.disp(last[0]), fmt.disp(last[1])))?;
                                } else {
                                    w.put(M(fmt.disp(last[0]), fmt.disp(last[1])))?;
                                }
                                first = false;
                            }
                            last_finite = Some([newx, newy]);
                            w.put(L(fmt.disp(newx), fmt.disp(newy)))?;
                        }
                        (true, false) => {
                            w.put(M(fmt.disp(newx), fmt.disp(newy)))?;
                        }
                        (false, true) => {
                            w.put(L(fmt.disp(last[0]), fmt.disp(base_line)))?;
                        }
                        _ => {}
                    };
                    last = [newx, newy];
                }
                if let Some([x, _]) = last_finite {
                    if add_start_end_base {
                        w.put(L(fmt.disp(x), fmt.disp(base_line)))?;
                    }
                    w.put(Z())?;
                }
            }

            Ok(())
        }))
    }
}

struct Line<I> {
    it: I,
    fmt: FloatFmt,
}
impl<I: Iterator<Item = [f64; 2]>> Line<I> {
    pub fn new(it: I, fmt: FloatFmt) -> Self {
        Line { it, fmt }
    }
}
impl<I: Iterator<Item = [f64; 2]>> attr::Attr for Line<I> {
    fn render(self, w: &mut attr::AttrWrite) -> fmt::Result {
        let Line { mut it, fmt } = self;

        w.render(hypermelon::build::path_from_closure(|w| {
            let mut w = w.start();

            if let Some([startx, starty]) = it.next() {
                use hypermelon::attr::PathCommand::*;

                let mut last = [startx, starty];
                let mut first = true;
                for [newx, newy] in it {
                    match (
                        newx.is_finite() && newy.is_finite(),
                        last[0].is_finite() && last[1].is_finite(),
                    ) {
                        (true, true) => {
                            if first {
                                w.put(M(fmt.disp(last[0]), fmt.disp(last[1])))?;
                                first = false;
                            }
                            w.put(L(fmt.disp(newx), fmt.disp(newy)))?;
                        }
                        (true, false) => {
                            w.put(M(fmt.disp(newx), fmt.disp(newy)))?;
                        }
                        _ => {}
                    };
                    last = [newx, newy];
                }
            }
            Ok(())
        }))
    }
}
