use std::iter::FusedIterator;

use super::*;

use crate::build::*;

struct SinglePlotIterator<I> {
    it: I,
    size_hint: (usize, Option<usize>),
    finished: bool,
}
impl<I: Iterator<Item = PlotTag<L, D>>, L: Point, D: Display> SinglePlotIterator<I> {
    fn new(mut it: I) -> Option<(Self, D, PlotMetaType)> {
        if let Some(o) = it.next() {
            match o {
                PlotTag::Start {
                    name,
                    typ,
                    size_hint,
                } => Some((
                    Self {
                        it,
                        size_hint,
                        finished: false,
                    },
                    name,
                    typ,
                )),
                PlotTag::Plot(_) => panic!("expected start"),
                PlotTag::Finish() => panic!("expected start"),
            }
        } else {
            None
        }
    }
}

impl<I: ExactSizeIterator<Item = PlotTag<L, D>>, L: Point, D: Display> ExactSizeIterator
    for SinglePlotIterator<I>
{
}
impl<I: Iterator<Item = PlotTag<L, D>>, L: Point, D: Display> FusedIterator
    for SinglePlotIterator<I>
{
}
impl<I: Iterator<Item = PlotTag<L, D>>, L: Point, D: Display> Iterator for SinglePlotIterator<I> {
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let o = self.it.next().unwrap();

        match o {
            PlotTag::Start { .. } => panic!("did not expect start"),
            PlotTag::Plot(a) => Some(a),
            PlotTag::Finish() => {
                self.finished = true;
                None
            }
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.size_hint
    }
}

pub(super) fn render_plot<
    X: PlotNum,
    Y: PlotNum,
    L: Point<X = X, Y = Y>,
    P: build::PlotIterator<L = L>,
>(
    writer: &mut elem::ElemWrite,
    boundx: &ticks::DataBound<X>,
    boundy: &ticks::DataBound<Y>,
    canvas: &RenderFrame,
    plots_all: P,
) -> std::fmt::Result {
    let RenderFrame {
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

    //let mut f = crate::build::RenderablePlotIter::new(plots_all);

    let mut names = vec![];

    let PlotRes {
        mut it, num_plots, ..
    } = plots_all.unpack();

    for i in 0..num_plots {
        let (mut it, label, typ) = SinglePlotIterator::new(&mut it).unwrap();

        let mut name = String::new();
        use std::fmt::Write;
        write!(&mut name, "{}", label).unwrap();

        let name_exists = !name.is_empty();

        if name_exists {
            names.push((typ, name, i));
        }

        let aa = minx.scale(&[minx, maxx], scalex);
        let bb = miny.scale(&[miny, maxy], scaley);

        match typ {
            PlotMetaType::Text => {
                assert_eq!((&mut it).count(), 0);

                // don't need to render any legend or plots
            }
            PlotMetaType::Plot(p_type) => {
                let colori = color_iter.next().unwrap();

                let rangex_ii = &[minx, maxx];
                let rangey_ii = &[miny, maxy];
                let basex_ii = xaspect_offset + padding - aa;
                let basey_ii = yaspect_offset + height - paddingy + bb;
                let maxx_ii = scalex;
                let maxy_ii = scaley;

                let it = it.map(move |l| {
                    let (x, y) = l.get();
                    [
                        basex_ii + x.scale(rangex_ii, maxx_ii),
                        basey_ii - y.scale(rangey_ii, maxy_ii),
                    ]
                });

                let precision = canvas.precision;
                let k = render(
                    it,
                    PlotRenderInfo {
                        canvas,
                        p_type,
                        colori,
                        precision,
                        bar_width: canvas.bar_width,
                    },
                );
                writer.render(k)?;
            }
        }
    }

    assert!(SinglePlotIterator::new(&mut it).is_none());

    let j = (!names.is_empty()).then(|| {
        let aa = hbuild::from_iter(names.iter().map(|(typ, name, i)| {
            match typ {
                PlotMetaType::Text => {
                    // don't need to render any legend or plots
                    None
                }
                &PlotMetaType::Plot(p_type) => {
                    let colori = color_iter2.next().unwrap();
                    let legendy1 =
                        paddingy - yaspect_offset - padding / 8.0 + (*i as f64) * spacing;

                    (!name.is_empty()).then(|| {
                        render_label(PlotRenderInfo2 {
                            canvas,
                            p_type,
                            colori,
                            legendy1,
                        })
                    })
                }
            }
        }));

        let bb = hbuild::from_iter(names.iter().map(|(typ, name, i)| {
            let class = match typ {
                PlotMetaType::Plot(e) => match e {
                    PlotType::Scatter => "poloto_scatter",
                    PlotType::Line => "poloto_line",
                    PlotType::Histo => "poloto_histo",
                    PlotType::LineFill => "poloto_linefill",
                    PlotType::LineFillRaw => "poloto_linefillraw",
                    PlotType::Bars => "poloto_bars",
                },
                PlotMetaType::Text => "",
            };

            let text = hbuild::elem("text")
                .with(attrs!(
                    (
                        "class",
                        format_move!("poloto_legend poloto_text {} poloto{}", class, i)
                    ),
                    ("x", width - padding / 1.2),
                    ("y", paddingy - yaspect_offset + (*i as f64) * spacing)
                ))
                .inline();

            text.append(hbuild::raw(name))
        }));

        aa.chain(bb)
    });

    writer.render(j)
}

struct PlotRenderInfo2<'a> {
    canvas: &'a RenderFrame,
    p_type: PlotType,
    colori: usize,
    legendy1: f64,
}

struct PlotRenderInfo<'a> {
    canvas: &'a RenderFrame,
    p_type: PlotType,
    colori: usize,
    precision: usize,
    bar_width: f64,
}

fn render_label(info: PlotRenderInfo2) -> impl hypermelon::elem::Elem + hypermelon::elem::Locked {
    let PlotRenderInfo2 {
        canvas,
        p_type,
        colori,
        legendy1,
        ..
    } = info;

    let RenderFrame {
        padding, legendx1, ..
    } = *canvas;

    let vals = match p_type {
        PlotType::Line => {
            let g = hbuild::elem("g").with((
                "class",
                format_move!(
                    "poloto_legend poloto_imgs poloto_line poloto{} poloto_stroke",
                    colori
                ),
            ));

            let g = g.append(hbuild::single("line").with(attrs!(
                ("x1", legendx1),
                ("x2", legendx1 + padding / 3.0),
                ("y1", legendy1),
                ("y2", legendy1)
            )));

            (g.inline().some(), None, None, None, None, None)
        }
        PlotType::Scatter => {
            let g = hbuild::elem("g").with((
                "class",
                format_move!(
                    "poloto_legend poloto_imgs poloto_scatter poloto{} poloto_stroke",
                    colori,
                ),
            ));
            let g = g.append(hbuild::single("line").with(attrs!(
                ("x1", legendx1 + padding / 30.0),
                ("x2", legendx1 + padding / 30.0),
                ("y1", legendy1),
                ("y2", legendy1)
            )));

            (None, g.inline().some(), None, None, None, None)
        }
        PlotType::Histo => {
            let g = hbuild::elem("g").with((
                "class",
                format_move!(
                    "poloto_legend poloto_imgs poloto_histo poloto{} poloto_fill",
                    colori,
                ),
            ));

            let g = g.append(hbuild::single("rect").with(attrs!(
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )));

            (None, None, g.inline().some(), None, None, None)
        }
        PlotType::LineFill => {
            let g = hbuild::elem("g").with((
                "class",
                format_move!(
                    "poloto_legend poloto_imgs poloto_linefill poloto{} poloto_fill",
                    colori,
                ),
            ));

            let g = g.append(hbuild::single("rect").with(attrs!(
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )));

            (None, None, None, g.inline().some(), None, None)
        }

        PlotType::LineFillRaw => {
            let g = hbuild::elem("g").with((
                "class",
                format_move!(
                    "poloto_legend poloto_imgs poloto_linefillraw poloto{} poloto_fill",
                    colori,
                ),
            ));

            let g = g.append(hbuild::single("rect").with(attrs!(
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )));

            (None, None, None, None, g.inline().some(), None)
        }

        PlotType::Bars => {
            let g = hbuild::elem("g").with((
                "class",
                format_move!(
                    "poloto_legend poloto_imgs poloto_bars poloto{} poloto_fill",
                    colori
                ),
            ));
            let g = g.append(hbuild::single("rect").with(attrs!(
                ("x", legendx1),
                ("y", legendy1 - padding / 30.0),
                ("width", padding / 3.0),
                ("height", padding / 20.0),
                ("rx", padding / 30.0),
                ("ry", padding / 30.0)
            )));

            (None, None, None, None, None, g.inline().some())
        }
    };

    let (a, b, c, d, e, f) = vals;
    a.chain(b).chain(c).chain(d).chain(e).chain(f)
}

fn render(
    it: impl Iterator<Item = [f64; 2]>,
    info: PlotRenderInfo,
) -> impl hypermelon::elem::Elem + hypermelon::elem::Locked {
    let PlotRenderInfo {
        canvas,
        p_type,
        colori,
        precision,
        bar_width,
        ..
    } = info;

    let RenderFrame {
        height,
        padding,
        paddingy,
        ..
    } = *canvas;

    let ffmt = FloatFmt::new(precision);

    let vals = match p_type {
        PlotType::Line => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!(
                        "poloto_plot poloto_imgs poloto_line poloto{} poloto_stroke",
                        colori
                    )
                ),
                ("fill", "none"),
                ("stroke", "black")
            ));

            let j = hbuild::single("path").with(attrs!(Line::new(it, ffmt)));

            (g.append(j).some(), None, None, None, None, None)
        }
        PlotType::Scatter => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!(
                        "poloto_plot poloto_imgs poloto_scatter poloto{} poloto_stroke",
                        colori
                    ),
                )
            ));

            let j = hbuild::single("path").with(attrs!(hbuild::path_from_closure(move |w| {
                let mut w = w.start();
                use hypermelon::attr::PathCommand::*;
                for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                    w.put(M(ffmt.disp(x), ffmt.disp(y)))?;
                    w.put(H_(ffmt.disp(0.0)))?;
                }
                Ok(())
            })));

            (None, g.append(j).some(), None, None, None, None)
        }
        PlotType::Histo => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!(
                        "poloto_plot poloto_imgs poloto_histo poloto{} poloto_fill",
                        colori
                    ),
                )
            ));

            let h = hbuild::from_closure(move |w| {
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

            (None, None, g.append(h).some(), None, None, None)
        }
        PlotType::LineFill => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!(
                        "poloto_plot poloto_imgs poloto_linefill poloto{} poloto_fill",
                        colori
                    ),
                )
            ));

            let j = hbuild::single("path").with(attrs!(LineFill::new(
                it,
                ffmt,
                height - paddingy,
                true
            )));

            (None, None, None, g.append(j).some(), None, None)
        }
        PlotType::LineFillRaw => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!(
                        "poloto_plot poloto_imgs poloto_linefill poloto{} poloto_fill",
                        colori
                    ),
                )
            ));
            let j = hbuild::single("path").with(attrs!(LineFill::new(
                it,
                ffmt,
                height - paddingy,
                false
            )));

            (None, None, None, None, g.append(j).some(), None)
        }
        PlotType::Bars => {
            let g = hbuild::elem("g").with(attrs!(
                ("id", format_move!("poloto_plot{}", colori)),
                (
                    "class",
                    format_move!(
                        "poloto_plot poloto_imgs poloto_histo poloto{} poloto_fill",
                        colori
                    ),
                )
            ));

            let h = hbuild::from_closure(move |w| {
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

            (None, None, None, None, None, g.append(h).some())
        }
    };

    let (a, b, c, d, e, f) = vals;
    a.chain(b).chain(c).chain(d).chain(e).chain(f)
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
