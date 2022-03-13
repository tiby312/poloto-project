use crate::*;

mod render_base;
mod render_plot;

pub struct Extra<X, Y> {
    pub canvas: render::Canvas,
    pub boundx: Bound<X>,
    pub boundy: Bound<Y>,
    pub xtick_lines: bool,
    pub ytick_lines: bool,
    pub precision: usize,
}

///
/// Main render function.
///
pub fn render<P: BaseAndPlotsFmt>(
    mut writer: impl fmt::Write,
    all: P,
    extra: &Extra<P::X, P::Y>,
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

    render::render_plot::render_plot(&mut writer, extra, plot_fmt)?;
    render::render_base::render_base(&mut writer, extra, base_fmt)
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

#[derive(Copy, Clone)]
pub struct Canvas {
    pub ideal_num_xsteps: u32,
    pub ideal_num_ysteps: u32,
    pub ideal_dash_size: f64,
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
    xaspect_offset: f64,
    yaspect_offset: f64,
    pub scalex: f64,
    pub scaley: f64,
    spacing: f64,
    legendx1: f64,
    num_css_classes: Option<usize>,
}
impl Canvas {
    pub fn get_dim(&self) -> [f64; 2] {
        [self.width, self.height]
    }
    pub fn with_options<X: PlotNum, Y: PlotNum>(
        boundx: [X; 2],
        boundy: [Y; 2],
        dim: Option<[f64; 2]>,
        preserve_aspect: bool,
        num_css_classes: Option<usize>,
    ) -> Self {
        let (width, height) = if let Some([x, y]) = dim {
            (x, y)
        } else {
            (crate::WIDTH as f64, crate::HEIGHT as f64)
        };

        let ideal_dash_size = 20.0;
        let padding = 150.0;
        let paddingy = 100.0;

        //The range over which the data will be scaled to fit
        let (scalex, scaley) = if preserve_aspect {
            if width > height {
                (height - paddingy * 2.0, height - paddingy * 2.0)
            } else {
                (width - padding * 2.0, width - padding * 2.0)
            }
        } else {
            (width - padding * 2.0, height - paddingy * 2.0)
        };

        let [minx, maxx] = boundx;
        let [miny, maxy] = boundy;

        let distancex_min_to_max =
            maxx.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex);
        let distancey_min_to_max =
            maxy.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley);

        let (xaspect_offset, yaspect_offset) = if preserve_aspect {
            if width > height {
                (-padding + width / 2.0 - distancey_min_to_max / 2.0, 0.0)
            } else {
                (
                    0.0,
                    -height + paddingy + height / 2.0 + distancey_min_to_max / 2.0,
                )
            }
        } else {
            (0.0, 0.0)
        };

        let ideal_xtick_spacing = 80.0;

        let ideal_ytick_spacing = 60.0;

        let ideal_num_xsteps = (distancex_min_to_max / ideal_xtick_spacing).floor() as u32;
        let ideal_num_ysteps = (distancey_min_to_max / ideal_ytick_spacing).floor() as u32;
        let ideal_num_xsteps = ideal_num_xsteps.max(2);
        let ideal_num_ysteps = ideal_num_ysteps.max(2);

        let spacing = padding / 3.0;
        let legendx1 = width - padding / 1.2 + padding / 30.0;

        Canvas {
            ideal_num_xsteps,
            ideal_num_ysteps,
            ideal_dash_size,
            width,
            height,
            padding,
            paddingy,
            xaspect_offset,
            yaspect_offset,
            scalex,
            scaley,
            spacing,
            legendx1,
            num_css_classes,
        }
    }
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
