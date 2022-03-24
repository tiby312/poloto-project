//!
//! Tools to render plots
//!

use crate::*;
mod render_base;
mod render_plot;

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
/// Build a [`RenderOptions`]
///
/// Created by [`render_opt_builder()`]
///
pub struct RenderOptionsBuilder {
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    dim: Option<[f64; 2]>,
    xtick_lines: bool,
    ytick_lines: bool,
    precision: usize,
    bar_width: f64,
}

impl Default for RenderOptionsBuilder {
    fn default() -> Self {
        RenderOptionsBuilder {
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

impl RenderOptionsBuilder {
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

    pub fn build(&mut self) -> RenderOptions {
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

        RenderOptions {
            boundx: ticks::RenderOptionsBound {
                ideal_num_steps: ideal_num_xsteps,
                ideal_dash_size,
                max: scalex,
                axis: Axis::X,
            },
            boundy: ticks::RenderOptionsBound {
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
///
#[derive(Clone)]
pub struct RenderOptions {
    boundx: ticks::RenderOptionsBound,
    boundy: ticks::RenderOptionsBound,
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

impl RenderOptions {
    pub fn get_dim(&self) -> [f64; 2] {
        [self.width, self.height]
    }
}

impl<T: Renderable> Renderable for &T {
    fn get_dim(&self) -> [f64; 2] {
        (*self).get_dim()
    }
    fn bounds(&self) -> (&RenderOptionsBound, &RenderOptionsBound) {
        (*self).bounds()
    }
    fn render<X: PlotNum, Y: PlotNum>(
        &self,
        writer: &mut dyn fmt::Write,
        plots: &mut impl build::PlotIterator<Item = (X, Y)>,
        base: &mut dyn BaseFmt<X = X, Y = Y>,
        boundx: &DataBound<X>,
        boundy: &DataBound<Y>,
    ) -> fmt::Result {
        (*self).render(writer, plots, base, boundx, boundy)
    }
}

impl Renderable for RenderOptions {
    fn bounds(&self) -> (&RenderOptionsBound, &RenderOptionsBound) {
        (&self.boundx, &self.boundy)
    }
    fn get_dim(&self) -> [f64; 2] {
        [self.width, self.height]
    }
    fn render<X: PlotNum, Y: PlotNum>(
        &self,
        mut writer: &mut dyn fmt::Write,
        plots: &mut impl build::PlotIterator<Item = (X, Y)>,
        base: &mut dyn BaseFmt<X = X, Y = Y>,
        boundx: &DataBound<X>,
        boundy: &DataBound<Y>,
    ) -> fmt::Result {
        //render background
        {
            let mut writer = tagger::new(&mut writer);
            writer.single("circle", |d| {
                d.attr("r", "1e5")?;
                d.attr("class", "poloto_background")
            })?;
        }

        render::render_plot::render_plot(&mut writer, boundx, boundy, self, plots)?;

        render::render_base::render_base(&mut writer, boundx, boundy, base, self)
    }
}
pub trait Renderable {
    fn get_dim(&self) -> [f64; 2];
    fn bounds(&self) -> (&RenderOptionsBound, &RenderOptionsBound);
    fn render<X: PlotNum, Y: PlotNum>(
        &self,
        writer: &mut dyn fmt::Write,
        plots: &mut impl build::PlotIterator<Item = (X, Y)>,
        base: &mut dyn BaseFmt<X = X, Y = Y>,
        boundx: &DataBound<X>,
        boundy: &DataBound<Y>,
    ) -> fmt::Result;
}

///
/// Build a [`RenderOptions`]
///
#[deprecated(note = "Renamed Canvas to RenderOptions. Use render_opt() instead.")]
pub fn canvas() -> RenderOptions {
    RenderOptionsBuilder::default().build()
}

///
/// Build a [`RenderOptions`]
///
pub fn render_opt() -> RenderOptions {
    RenderOptionsBuilder::default().build()
}

pub fn render_opt_builder() -> RenderOptionsBuilder {
    RenderOptionsBuilder::default()
}

#[deprecated(note = "Renamed Canvas to RenderOptions. Use render_opt_builder() instead.")]
pub fn canvas_builder() -> RenderOptionsBuilder {
    RenderOptionsBuilder::default()
}

///
/// Linke some plots with a way to render them.
///
pub struct Data<P: build::PlotIteratorAndMarkers> {
    boundx: ticks::DataBound<P::X>,
    boundy: ticks::DataBound<P::Y>,
    plots: P,
}

impl<P: build::PlotIteratorAndMarkers> Data<P> {
    pub fn new(mut plots: P) -> Data<P> {
        let mut area = build::marker::Area::new();
        plots.increase_area(&mut area);
        let (boundx, boundy) = area.build();

        Data {
            boundx,
            boundy,
            plots,
        }
    }

    pub fn bounds(&self) -> (&ticks::DataBound<P::X>, &ticks::DataBound<P::Y>) {
        (&self.boundx, &self.boundy)
    }

    ///
    /// Move to final stage in pipeline collecting the title/xname/yname.
    ///
    #[deprecated(note = "Use poloto::plot_with()")]
    pub fn plot_with<A: BaseFmt<X = P::X, Y = P::Y>, K: Renderable>(
        self,
        canvas: K,
        plot_fmt: A,
    ) -> Plotter<P, K, A> {
        Plotter {
            plots: self.plots,
            base: plot_fmt,
            boundx: self.boundx,
            boundy: self.boundy,
            canvas,
        }
    }
}

pub fn plot_with<
    P: build::PlotIteratorAndMarkers,
    K: Renderable,
    A: BaseFmt<X = P::X, Y = P::Y>,
>(
    data: Data<P>,
    canvas: K,
    base: A,
) -> Plotter<P, K, A> {
    Plotter {
        plots: data.plots,
        base,
        boundx: data.boundx,
        boundy: data.boundy,
        canvas,
    }
}

///
/// Created by [`Data::plot_with`]
///
pub struct Plotter<P: build::PlotIterator<Item = (B::X, B::Y)>, K: Renderable, B: BaseFmt> {
    canvas: K,
    boundx: DataBound<B::X>,
    boundy: DataBound<B::Y>,
    plots: P,
    base: B,
}

impl<P: build::PlotIterator<Item = (B::X, B::Y)>, K: Renderable, B: BaseFmt> Plotter<P, K, B> {
    pub fn get_dim(&self) -> [f64; 2] {
        self.canvas.get_dim()
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
    /// let plotter=poloto::simple_fmt!(poloto::build::line("",data),"title","x","y");
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```
    pub fn render<T: std::fmt::Write>(mut self, mut writer: T) -> fmt::Result {
        self.canvas.render(
            &mut writer,
            &mut self.plots,
            &mut self.base,
            &self.boundx,
            &self.boundy,
        )
    }
}

///
/// A simple plot formatter that is composed of
/// display objects as TickFormats.
///
pub struct SimplePlotFormatter<A, B, C, D, E> {
    pub(crate) title: A,
    pub(crate) xname: B,
    pub(crate) yname: C,
    pub(crate) tickx: D,
    pub(crate) ticky: E,
}
impl<A, B, C, D, E> BaseFmt for SimplePlotFormatter<A, B, C, D, E>
where
    A: Display,
    B: Display,
    C: Display,
    D: TickFormat,
    E: TickFormat,
{
    type X = D::Num;
    type Y = E::Num;
    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.title)
    }
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.xname)
    }
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.yname)
    }
    fn write_xtick(&mut self, writer: &mut dyn fmt::Write, val: &Self::X) -> fmt::Result {
        self.tickx.write_tick(writer, val)
    }
    fn write_ytick(&mut self, writer: &mut dyn fmt::Write, val: &Self::Y) -> fmt::Result {
        self.ticky.write_tick(writer, val)
    }
    fn write_xwher(
        &mut self,
        writer: &mut dyn fmt::Write,
        ind: ticks::IndexRequester,
    ) -> fmt::Result {
        self.tickx.write_where(writer, ind)
    }
    fn write_ywher(
        &mut self,
        writer: &mut dyn fmt::Write,
        ind: ticks::IndexRequester,
    ) -> fmt::Result {
        self.ticky.write_where(writer, ind)
    }

    fn next_xtick(&mut self) -> Option<Self::X> {
        self.tickx.next_tick()
    }

    fn next_ytick(&mut self) -> Option<Self::Y> {
        self.ticky.next_tick()
    }

    fn xdash_size(&self) -> Option<f64> {
        self.tickx.dash_size()
    }

    fn ydash_size(&self) -> Option<f64> {
        self.ticky.dash_size()
    }
}
