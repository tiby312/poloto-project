use crate::*;
mod render_base;
mod render_plot;

///
/// Main render function.
///
pub fn render<P: BaseAndPlotsFmt>(
    mut writer: impl fmt::Write,
    all: P,
    boundx: DataBound<P::X>,
    boundy: DataBound<P::Y>,
    canvas: Canvas,
) -> fmt::Result {
    let (base_fmt, plot_fmt) = all.gen();

    //render background
    {
        let mut writer = tagger::new(&mut writer);
        writer.single("circle", |d| {
            d.attr("r", "1e5")?;
            d.attr("class", "poloto_background")
        })?;
    }

    render::render_plot::render_plot(&mut writer, &boundx, &boundy, &canvas, plot_fmt)?;
    render::render_base::render_base(&mut writer, &boundx, &boundy, &canvas, base_fmt)
}

pub trait BaseAndPlotsFmt {
    type X: PlotNum;
    type Y: PlotNum;
    type A: BaseFmtAndTicks<X = Self::X, Y = Self::Y>;
    type B: AllPlotFmt<Item2 = (Self::X, Self::Y)>;
    fn gen(self) -> (Self::A, Self::B);
}

#[derive(Copy, Clone, Debug)]
pub enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
    LineFillRaw,
    Bars,
}

#[derive(Copy, Clone, Debug)]
pub enum PlotMetaType {
    Plot(PlotType),
    Text,
}

pub trait OnePlotFmt {
    type Item;
    type It: Iterator<Item = Self::Item>;
    fn get_iter(&mut self) -> Self::It;
    fn plot_type(&mut self) -> PlotMetaType;
    fn fmt(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
}

pub trait AllPlotFmt {
    type Item2;
    type InnerIt: OnePlotFmt<Item = Self::Item2>;
    type It: Iterator<Item = Self::InnerIt>;
    fn iter(self) -> Self::It;
}

///
/// Trait that captures all user defined plot formatting. This includes:
///
/// * The distribution of ticks on each axis,
///
/// * The formatting of:
///     * title
///     * xname
///     * yname
///     * xticks
///     * yticks
///
pub trait BaseFmtAndTicks {
    type X: PlotNum;
    type Y: PlotNum;
    type Fmt: BaseFmt<X = Self::X, Y = Self::Y>;
    type XI: IntoIterator<Item = Self::X>;
    type YI: IntoIterator<Item = Self::Y>;
    fn gen(self) -> (Self::Fmt, TickInfo<Self::XI>, TickInfo<Self::YI>);
}

pub trait NumFmt {
    type K: Display;
    fn fmt(&self, a: f64) -> Self::K;
}

pub struct MyPathBuilder<'a, 'b, T: fmt::Write, K> {
    num_fmt: K,
    path: &'a mut tagger::PathBuilder<'b, T>,
}
impl<T: fmt::Write, K: NumFmt> MyPathBuilder<'_, '_, T, K> {
    pub fn put(&mut self, a: tagger::PathCommand<f64>) -> fmt::Result {
        self.path.put(a.map(|x| self.num_fmt.fmt(x)))
    }
    pub fn put_z(&mut self) -> fmt::Result {
        self.path.put(tagger::PathCommand::Z(""))
    }
}

pub fn line_fill<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
    base_line: f64,
    add_start_end_base: bool,
    num_fmt: impl NumFmt,
) -> fmt::Result {
    let mut path = MyPathBuilder { num_fmt, path };

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
            path.put_z()?;
        }
    }
    Ok(())
}

pub fn line<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
    num_fmt: impl NumFmt,
) -> fmt::Result {
    let mut path = MyPathBuilder { num_fmt, path };

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
