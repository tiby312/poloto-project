//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/target/assets) folder.
//!
//!
//!
//! Pipeline:
//! * Collect plots ([`data`] function)
//! * Compute min/max (call [`Data::build`] and generate a [`DataResult`]).
//! * Create tick distributions. (This step can be done automatically using [`DataResult::plot`] instead of [`DataResult::plot_with`])
//! * Collect title/xname/yname
//! * Write everything to svg. [`Plotter::render`] for no svg tag/css. [`simple_theme::SimpleTheme`] for basic css/svg tag.
//!
//! Poloto provides by default 3 impls of [`HasDefaultTicks`] for the following types:
//!
//! * [`i128`] - decimal/scientific notation ticks.
//! * [`f64`] - decimal/scientific notation ticks.
//! * [`UnixTime`](num::timestamp::UnixTime) - date/time
//!
//! The above types have the advantage of automatically selecting reasonable
//! tick intervals. The user can change the formatting of the ticks while still using
//! the ticks that were selected via its automatic methods using [`TickFormatExt::with_tick_fmt`].
//!
//! However, sometimes you may want more control on the ticks, or want to use a type
//! other than [`i128`]/[`f64`]/[`UnixTime`](num::timestamp::UnixTime). One way would be to write your own function that returns a [`TickInfo`].
//! Alternatively you can use the [`ticks_from_iter`] function that just takes an iterator of ticks and returns a [`TickInfo`].
//! This puts more responsibility on the user to pass a decent number of ticks. This should only really be used when the user
//! knows up front the min and max values of that axis. This is typically the case for
//! at least one of the axis, typically the x axis. [See marathon example](https://github.com/tiby312/poloto/blob/master/examples/marathon.rs)

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

use std::fmt;

pub use tagger::upgrade_write;

pub mod plottable;
use plottable::Plottable;

mod render;
pub mod util;

pub mod bounded_iter;
pub mod buffered_iter;

pub mod plotnum;
use plotnum::*;
pub mod num;
pub mod simple_theme;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::formatm;
    pub use super::plotnum::TickFormatExt;
    pub use super::plottable::crop::Croppable;
    pub use super::simple_theme::SimpleTheme;
}

use fmt::Display;
use std::marker::PhantomData;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

use render::*;

trait PlotTrait<'a> {
    type Item;
    fn plot_type(&self) -> PlotMetaType;
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result;
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = Self::Item>;
    fn iter_second(&mut self) -> Box<dyn Iterator<Item = Self::Item> + 'a>;
}

struct PlotStruct<I: PlotIter, F: Display> {
    ptype: PlotMetaType,
    iter: Option<I>,
    it1: Option<I::It1>,
    func: F,
}

impl<I: PlotIter, F: Display> PlotStruct<I, F> {
    fn new(iter: I, func: F, ptype: PlotMetaType) -> Self {
        PlotStruct {
            iter: Some(iter),
            it1: None,
            func,
            ptype,
        }
    }
}

impl<'a, X, Y, D: PlotIter<Item1 = (X, Y), Item2 = (X, Y)> + 'a, F: Display> PlotTrait<'a>
    for PlotStruct<D, F>
{
    type Item = (X, Y);
    fn plot_type(&self) -> PlotMetaType {
        self.ptype
    }
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result {
        write!(a, "{}", self.func)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = Self::Item> {
        self.it1 = Some(self.iter.as_mut().unwrap().first());
        self.it1.as_mut().unwrap()
    }

    fn iter_second(&mut self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
        Box::new(self.iter.take().unwrap().second(self.it1.take().unwrap()))
    }
}

///
/// Created once the min and max bounds of all the plots has been computed.
/// Contains in it all the information typically needed to make a [`TickInfo`].
///
///
#[derive(Debug, Clone)]
pub struct Bound<X> {
    pub min: X,
    pub max: X,
    pub ideal_num_steps: u32,
    pub dash_info: DashInfo,
    pub axis: Axis,
}

///
/// Create a tick distribution from the default tick generator for the plotnum type.
///
pub fn ticks_from_default<X: HasDefaultTicks>(bound: &Bound<X>) -> (TickInfo<X::IntoIter>, X::Fmt) {
    X::generate(bound)
}

///
/// Created by [`Data::build`]
///
pub struct DataResult<'a, X: 'a, Y: 'a> {
    plots: Vec<Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>>,
    canvas: render::Canvas,
    boundx: Bound<X>,
    boundy: Bound<Y>,
    xtick_lines: bool,
    ytick_lines: bool,
    precision: usize,
}
impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> DataResult<'a, X, Y> {
    pub fn boundx(&self) -> &Bound<X> {
        &self.boundx
    }
    pub fn boundy(&self) -> &Bound<Y> {
        &self.boundy
    }

    ///
    /// Automatically create a tick distribution using the default
    /// tick generators tied to a [`PlotNum`].
    ///
    pub fn plot(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<impl Disp + 'a>
    where
        X: HasDefaultTicks,
        Y: HasDefaultTicks,
    {
        let (x, xt) = ticks_from_default(&self.boundx);
        let (y, yt) = ticks_from_default(&self.boundy);

        let p = plot_fmt(title, xname, yname, xt, yt);
        self.plot_with(x, y, p)
    }

    ///
    /// Move to final stage in pipeline collecting the title/xname/yname.
    /// Unlike [`DataResult::plot`] User must supply own tick distribution.
    ///
    pub fn plot_with<XI: 'a, YI: 'a, PF: 'a>(
        self,
        xtick: TickInfo<XI>,
        ytick: TickInfo<YI>,
        plot_fmt: PF,
    ) -> Plotter<impl Disp + 'a>
    where
        XI: IntoIterator<Item = X>,
        YI: IntoIterator<Item = Y>,
        PF: BaseFmt<X = X, Y = Y>,
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
    fn plot_with_all<PF: BaseFmtAndTicks<X = X, Y = Y> + 'a>(
        self,
        p: PF,
    ) -> Plotter<impl Disp + 'a> {
        struct Foo2<'a, X, Y> {
            plots: Vec<Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>>,
        }

        struct One<'a, X, Y> {
            one: Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>,
        }
        impl<'a, X, Y> OnePlotFmt for One<'a, X, Y> {
            type It = Box<dyn Iterator<Item = Self::Item> + 'a>;
            type Item = (X, Y);
            fn plot_type(&mut self) -> PlotMetaType {
                self.one.plot_type()
            }

            fn fmt(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
                self.one.write_name(writer)
            }

            fn get_iter(&mut self) -> Box<dyn Iterator<Item = Self::Item> + 'a> {
                self.one.iter_second()
            }
        }

        impl<'a, X: 'a, Y: 'a> AllPlotFmt for Foo2<'a, X, Y> {
            type Item2 = (X, Y);
            type It = Box<dyn Iterator<Item = One<'a, X, Y>> + 'a>;
            type InnerIt = One<'a, X, Y>;
            fn iter(self) -> Self::It {
                Box::new(self.plots.into_iter().map(|one| One { one }))
            }
        }

        struct Combine<A: BaseFmtAndTicks, B: AllPlotFmt> {
            pub a: A,
            pub b: B,
        }

        impl<A: BaseFmtAndTicks, B: AllPlotFmt<Item2 = (A::X, A::Y)>> BaseAndPlotsFmt for Combine<A, B> {
            type X = A::X;
            type Y = A::Y;
            type A = A;
            type B = B;
            fn gen(self) -> (Self::A, Self::B) {
                (self.a, self.b)
            }
        }

        struct InnerPlotter<PF: BaseAndPlotsFmt> {
            all: PF,
            extra: Extra<PF::X, PF::Y>,
        }

        impl<PF: BaseAndPlotsFmt> Disp for InnerPlotter<PF> {
            fn disp<T: std::fmt::Write>(self, mut writer: T) -> fmt::Result {
                render::render(&mut writer, self.all, &self.extra)
            }
        }

        let pp = InnerPlotter {
            all: Combine {
                a: p,
                b: Foo2 { plots: self.plots },
            },
            extra: Extra {
                precision: self.precision,
                canvas: self.canvas,
                boundx: self.boundx,
                boundy: self.boundy,
                xtick_lines: self.xtick_lines,
                ytick_lines: self.ytick_lines,
            },
        };

        let dim = pp.extra.canvas.get_dim();
        Plotter {
            inner: Some(pp),
            dim,
        }
    }
}

///
/// Create a plot formatter that implements [`plotnum::BaseFmt`]
///
pub fn plot_fmt<D, E>(
    title: impl Display,
    xname: impl Display,
    yname: impl Display,
    tickx: D,
    ticky: E,
) -> impl BaseFmt<X = D::Num, Y = E::Num>
where
    D: TickFormat,
    E: TickFormat,
{
    ///
    /// A simple plot formatter that is composed of
    /// display objects as TickFormats.
    ///
    struct SimplePlotFormatter<A, B, C, D, E> {
        title: A,
        xname: B,
        yname: C,
        tickx: D,
        ticky: E,
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
        fn write_xwher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
            self.tickx.write_where(writer)
        }
        fn write_ywher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
            self.ticky.write_where(writer)
        }
    }

    SimplePlotFormatter {
        title,
        xname,
        yname,
        tickx,
        ticky,
    }
}

///
/// Start plotting.
///
pub fn data<'a, X: PlotNum, Y: PlotNum>() -> Data<'a, X, Y> {
    Data::default()
}

pub mod bar {
    use super::*;
    pub struct Blop<'a, D> {
        ticks: &'a [D],
    }

    impl<'a, D: Display> TickFormat for Blop<'a, D> {
        type Num = i128;
        fn write_tick(&mut self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> fmt::Result {
            let j = &self.ticks[*val as usize];
            write!(writer, "{}", j)
        }
    }

    pub fn gen_bar<'a, D: Display>(
        _bound: &Bound<i128>,
        vals: &'a [D],
    ) -> (TickInfo<Vec<i128>>, Blop<'a, D>) {
        //assert!(vals.len()>0);
        //assert_eq!(bound.max-bound.min,(vals.len()-1) as i128);
        let ticks = (0..vals.len()).map(|x| x as i128).collect();

        let b = Blop { ticks: vals };
        (
            TickInfo {
                ticks,
                dash_size: None,
            },
            b,
        )
    }
}

use plotnum::PlotIter;

///
/// Plot collector.
///
pub struct Data<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    plots: Vec<Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    dim: Option<[f64; 2]>,
    xtick_lines: bool,
    ytick_lines: bool,
    precision: usize,
}
impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> Default for Data<'a, X, Y> {
    fn default() -> Self {
        Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
            num_css_classes: Some(8),
            preserve_aspect: false,
            dim: None,
            xtick_lines: false,
            ytick_lines: false,
            precision: 2,
        }
    }
}
impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> Data<'a, X, Y> {
    pub fn with_dim(&mut self, dim: [f64; 2]) -> &mut Self {
        self.dim = Some(dim);
        self
    }
    pub fn xmarker(&mut self, a: X) -> &mut Self {
        self.xmarkers.push(a);
        self
    }

    pub fn ymarker(&mut self, a: Y) -> &mut Self {
        self.ymarkers.push(a);
        self
    }

    pub fn xtick_lines(&mut self) -> &mut Self {
        self.xtick_lines = true;
        self
    }
    pub fn ytick_lines(&mut self) -> &mut Self {
        self.ytick_lines = true;
        self
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    /// ```
    /// let mut plotter = poloto::data::<f64,f64>();
    /// plotter.text("This is a note");
    /// ```
    pub fn text(&mut self, name: impl Display + 'a) -> &mut Self {
        self.plots.push(Box::new(PlotStruct::new(
            std::iter::empty(),
            name,
            PlotMetaType::Text,
        )));
        self
    }

    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// ```
    pub fn line<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Line),
        )));
        self
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line_fill("", &data);
    /// ```
    pub fn line_fill<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::LineFill),
        )));
        self
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line_fill_raw("", &data);
    /// ```
    pub fn line_fill_raw<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::LineFillRaw),
        )));
        self
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.scatter("", &data);
    /// ```
    pub fn scatter<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Scatter),
        )));
        self
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.histogram("", &data);
    /// ```
    pub fn histogram<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Histo),
        )));
        self
    }

    pub fn bars<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: PlotIter + 'a,
        I::Item1: Plottable<Item = (X, Y)>,
        I::Item2: Plottable<Item = (X, Y)>,
    {
        self.plots.push(Box::new(PlotStruct::new(
            plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
            name,
            PlotMetaType::Plot(PlotType::Bars),
        )));
        self
    }

    ///
    /// Preserve the aspect ratio by drawing a smaller graph in the same area.
    ///
    pub fn preserve_aspect(&mut self) -> &mut Self {
        self.preserve_aspect = true;
        self
    }

    ///
    /// The number of distinct css classes. If there are more plots than
    /// classes, then they will wrap around. The default value is 8.
    ///
    /// A value of None, means it will never wrap around.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// plotter.num_css_class(Some(30));
    /// ```
    ///
    pub fn num_css_class(&mut self, a: Option<usize>) -> &mut Self {
        self.num_css_classes = a;
        self
    }

    pub fn move_into(&mut self) -> Self {
        let mut val = Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
            num_css_classes: None,
            preserve_aspect: false,
            dim: None,
            xtick_lines: false,
            ytick_lines: false,
            precision: 0,
        };

        std::mem::swap(&mut val, self);
        val
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
    /// Compute min/max bounds and prepare for next stage in pipeline.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::data();
    /// plotter.line("", &data);
    /// plotter.build();
    /// ```
    ///
    pub fn build(&mut self) -> DataResult<'a, X, Y> {
        let mut val = self.move_into();

        let (boundx, boundy) = util::find_bounds(
            val.plots.iter_mut().flat_map(|x| x.iter_first()),
            val.xmarkers.clone(),
            val.ymarkers.clone(),
        );

        let canvas = render::Canvas::with_options(
            boundx,
            boundy,
            val.dim,
            val.preserve_aspect,
            val.num_css_classes,
        );

        let ideal_dash_size = canvas.ideal_dash_size;
        let boundx = Bound {
            min: boundx[0],
            max: boundx[1],
            ideal_num_steps: canvas.ideal_num_xsteps,
            dash_info: DashInfo {
                ideal_dash_size,
                max: canvas.scalex,
            },
            axis: Axis::X,
        };
        let boundy = Bound {
            min: boundy[0],
            max: boundy[1],
            ideal_num_steps: canvas.ideal_num_ysteps,
            dash_info: DashInfo {
                ideal_dash_size,
                max: canvas.scaley,
            },
            axis: Axis::Y,
        };

        DataResult {
            precision: val.precision,
            plots: val.plots,
            canvas,
            boundx,
            boundy,
            xtick_lines: val.xtick_lines,
            ytick_lines: val.ytick_lines,
        }
    }
}

///
/// One-time function to write to a `fmt::Write`.
///
pub trait Disp {
    fn disp<T: fmt::Write>(self, writer: T) -> fmt::Result;
}

///
/// Created by [`DataResult::plot`] or [`DataResult::plot_with`]
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
    /// let mut s = poloto::data();
    /// s.line("", &data);
    /// let mut plotter=s.build().plot("title","x","y");
    ///
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```

    pub fn render<T: std::fmt::Write>(&mut self, writer: T) -> fmt::Result {
        self.inner.take().unwrap().disp(writer)
    }
}

/// Shorthand for `disp_const(move |w|write!(w,...))`
/// Similar to `std::format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::disp_const(move |w| write!(w,$($arg)*))
    }
}

///
/// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
///
pub fn disp<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(
    a: F,
) -> util::DisplayableClosureOnce<F> {
    util::DisplayableClosureOnce::new(a)
}

///
/// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
///
pub fn disp_mut<F: FnMut(&mut fmt::Formatter) -> fmt::Result>(
    a: F,
) -> util::DisplayableClosureMut<F> {
    util::DisplayableClosureMut::new(a)
}

///
/// Convert a closure to a object that implements Display
///
pub fn disp_const<F: Fn(&mut fmt::Formatter) -> fmt::Result>(a: F) -> util::DisplayableClosure<F> {
    util::DisplayableClosure::new(a)
}

///
/// Create a [`plotnum::TickInfo`] from a step iterator.
///
#[deprecated(note = "Use ticks_from_iter() instead.")]
pub fn steps<X: PlotNum + Display, I: Iterator<Item = X>>(
    _bound: &Bound<X>,
    ticks: I,
) -> (TickInfo<I>, TickIterFmt<X>) {
    (
        TickInfo {
            ticks,
            dash_size: None,
        },
        TickIterFmt { _p: PhantomData },
    )
}

///
/// Create a [`plotnum::TickInfo`] from a step iterator.
///
///
pub fn ticks_from_iter<X: PlotNum + Display, I: Iterator<Item = X>>(
    ticks: I,
) -> (TickInfo<I>, TickIterFmt<X>) {
    (
        TickInfo {
            ticks,
            dash_size: None,
        },
        TickIterFmt { _p: PhantomData },
    )
}

#[deprecated(note = "Use TickIterFmt instead.")]
pub type StepFmt<T> = TickIterFmt<T>;

///
/// Used by [`ticks_from_iter`]
///
pub struct TickIterFmt<T> {
    _p: PhantomData<T>,
}
impl<J: PlotNum + Display> TickFormat for TickIterFmt<J> {
    type Num = J;
    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        write!(writer, "{}", val)
    }
}
