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
//! * Create tick distributions. (This step can be done automatically using [`DataResult::plot`])
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

pub mod bar;
pub mod bounded_iter;
pub mod buffered_iter;
mod build;
pub mod canvas;
pub mod plotnum;
mod render;
pub mod util;
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
pub struct DataBound<X> {
    pub min: X,
    pub max: X,
}

pub struct CanvasBound {
    pub ideal_num_steps: u32,
    pub ideal_dash_size: f64,
    pub max: f64,
    pub axis: Axis,
}

pub struct Canvas {
    boundx: CanvasBound,
    boundy: CanvasBound,
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

pub struct Bound<'a, X: PlotNum> {
    pub data: &'a DataBound<X>,
    pub canvas: &'a CanvasBound,
}

///
/// Create a tick distribution from the default tick generator for the plotnum type.
///
pub fn ticks_from_default<X: HasDefaultTicks>(bound: Bound<X>) -> (TickInfo<X::IntoIter>, X::Fmt) {
    X::generate(bound)
}

///
/// Created by [`Data::build`]
///
pub struct DataResult<'a, X: 'a, Y: 'a> {
    plots: Vec<Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>>,
    boundx: DataBound<X>,
    boundy: DataBound<Y>,
}

pub struct Stager<'a, X: 'a, Y: 'a> {
    res: DataResult<'a, X, Y>,
    canvas: Canvas,
}

///
/// Start plotting.
///
pub fn data<'a, X: PlotNum, Y: PlotNum>() -> Data<'a, X, Y> {
    Data::default()
}

pub fn canvas() -> canvas::CanvasBuilder {
    canvas::CanvasBuilder::default()
}

use plotnum::PlotIter;

///
/// Plot collector.
///
pub struct Data<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    plots: Vec<Box<dyn PlotTrait<'a, Item = (X, Y)> + 'a>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
}

///
/// One-time function to write to a `fmt::Write`.
///
pub trait Disp {
    fn disp<T: fmt::Write>(self, writer: T) -> fmt::Result;
}

///
/// Created by [`DataResult::plot`]
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
