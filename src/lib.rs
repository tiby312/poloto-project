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
//! * Collect plots using functions in [`build`] module
//! * Create a canvas using [`render`] module.
//! * Compute min/max by calling [`render::Canvas::build()`].
//! * Create tick distributions. (This step can be done automatically using [`Data::plot()`])
//! * Collect title/xname/yname using [`Data::plot()`] or [`Data::plot_with()`]
//! * Write everything to svg. [`Plotter::render()`] for no svg tag/css. [`simple_theme::SimpleTheme`] for basic css/svg tag.
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
//! other than [`i128`]/[`f64`]/[`UnixTime`](num::timestamp::UnixTime). One way would be to write your own function that returns a [`TickFormat`].
//! Alternatively you can use the [`ticks::from_iter`] function that just takes an iterator of ticks and returns a [`TickFormat`].
//! This puts more responsibility on the user to pass a decent number of ticks. This should only really be used when the user
//! knows up front the min and max values of that axis. This is typically the case for
//! at least one of the axis, typically the x axis. [See step example](https://github.com/tiby312/poloto/blob/master/examples/steps.rs)

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

pub mod build;
pub mod plotnum;
pub mod render;
pub mod ticks;
pub mod util;
use plotnum::*;
pub mod num;
pub mod simple_theme;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::build::crop::Croppable;
    pub use super::build::Markerable;
    pub use super::build::PlotIteratorExt;
    pub use super::formatm;
    pub use super::plots;
    pub use super::simple_fmt;
    pub use super::simple_theme::SimpleTheme;
    pub use super::ticks::TickFormatExt;
}

use fmt::Display;

use ticks::*;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

use render::*;

use build::PlotIterator;

use std::borrow::Borrow;

/// Shorthand for `disp_const(move |w|write!(w,...))`
/// Similar to `std::format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::disp_const(move |w| write!(w,$($arg)*))
    }
}

///
/// Macro to chain multiple plots together instead of caling [`chain`](build::PlotIteratorExt::chain) repeatedly.
///
#[macro_export]
macro_rules! plots {
    ( $a:expr,$( $x:expr ),* ) => {
        {
            use $crate::build::PlotIteratorExt;
            let mut a=$a;
            $(
                let a=a.chain($x);
            )*
            a
        }
    };
}

///
/// Create plots without having to manually create the ticks
/// for each axis.
///
/// ### No canvas argument.
/// ```
/// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
/// let plotter=poloto::simple_fmt!(poloto::build::line("",data),"title","x","y");
/// let mut k=String::new();
/// plotter.render(&mut k);
/// ```
///
/// ### With canvas argument.
/// ```
/// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
/// let canvas=poloto::render::canvas();
/// let plotter=poloto::simple_fmt!(canvas,poloto::build::line("",data),"title","x","y");
/// let mut k=String::new();
/// plotter.render(&mut k);
/// ```
///
#[macro_export]
macro_rules! simple_fmt {
    ($data:expr,$title:expr,$xname:expr,$yname:expr) => {{
        let canvas = $crate::render::canvas_builder().build();
        let data = canvas.build_moved($data);
        let (bx, by) = data.bounds();
        let xt = $crate::ticks::from_default(bx);
        let yt = $crate::ticks::from_default(by);
        data.plot_with($crate::plot_fmt($title, $xname, $yname, xt, yt))
    }};
    ($canvas:expr,$data:expr,$title:expr,$xname:expr,$yname:expr) => {{
        let data = $canvas.build($data);
        let (bx, by) = data.bounds();
        let xt = $crate::ticks::from_default(bx);
        let yt = $crate::ticks::from_default(by);
        data.plot_with($crate::plot_fmt($title, $xname, $yname, xt, yt))
    }};
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
/// Iterate over the specified range over num iterations.
///
pub fn range_iter(
    range: [f64; 2],
    num: usize,
) -> impl ExactSizeIterator<Item = f64> + Clone + Send + Sync + std::iter::FusedIterator {
    let [min, max] = range;
    let diff = max - min;
    let divf = num as f64;
    (0..num).map(move |x| min + (x as f64 / divf) * diff)
}

///
/// Create a plot formatter that implements [`plotnum::BaseFmt`]
///
pub fn plot_fmt<A: Display, B: Display, C: Display, D, E>(
    title: A,
    xname: B,
    yname: C,
    tickx: D,
    ticky: E,
) -> SimplePlotFormatter<A, B, C, D, E>
where
    D: TickFormat,
    E: TickFormat,
{
    SimplePlotFormatter {
        title,
        xname,
        yname,
        tickx,
        ticky,
    }
}
