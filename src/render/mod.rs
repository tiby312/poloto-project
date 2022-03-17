//!
//! Tools to render plots
//!
use crate::*;
mod render_base;
mod render_plot;

///
/// One-time function to write to a `fmt::Write`.
///
pub trait Disp {
    fn disp<T: fmt::Write>(self, writer: T) -> fmt::Result;
}

///
/// Created by [`Stager::plot`]
///
pub struct Plotter<A: Disp> {
    inner: Option<A>,
    dim: [f64; 2],
}
impl<A: Disp> Plotter<A> {
    pub(crate) fn new(a: A, dim: [f64; 2]) -> Self {
        Plotter {
            inner: Some(a),
            dim,
        }
    }

    pub fn get_dim(&self) -> [f64; 2] {
        self.dim
    }

    ///
    /// Use the plot iterators to write out the graph elements.
    /// Does not add a svg tag, or any styling elements.
    /// Use this if you want to embed a svg into your html.
    /// You will just have to add your own svg sag and then supply styling.
    ///
    /// Panics if the render fails.
    ///
    /// In order to meet a more flexible builder pattern, instead of consuming the Plotter,
    /// this function will mutable borrow the Plotter and leave it with empty data.
    ///
    /// ```
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut s = poloto::build::line("",data);
    /// let mut plotter=s.build().stage().plot("title","x","y");
    ///
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```

    pub fn render<T: std::fmt::Write>(&mut self, writer: T) -> fmt::Result {
        self.inner.take().unwrap().disp(writer)
    }
}

///
/// Main render function.
///
pub(crate) fn render<P: BaseAndPlotsFmt>(
    mut writer: impl fmt::Write,
    all: P,
    boundx: ticks::DataBound<P::X>,
    boundy: ticks::DataBound<P::Y>,
    canvas: impl Borrow<Canvas>,
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

    let canvas = canvas.borrow();
    render::render_plot::render_plot(&mut writer, &boundx, &boundy, canvas, plot_fmt)?;
    render::render_base::render_base(&mut writer, &boundx, &boundy, canvas, base_fmt)
}

pub(crate) trait BaseAndPlotsFmt {
    type X: PlotNum;
    type Y: PlotNum;
    type A: BaseFmtAndTicks<X = Self::X, Y = Self::Y>;
    type B: RenderablePlotIterator<X = Self::X, Y = Self::Y>;
    fn gen(self) -> (Self::A, Self::B);
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
pub(crate) trait BaseFmtAndTicks {
    type X: PlotNum;
    type Y: PlotNum;
    type Fmt: BaseFmt<X = Self::X, Y = Self::Y>;
    type XI: IntoIterator<Item = Self::X>;
    type YI: IntoIterator<Item = Self::Y>;
    fn gen(self) -> (Self::Fmt, TickInfo<Self::XI>, TickInfo<Self::YI>);
}

trait NumFmt {
    type K: Display;
    fn fmt(&self, a: f64) -> Self::K;
}

struct MyPathBuilder<'a, 'b, T: fmt::Write, K> {
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

fn line_fill<T: std::fmt::Write>(
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

fn line<T: std::fmt::Write>(
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

///
/// Build a [`Canvas`]
///
/// Created by [`canvas()`]
///
pub struct CanvasBuilder {
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    dim: Option<[f64; 2]>,
    xtick_lines: bool,
    ytick_lines: bool,
    precision: usize,
    bar_width: f64,
}

impl Default for CanvasBuilder {
    fn default() -> Self {
        CanvasBuilder {
            num_css_classes: Some(8),
            preserve_aspect: false,
            dim: None,
            xtick_lines: false,
            ytick_lines: false,
            precision: 2,
            bar_width: 20.0,
        }
    }
}

impl CanvasBuilder {
    pub fn with_dim(&mut self, dim: [f64; 2]) -> &mut Self {
        self.dim = Some(dim);
        self
    }

    pub fn with_tick_lines(&mut self, a: bool, b: bool) -> &mut Self {
        self.xtick_lines = a;
        self.ytick_lines = b;
        self
    }

    ///
    /// The number of distinct css classes. If there are more plots than
    /// classes, then they will wrap around. The default value is 8.
    ///
    /// A value of None, means it will never wrap around.
    ///
    pub fn num_css_class(&mut self, a: Option<usize>) -> &mut Self {
        self.num_css_classes = a;
        self
    }

    ///
    /// Specify the number of decimal places of each plot value in the SVG output itself.
    /// Defaults to a precision of 2 (2 decimal places).
    ///
    /// For most usecases, you don't need a high precision. However, if you plan on blowing
    /// up the svg output significantly or zooming in a bunch, then you might want better
    /// precision.
    ///
    pub fn with_precision(&mut self, precision: usize) -> &mut Self {
        self.precision = precision;
        self
    }
    ///
    /// Preserve the aspect ratio by drawing a smaller graph in the same area.
    ///
    pub fn preserve_aspect(&mut self) -> &mut Self {
        self.preserve_aspect = true;
        self
    }

    pub fn bar_width(&mut self, val: f64) -> &mut Self {
        self.bar_width = val;
        self
    }

    pub fn build(&mut self) -> Canvas {
        let (width, height) = if let Some([x, y]) = self.dim {
            (x, y)
        } else {
            (crate::WIDTH as f64, crate::HEIGHT as f64)
        };

        let ideal_dash_size = 20.0;
        let padding = 150.0;
        let paddingy = 100.0;

        //The range over which the data will be scaled to fit
        let (scalex, scaley) = if self.preserve_aspect {
            if width > height {
                (height - paddingy * 2.0, height - paddingy * 2.0)
            } else {
                (width - padding * 2.0, width - padding * 2.0)
            }
        } else {
            (width - padding * 2.0, height - paddingy * 2.0)
        };

        let distancex_min_to_max = scalex;
        let distancey_min_to_max = scaley;

        let (xaspect_offset, yaspect_offset) = if self.preserve_aspect {
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
            boundx: ticks::CanvasBound {
                ideal_num_steps: ideal_num_xsteps,
                ideal_dash_size,
                max: scalex,
                axis: Axis::X,
            },
            boundy: ticks::CanvasBound {
                ideal_num_steps: ideal_num_ysteps,
                ideal_dash_size,
                max: scaley,
                axis: Axis::Y,
            },

            width,
            height,
            padding,
            paddingy,
            xaspect_offset,
            yaspect_offset,
            spacing,
            legendx1,
            num_css_classes: self.num_css_classes,
            xtick_lines: self.xtick_lines,
            ytick_lines: self.ytick_lines,
            precision: self.precision,
            bar_width: self.bar_width,
        }
    }
}

///
/// Contains graphical information for a svg graph.
///
/// Built from [`canvas()`]
///
pub struct Canvas {
    boundx: ticks::CanvasBound,
    boundy: ticks::CanvasBound,
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
    xaspect_offset: f64,
    yaspect_offset: f64,
    spacing: f64,
    legendx1: f64,
    num_css_classes: Option<usize>,
    xtick_lines: bool,
    ytick_lines: bool,
    precision: usize,
    bar_width: f64,
}
impl Canvas {
    fn boundx(&self) -> ticks::CanvasBound {
        self.boundx.clone()
    }
    fn boundy(&self) -> ticks::CanvasBound {
        self.boundy.clone()
    }

    pub fn get_dim(&self) -> [f64; 2] {
        [self.width, self.height]
    }
}

///
/// Build a [`Canvas`]
///
pub fn canvas() -> CanvasBuilder {
    CanvasBuilder::default()
}

///
/// Created by [`Data::stage()`] or [`Data::stage_with`].
///
pub struct Stager<P: RenderablePlotIterator, K: Borrow<Canvas>> {
    boundx: ticks::Bound<P::X>,
    boundy: ticks::Bound<P::Y>,
    plots: P,
    canvas: K,
}

impl<P: RenderablePlotIterator, K: Borrow<Canvas>> Stager<P, K> {
    fn new(
        plots: P,
        boundx: ticks::DataBound<P::X>,
        boundy: ticks::DataBound<P::Y>,
        canvas: K,
    ) -> Self {
        let boundx = ticks::Bound {
            data: boundx,
            canvas: canvas.borrow().boundx(),
        };
        let boundy = ticks::Bound {
            data: boundy,
            canvas: canvas.borrow().boundy(),
        };
        Stager {
            canvas,
            plots,
            boundx,
            boundy,
        }
    }

    pub fn bounds(&self) -> (&ticks::Bound<P::X>, &ticks::Bound<P::Y>) {
        (&self.boundx, &self.boundy)
    }

    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///

    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///
    pub fn plot(
        self,
        title: impl Display,
        xname: impl Display,
        yname: impl Display,
    ) -> Plotter<impl Disp>
    where
        P::X: HasDefaultTicks,
        P::Y: HasDefaultTicks,
    {
        let (x, xt) = ticks::from_default(&self.boundx);
        let (y, yt) = ticks::from_default(&self.boundy);

        let p = plot_fmt(title, xname, yname, xt, yt);
        self.plot_with(x, y, p)
    }

    ///
    /// Move to final stage in pipeline collecting the title/xname/yname.
    /// Unlike [`Stager::plot`] User must supply own tick distribution.
    ///
    pub fn plot_with<XI, YI, PF>(
        self,
        xtick: TickInfo<XI>,
        ytick: TickInfo<YI>,
        plot_fmt: PF,
    ) -> Plotter<impl Disp>
    where
        XI: IntoIterator<Item = P::X>,
        YI: IntoIterator<Item = P::Y>,
        PF: BaseFmt<X = P::X, Y = P::Y>,
    {
        ///
        /// Wrap tick iterators and a [`PlotFmt`] behind the [`PlotFmtAll`] trait.
        ///
        struct PlotAllStruct<XI: IntoIterator, YI: IntoIterator, PF: BaseFmt> {
            xtick: TickInfo<XI>,
            ytick: TickInfo<YI>,
            fmt: PF,
        }

        impl<XI: IntoIterator, YI: IntoIterator, PF: BaseFmt<X = XI::Item, Y = YI::Item>>
            BaseFmtAndTicks for PlotAllStruct<XI, YI, PF>
        where
            XI::Item: PlotNum,
            YI::Item: PlotNum,
        {
            type X = PF::X;
            type Y = PF::Y;
            type Fmt = PF;
            type XI = XI;
            type YI = YI;

            fn gen(self) -> (Self::Fmt, TickInfo<Self::XI>, TickInfo<Self::YI>) {
                (self.fmt, self.xtick, self.ytick)
            }
        }

        self.plot_with_all(PlotAllStruct {
            fmt: plot_fmt,
            xtick,
            ytick,
        })
    }

    ///
    /// Create a plotter directly from a [`BaseFmtAndTicks`]
    ///
    fn plot_with_all<PF: BaseFmtAndTicks<X = P::X, Y = P::Y>>(self, p: PF) -> Plotter<impl Disp> {
        struct Combine<A: BaseFmtAndTicks, B: RenderablePlotIterator> {
            pub a: A,
            pub b: B,
        }

        impl<A: BaseFmtAndTicks, B: RenderablePlotIterator<X = A::X, Y = A::Y>> BaseAndPlotsFmt
            for Combine<A, B>
        {
            type X = A::X;
            type Y = A::Y;
            type A = A;
            type B = B;
            fn gen(self) -> (Self::A, Self::B) {
                (self.a, self.b)
            }
        }

        struct InnerPlotter<PF: BaseAndPlotsFmt, K: Borrow<Canvas>> {
            all: PF,
            boundx: ticks::DataBound<PF::X>,
            boundy: ticks::DataBound<PF::Y>,
            canvas: K,
        }

        impl<PF: BaseAndPlotsFmt, K: Borrow<Canvas>> Disp for InnerPlotter<PF, K> {
            fn disp<T: std::fmt::Write>(self, mut writer: T) -> fmt::Result {
                render::render(&mut writer, self.all, self.boundx, self.boundy, self.canvas)
            }
        }

        let pp = InnerPlotter {
            all: Combine {
                a: p,
                b: self.plots,
            },
            boundx: self.boundx.data,
            boundy: self.boundy.data,
            canvas: self.canvas,
        };

        let dim = pp.canvas.borrow().get_dim();
        Plotter::new(pp, dim)
    }
}

impl<P: RenderablePlotIterator> Data<P> {
    pub fn stage(self) -> Stager<P, Canvas> {
        Stager::new(
            self.plots,
            self.boundx,
            self.boundy,
            crate::canvas().build(),
        )
    }

    pub fn stage_with<K: Borrow<Canvas>>(self, canvas: K) -> Stager<P, K> {
        Stager::new(self.plots, self.boundx, self.boundy, canvas)
    }

    pub(crate) fn new(
        boundx: ticks::DataBound<P::X>,
        boundy: ticks::DataBound<P::Y>,
        plots: P,
    ) -> Self {
        Data {
            boundx,
            boundy,
            plots,
        }
    }
}

///
/// Created by [`build::RenderablePlotIteratorExt::collect`]
///
pub struct Data<P: RenderablePlotIterator> {
    boundx: ticks::DataBound<P::X>,
    boundy: ticks::DataBound<P::Y>,
    plots: P,
}
