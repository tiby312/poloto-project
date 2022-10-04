//!
//! Tools to render plots
//!

use super::*;
use crate::build::PlotIterator;
mod render_base;
mod render_plot;

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
        disp_const(move |f| write!(f, "{:.*}", precision, num))
    }
}

pub struct LineFill<I> {
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

pub struct Line<I> {
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

///
/// Build a [`RenderOptions`]
///
/// Created by [`render_opt_builder()`]
///
#[derive(Clone)]
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
    pub fn new() -> Self {
        Self::default()
    }
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

    pub fn move_into(&mut self) -> Self {
        self.clone()
    }

    fn compute(&mut self) -> RenderOptions {
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

pub fn render_opt_builder() -> RenderOptionsBuilder {
    RenderOptionsBuilder::default()
}

///
/// Link some plots with a way to render them.
///
pub struct Data<P: PlotIterator, TX, TY> {
    opt: RenderOptionsBuilder,
    tickx: TX,
    ticky: TY,
    plots: P,
    boundx: DataBound<P::X>,
    boundy: DataBound<P::Y>,
}

impl<X, Y, P: build::PlotIterator<X = X, Y = Y>> Data<P, X::DefaultTicks, Y::DefaultTicks>
where
    X: HasDefaultTicks,
    Y: HasDefaultTicks,
{
    pub fn new(plots: P) -> Self {
        Self::from_parts(
            plots,
            P::X::default_ticks(),
            P::Y::default_ticks(),
            RenderOptionsBuilder::new(),
        )
    }
}

impl<P: build::PlotIterator, TX: GenTickDist<P::X>, TY: GenTickDist<P::Y>> Data<P, TX, TY> {
    pub fn from_parts(
        mut plots: P,
        tickx: TX,
        ticky: TY,
        opt: RenderOptionsBuilder,
    ) -> Data<P, TX, TY> {
        let mut area = build::marker::Area::new();
        plots.increase_area(&mut area);
        let (boundx, boundy) = area.build();

        Data {
            opt,
            plots,
            ticky,
            tickx,
            boundx,
            boundy,
        }
    }

    pub fn map_opt<F: FnOnce(RenderOptionsBuilder) -> RenderOptionsBuilder>(self, func: F) -> Self {
        Data {
            opt: func(self.opt),
            tickx: self.tickx,
            ticky: self.ticky,
            plots: self.plots,
            boundx: self.boundx,
            boundy: self.boundy,
        }
    }

    pub fn map_xticks<TTT: GenTickDist<P::X>, F: FnOnce(TX) -> TTT>(
        self,
        func: F,
    ) -> Data<P, TTT, TY> {
        let tickx = func(self.tickx);
        Data {
            opt: self.opt,
            tickx,
            ticky: self.ticky,
            plots: self.plots,
            boundx: self.boundx,
            boundy: self.boundy,
        }
    }

    pub fn map_yticks<TTT: GenTickDist<P::Y>, F: FnOnce(TY) -> TTT>(
        self,
        func: F,
    ) -> Data<P, TX, TTT> {
        let ticky = func(self.ticky);
        Data {
            opt: self.opt,
            tickx: self.tickx,
            ticky,
            plots: self.plots,
            boundx: self.boundx,
            boundy: self.boundy,
        }
    }

    pub fn build_map<F: FnOnce(DataBuilt<P, TX::Res, TY::Res>) -> K, K>(self, func: F) -> K {
        let k = self.build();
        func(k)
    }

    pub fn build(self) -> DataBuilt<P, TX::Res, TY::Res> {
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
        DataBuilt {
            opt,
            xticks,
            yticks,
            boundx: data.boundx,
            boundy: data.boundy,
            plots: data.plots,
        }
    }

    pub fn build_and_label<Fmt: BaseFmt>(self, fmt: Fmt) -> Plotter<P, TX::Res, TY::Res, Fmt> {
        self.build().label(fmt)
    }
}

pub struct DataBuilt<P: PlotIterator, A, B> {
    opt: RenderOptions,
    xticks: A,
    yticks: B,
    plots: P,
    boundx: DataBound<P::X>,
    boundy: DataBound<P::Y>,
}

impl<P: PlotIterator, A: TickDist<Num = P::X>, B: TickDist<Num = P::Y>> DataBuilt<P, A, B> {
    pub fn label<Fmt: BaseFmt>(self, fmt: Fmt) -> Plotter<P, A, B, Fmt> {
        Plotter {
            data: self,
            base: fmt,
        }
    }

    pub fn opt(&self) -> &RenderOptions {
        &self.opt
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

///
/// Created by [`plot_with`]
///
pub struct Plotter<P: PlotIterator, A, B, BB: BaseFmt> {
    data: DataBuilt<P, A, B>,
    base: BB,
}

impl<P, A, B, BB> Plotter<P, A, B, BB>
where
    P: PlotIterator,
    A: crate::ticks::TickDist<Num = P::X>,
    B: crate::ticks::TickDist<Num = P::Y>,
    BB: BaseFmt,
{
    pub fn append_to<E: Elem>(self, elem: E) -> Themer<hypermelon::Append<E, Self>> {
        Themer(elem.append(self))
    }

    pub fn headless(self) -> Themer<Self> {
        Themer(self)
    }
}

impl<P, A, B, BB> hypermelon::Elem for Plotter<P, A, B, BB>
where
    P: PlotIterator,
    A: crate::ticks::TickDist<Num = P::X>,
    B: crate::ticks::TickDist<Num = P::Y>,
    BB: BaseFmt,
{
    type Tail = ();
    fn render_head(mut self, writer: &mut hypermelon::ElemWrite) -> Result<Self::Tail, fmt::Error> {
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

pub struct Themer<R: Elem>(R);
impl<R: Elem> Themer<R> {
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
impl<R: Elem> Elem for Themer<R> {
    type Tail = R::Tail;

    fn render_head(self, w: &mut hypermelon::ElemWrite) -> Result<Self::Tail, fmt::Error> {
        self.0.render_head(w)
    }
}
