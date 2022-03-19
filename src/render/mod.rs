//!
//! Tools to render plots
//!

use build::PlotIteratorExt;

use crate::*;
mod render_base;
mod render_plot;

///
/// One-time function to write to a `fmt::Write`.
///
pub trait Disp {
    fn disp(self, writer: &mut dyn fmt::Write) -> fmt::Result;
}
struct Disper<F> {
    func: F,
}
impl<F: FnOnce(&mut dyn fmt::Write) -> fmt::Result> Disper<F> {
    fn new(func: F) -> Disper<F> {
        Disper { func }
    }
}
impl<F: FnOnce(&mut dyn fmt::Write) -> fmt::Result> Disp for Disper<F> {
    fn disp(self, w: &mut dyn fmt::Write) -> fmt::Result {
        (self.func)(w)
    }
}

///
/// Created by [`Data::plot`]
///
pub struct Plotter<A: Disp> {
    inner: Option<A>,
    dim: [f64; 2],
}
impl<A: Disp> Plotter<A> {
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
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let canvas=poloto::render::canvas();
    /// let mut plotter=canvas.build(poloto::build::line("",data)).plot("title","x","y");
    ///
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```
    pub fn render<T: std::fmt::Write>(&mut self, mut writer: T) -> fmt::Result {
        self.inner.take().unwrap().disp(&mut writer)
    }
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
    #[inline(always)]
    pub fn put(&mut self, a: tagger::PathCommand<f64>) -> fmt::Result {
        self.path.put(a.map(|x| self.num_fmt.fmt(x)))
    }

    #[inline(always)]
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

    pub fn with_tick_lines(&mut self, a: [bool; 2]) -> &mut Self {
        self.xtick_lines = a[0];
        self.ytick_lines = a[1];
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

struct RenderOptions<P: PlotIterator<Item = (A::X, A::Y)>, A: BaseFmt, X, Y> {
    plots: P,
    base: A,
    xtick: X,
    ytick: Y,
    boundx: ticks::DataBound<A::X>,
    boundy: ticks::DataBound<A::Y>,
}
impl Canvas {
    pub fn build<X: PlotNum, Y: PlotNum, P: PlotIterator<Item = (X, Y)>>(
        &self,
        plots: P,
    ) -> Data<X, Y, P, &Canvas> {
        self.build_with(plots, [], [])
    }

    pub fn build_with<X: PlotNum, Y: PlotNum, P: PlotIterator<Item = (X, Y)>>(
        &self,
        mut plots: P,
        xmarkers: impl IntoIterator<Item = X>,
        ymarkers: impl IntoIterator<Item = Y>,
    ) -> Data<X, Y, P, &Canvas> {
        let ii = std::iter::from_fn(|| plots.next_bound_point());

        let (boundx, boundy) = util::find_bounds(ii, xmarkers, ymarkers);

        let boundx = ticks::DataBound {
            min: boundx[0],
            max: boundx[1],
        };
        let boundy = ticks::DataBound {
            min: boundy[0],
            max: boundy[1],
        };

        Data {
            boundx,
            boundy,
            plots,
            canvas: self,
        }
    }

    pub fn get_dim(&self) -> [f64; 2] {
        [self.width, self.height]
    }

    fn render<X: PlotNum, Y: PlotNum>(
        &self,
        mut writer: &mut dyn fmt::Write,
        mut data: RenderOptions<
            impl PlotIterator<Item = (X, Y)>,
            impl BaseFmt<X = X, Y = Y>,
            impl TickGen<Item = X>,
            impl TickGen<Item = Y>,
        >,
    ) -> fmt::Result {
        //render background
        {
            let mut writer = tagger::new(&mut writer);
            writer.single("circle", |d| {
                d.attr("r", "1e5")?;
                d.attr("class", "poloto_background")
            })?;
        }

        //
        // Using `cargo bloat` determined that these lines reduces alot of code bloat.
        //
        let plot_fmt = data.plots.as_mut_dyn();

        render::render_plot::render_plot(&mut writer, &data.boundx, &data.boundy, self, plot_fmt)?;

        // reduce code bloat
        let xtick_info = &mut data.xtick as &mut dyn ticks::TickGen<Item = X>;

        // reduce code bloat
        let ytick_info = &mut data.ytick as &mut dyn ticks::TickGen<Item = Y>;

        render::render_base::render_base(
            &mut writer,
            &data.boundx,
            &data.boundy,
            &mut data.base,
            xtick_info,
            ytick_info,
            self,
        )
    }
}
///
/// Build a [`Canvas`]
///
pub fn canvas() -> Canvas {
    CanvasBuilder::default().build()
}

pub fn canvas_builder() -> CanvasBuilder {
    CanvasBuilder::default()
}

///
/// Created by [`Canvas::build`]
///
pub struct Data<X: PlotNum, Y: PlotNum, P: PlotIterator, K: Borrow<Canvas>> {
    canvas: K,
    boundx: ticks::DataBound<X>,
    boundy: ticks::DataBound<Y>,
    plots: P,
}

impl<X: PlotNum, Y: PlotNum, P: PlotIterator<Item = (X, Y)>, K: Borrow<Canvas>> Data<X, Y, P, K> {
    pub fn bounds(&self) -> (ticks::Bound<X>, ticks::Bound<Y>) {
        (
            Bound {
                data: &self.boundx,
                canvas: &self.canvas.borrow().boundx,
            },
            Bound {
                data: &self.boundy,
                canvas: &self.canvas.borrow().boundy,
            },
        )
    }

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
        X: HasDefaultTicks,
        Y: HasDefaultTicks,
    {
        let (bx, by) = self.bounds();
        let (x, xt) = ticks::from_default(bx);
        let (y, yt) = ticks::from_default(by);

        let p = plot_fmt(title, xname, yname, xt, yt);
        self.plot_with(x, y, p)
    }

    ///
    /// Move to final stage in pipeline collecting the title/xname/yname.
    /// Unlike [`Data::plot`] User must supply own tick distribution.
    ///
    pub fn plot_with<XI, YI, PF>(self, xtick: XI, ytick: YI, plot_fmt: PF) -> Plotter<impl Disp>
    where
        XI: TickGen<Item = X>,
        YI: TickGen<Item = Y>,
        PF: BaseFmt<X = X, Y = Y>,
    {
        let dim = self.canvas.borrow().get_dim();
        Plotter {
            inner: Some(Disper::new(move |w| {
                self.canvas.borrow().render(
                    w,
                    RenderOptions {
                        base: plot_fmt,
                        plots: self.plots,
                        xtick,
                        ytick,
                        boundx: self.boundx,
                        boundy: self.boundy,
                    },
                )
            })),
            dim,
        }
    }
}
