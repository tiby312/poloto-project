//!
//! Tools to render plots
//!

use super::*;
use crate::build::PlotIterator;
mod render_base;
mod render_plot;

///
/// Specify options for the svg plots
///
#[derive(Clone)]
pub struct RenderOptions {
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    dim: Option<[f64; 2]>,
    xtick_lines: bool,
    ytick_lines: bool,
    precision: usize,
    bar_width: f64,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
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

impl RenderOptions {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_viewbox(&mut self, dim: [f64; 2]) -> &mut Self {
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

    pub fn move_into(&mut self) -> Self {
        self.clone()
    }

    fn compute(&mut self) -> RenderOptionsResult {
        let (width, height) = if let Some([x, y]) = self.dim {
            (x, y)
        } else {
            let [x, y] = Header::new().get_viewbox();
            (x, y)
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

        RenderOptionsResult {
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
#[derive(Clone)]
struct RenderOptionsResult {
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

pub fn render_opt() -> RenderOptions {
    RenderOptions::default()
}

///
/// Link some plots with a way to render them.
///
pub struct Stage1<P: PlotIterator, TX, TY> {
    opt: RenderOptions,
    tickx: TX,
    ticky: TY,
    plots: P,
    boundx: DataBound<P::X>,
    boundy: DataBound<P::Y>,
}

impl<X, Y, P: build::PlotIterator<X = X, Y = Y>> Stage1<P, X::DefaultTicks, Y::DefaultTicks>
where
    X: HasDefaultTicks,
    Y: HasDefaultTicks,
{
    pub fn new(plots: P) -> Self {
        Self::from_parts(
            plots,
            P::X::default_ticks(),
            P::Y::default_ticks(),
            RenderOptions::new(),
        )
    }
}

impl<P: build::PlotIterator, TX: TickDistGen<P::X>, TY: TickDistGen<P::Y>> Stage1<P, TX, TY> {
    pub fn from_parts(mut plots: P, tickx: TX, ticky: TY, opt: RenderOptions) -> Stage1<P, TX, TY> {
        let mut area = build::marker::Area::new();
        plots.increase_area(&mut area);
        let (boundx, boundy) = area.build();

        Stage1 {
            opt,
            plots,
            ticky,
            tickx,
            boundx,
            boundy,
        }
    }

    pub fn with_opt<F: FnOnce(RenderOptions) -> RenderOptions>(self, func: F) -> Self {
        Stage1 {
            opt: func(self.opt),
            tickx: self.tickx,
            ticky: self.ticky,
            plots: self.plots,
            boundx: self.boundx,
            boundy: self.boundy,
        }
    }

    pub fn with_xticks<TTT: TickDistGen<P::X>, F: FnOnce(TX) -> TTT>(
        self,
        func: F,
    ) -> Stage1<P, TTT, TY> {
        let tickx = func(self.tickx);
        Stage1 {
            opt: self.opt,
            tickx,
            ticky: self.ticky,
            plots: self.plots,
            boundx: self.boundx,
            boundy: self.boundy,
        }
    }

    pub fn with_yticks<TTT: TickDistGen<P::Y>, F: FnOnce(TY) -> TTT>(
        self,
        func: F,
    ) -> Stage1<P, TX, TTT> {
        let ticky = func(self.ticky);
        Stage1 {
            opt: self.opt,
            tickx: self.tickx,
            ticky,
            plots: self.plots,
            boundx: self.boundx,
            boundy: self.boundy,
        }
    }

    pub fn build_map<F: FnOnce(Stage2<P, TX::Res, TY::Res>) -> K, K>(self, func: F) -> K {
        let k = self.build();
        func(k)
    }

    pub fn build(self) -> Stage2<P, TX::Res, TY::Res> {
        let mut index_counter = 0;
        let mut data = self;
        let opt = data.opt.compute();

        let xticks = data.tickx.generate(
            &data.boundx,
            &opt.boundx,
            IndexRequester::new(&mut index_counter),
        );
        let yticks = data.ticky.generate(
            &data.boundy,
            &opt.boundy,
            IndexRequester::new(&mut index_counter),
        );
        Stage2 {
            opt,
            xticks,
            yticks,
            boundx: data.boundx,
            boundy: data.boundy,
            plots: data.plots,
        }
    }

    pub fn build_and_label<Fmt: BaseFmt>(self, fmt: Fmt) -> Stage3<P, TX::Res, TY::Res, Fmt> {
        self.build().label(fmt)
    }
}

pub struct Stage2<P: PlotIterator, A, B> {
    opt: RenderOptionsResult,
    xticks: A,
    yticks: B,
    plots: P,
    boundx: DataBound<P::X>,
    boundy: DataBound<P::Y>,
}

impl<P: PlotIterator, A: TickDist<Num = P::X>, B: TickDist<Num = P::Y>> Stage2<P, A, B> {
    pub fn label<Fmt: BaseFmt>(self, fmt: Fmt) -> Stage3<P, A, B, Fmt> {
        Stage3 {
            data: self,
            base: fmt,
        }
    }

    pub fn boundx(&self) -> &DataBound<P::X> {
        &self.boundx
    }

    pub fn boundy(&self) -> &DataBound<P::Y> {
        &self.boundy
    }

    pub fn xticks(&self) -> &A {
        &self.xticks
    }
    pub fn yticks(&self) -> &B {
        &self.yticks
    }

    // pub fn map_xticks<X: TickDist<Num = P::X>, F: FnOnce(A) -> X>(
    //     self,
    //     func: F,
    // ) -> DataBuilt<P, X, B> {
    //     let k = func(self.xticks);
    //     DataBuilt {
    //         opt: self.opt,
    //         xticks: k,
    //         yticks: self.yticks,
    //         plots: self.plots,
    //         boundx: self.boundx,
    //         boundy: self.boundy,
    //     }
    // }

    // pub fn map_yticks<Y: TickDist<Num = P::Y>, F: FnOnce(B) -> Y>(
    //     self,
    //     func: F,
    // ) -> DataBuilt<P, A, Y> {
    //     let k = func(self.yticks);
    //     DataBuilt {
    //         opt: self.opt,
    //         xticks: self.xticks,
    //         yticks: k,
    //         plots: self.plots,
    //         boundx: self.boundx,
    //         boundy: self.boundy,
    //     }
    // }
}

pub struct Stage3<P: PlotIterator, A, B, BB: BaseFmt> {
    data: Stage2<P, A, B>,
    base: BB,
}

impl<P, A, B, BB> Stage3<P, A, B, BB>
where
    P: PlotIterator,
    A: crate::ticks::TickDist<Num = P::X>,
    B: crate::ticks::TickDist<Num = P::Y>,
    BB: BaseFmt,
{
    pub fn append_to<E: Elem>(self, elem: E) -> Stage4<elem::Append<E, Self>> {
        Stage4(elem.append(self))
    }

    pub fn headless(self) -> Stage4<Self> {
        Stage4(self)
    }
}

impl<P, A, B, BB> elem::Elem for Stage3<P, A, B, BB>
where
    P: PlotIterator,
    A: crate::ticks::TickDist<Num = P::X>,
    B: crate::ticks::TickDist<Num = P::Y>,
    BB: BaseFmt,
{
    type Tail = ();
    fn render_head(mut self, writer: &mut elem::ElemWrite) -> Result<Self::Tail, fmt::Error> {
        writer.render(
            hbuild::single("circle").with(attrs!(("r", "1e5"), ("class", "poloto_background"))),
        )?;

        render::render_plot::render_plot(
            writer,
            &self.data.boundx,
            &self.data.boundy,
            &self.data.opt,
            &mut self.data.plots,
        )?;

        render::render_base::render_base(
            writer,
            self.data.xticks,
            self.data.yticks,
            &self.data.boundx,
            &self.data.boundy,
            &mut self.base,
            &self.data.opt,
        )
    }
}

impl<A, B, C> BaseFmt for (A, B, C)
where
    A: Display,
    B: Display,
    C: Display,
{
    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.0)
    }
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.1)
    }
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.2)
    }
}

pub struct Stage4<R: Elem>(R);
impl<R: Elem> Stage4<R> {
    pub fn render_stdout(self) {
        hypermelon::render(self.0, hypermelon::stdout_fmt()).unwrap()
    }

    pub fn render_fmt_write<T: fmt::Write>(self, w: T) -> fmt::Result {
        hypermelon::render(self.0, w)
    }

    pub fn render_io_write<T: std::io::Write>(self, w: T) -> std::fmt::Result {
        hypermelon::render(self.0, hypermelon::tools::upgrade_write(w))
    }

    pub fn render_string(self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        hypermelon::render(self.0, &mut s)?;
        Ok(s)
    }
}
impl<R: Elem> Elem for Stage4<R> {
    type Tail = R::Tail;

    fn render_head(self, w: &mut elem::ElemWrite) -> Result<Self::Tail, fmt::Error> {
        self.0.render_head(w)
    }
}
