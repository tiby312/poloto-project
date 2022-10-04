use super::*;

use crate::build::*;

pub(super) fn render_plot<P: build::PlotIterator>(
    writer: &mut hypermelon::ElemWrite,
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

    let mut f = crate::build::RenderablePlotIter::new(plots_all);

    for i in 0.. {
        let mut ppp = if let Some(ppp) = f.next_plot() {
            ppp
        } else {
            break;
        };

        let legendy1 = paddingy - yaspect_offset - padding / 8.0 + (i as f64) * spacing;

        let typ = ppp.typ();

        let text = hbuild::elem("text").with(attrs!(
            ("class", "poloto_text poloto_legend_text"),
            ("x", width - padding / 1.2),
            ("y", paddingy - yaspect_offset + (i as f64) * spacing)
        ));

        let name_exists = text.render_closure(writer, |w| {
            let mut wc = util::WriteCounter::new(w.writer());
            ppp.name(&mut wc).unwrap()?;
            //p.write_name(&mut wc)?;
            Ok(wc.get_counter() != 0)
        })?;

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
                        name_exists,
                        colori,
                        legendy1,
                        precision,
                        bar_width: canvas.bar_width,
                    },
                )?;
            }
        }
    }

    Ok(())
}

struct PlotRenderInfo<'a> {
    canvas: &'a RenderOptionsResult,
    p_type: PlotType,
    name_exists: bool,
    colori: usize,
    legendy1: f64,
    precision: usize,
    bar_width: f64,
}

fn render(
    writer: &mut hypermelon::ElemWrite,
    it: impl Iterator<Item = [f64; 2]>,
    info: PlotRenderInfo,
) -> fmt::Result {
    let PlotRenderInfo {
        canvas,
        p_type,
        name_exists,
        colori,
        legendy1,
        precision,
        bar_width,
    } = info;

    let RenderOptionsResult {
        height,
        padding,
        paddingy,
        legendx1,
        ..
    } = *canvas;

    let ffmt = FloatFmt::new(precision);

    match p_type {
        PlotType::Line => {
            if name_exists {
                writer.render(hbuild::single("line").with(attrs!(
                    (
                        "class",
                        format_move!(
                            "poloto_line poloto_legend_icon poloto{}stroke poloto{}legend",
                            colori,
                            colori
                        )
                    ),
                    ("stroke", "black"),
                    ("x1", legendx1),
                    ("x2", legendx1 + padding / 3.0),
                    ("y1", legendy1),
                    ("y2", legendy1)
                )))?;
            }

            writer.render(hbuild::single("path").with(attrs!(
                ("class", format_move!("poloto_line poloto{}stroke", colori)),
                ("fill", "none"),
                ("stroke", "black"),
                Line::new(it, ffmt)
            )))?;
        }
        PlotType::Scatter => {
            if name_exists {
                writer.render(hbuild::single("line").with(attrs!(
                    (
                        "class",
                        format_move!(
                            "poloto_scatter poloto_legend_icon poloto{}stroke poloto{}legend",
                            colori,
                            colori
                        ),
                    ),
                    ("stroke", "black"),
                    ("x1", legendx1 + padding / 30.0),
                    ("x2", legendx1 + padding / 30.0),
                    ("y1", legendy1),
                    ("y2", legendy1)
                )))?;
            }

            writer.render(hbuild::single("path").with(attrs!(
                (
                    "class",
                    format_move!("poloto_scatter poloto{}stroke", colori),
                ),
                hbuild::sink::path_ext(|w| {
                    let mut w = w.start();
                    use hypermelon::build::PathCommand::*;
                    for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                        w.put(M(ffmt.disp(x), ffmt.disp(y)))?;
                        w.put(H_(ffmt.disp(0.0)))?;
                    }
                    Ok(())
                })
            )))?;
        }
        PlotType::Histo => {
            if name_exists {
                writer.render(hbuild::single("rect").with(attrs!(
                    (
                        "class",
                        format_move!(
                            "poloto_histo poloto_legend_icon poloto{}fill poloto{}legend",
                            colori,
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

            let g = hbuild::elem("g")
                .with(("class", format_move!("poloto_histo poloto{}fill", colori)));

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
            if name_exists {
                writer.render(hbuild::single("rect").with(attrs!(
                    (
                        "class",
                        format_move!(
                            "poloto_linefill poloto_legend_icon poloto{}fill poloto{}legend",
                            colori,
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

            writer.render(hbuild::single("path").with(attrs!(
                (
                    "class",
                    format_move!("poloto_linefill poloto{}fill", colori),
                ),
                LineFill::new(it, ffmt, height - paddingy, true)
            )))?;
        }
        PlotType::LineFillRaw => {
            if name_exists {
                writer.render(hbuild::single("rect").with(attrs!(
                    (
                        "class",
                        format_move!(
                            "poloto_linefillraw poloto_legend_icon poloto{}fill poloto{}legend",
                            colori,
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

            writer.render(hbuild::single("path").with(attrs!(
                (
                    "class",
                    format_move!("poloto_linefill poloto{}fill", colori),
                ),
                LineFill::new(it, ffmt, height - paddingy, false)
            )))?;
        }
        PlotType::Bars => {
            if name_exists {
                writer.render(hbuild::single("rect").with(attrs!(
                    (
                        "class",
                        format_move!(
                            "poloto_histo poloto_legend_icon poloto{}fill poloto{}legend",
                            colori,
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

            let g = hbuild::elem("g")
                .with(("class", format_move!("poloto_histo poloto{}fill", colori)));

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
impl<I: Iterator<Item = [f64; 2]>> hypermelon::Attr for LineFill<I> {
    fn render(self, w: &mut hypermelon::AttrWrite) -> fmt::Result {
        let LineFill {
            mut it,
            fmt,
            base_line,
            add_start_end_base,
        } = self;

        w.render(hypermelon::build::sink::path_ext(|w| {
            let mut w = w.start();

            if let Some([startx, starty]) = it.next() {
                use hypermelon::build::PathCommand::*;

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
impl<I: Iterator<Item = [f64; 2]>> hypermelon::Attr for Line<I> {
    fn render(self, w: &mut hypermelon::AttrWrite) -> fmt::Result {
        let Line { mut it, fmt } = self;

        w.render(hypermelon::build::sink::path_ext(|w| {
            let mut w = w.start();

            if let Some([startx, starty]) = it.next() {
                use hypermelon::build::PathCommand::*;

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

#[derive(Copy, Clone)]
pub struct FloatFmt {
    precision: usize,
}
impl FloatFmt {
    pub fn new(precision: usize) -> Self {
        FloatFmt { precision }
    }
    pub fn disp(&self, num: f64) -> impl Display {
        let precision = self.precision;
        util::disp_const(move |f| write!(f, "{:.*}", precision, num))
    }
}
