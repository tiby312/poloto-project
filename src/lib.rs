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
//! * Create a RenderOptions using [`render`] module.
//! * Compute min/max by calling [`data()`].
//! * Create tick distributions. (This step can be done automatically using [`quick_fmt!`])
//! * Collect title/xname/yname using [`plot_with()`] (done automatically using [`quick_fmt!`])
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
mod simple_theme;

use hypermelon::build as hbuild;
use hypermelon::prelude::*;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::build::crop::Croppable;
    pub use super::build::output_zip::OutputZip;
    pub use super::plots;
}

use fmt::Display;

use ticks::*;

// ///The width of the svg tag.
// const WIDTH: f64 = 800.0;
// ///The height of the svg tag.
// const HEIGHT: f64 = 500.0;

use render::*;

pub fn simple_light() -> hypermelon::Append<Header, Theme<'static>> {
    Header::new().append(Theme::light())
}

pub fn simple_dark() -> hypermelon::Append<Header, Theme<'static>> {
    Header::new().append(Theme::dark())
}

///
/// Macro to chain multiple plots together instead of calling [`chain`](build::PlotIteratorExt::chain) repeatedly.
///
#[macro_export]
macro_rules! plots {
    ($a:expr)=>{
        $a
    };
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
/// Construct a [`Data`].
///
pub fn data<
    X: PlotNum + HasDefaultTicks,
    Y: PlotNum + HasDefaultTicks,
    P: build::PlotIterator<X = X, Y = Y>,
>(
    plots: P,
) -> Stage1<P, X::DefaultTicks, Y::DefaultTicks> {
    render::Stage1::from_parts(
        plots,
        X::default_ticks(),
        Y::default_ticks(),
        render_opt_builder(),
    )
}

// ///
// /// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
// ///
// pub fn disp<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(
//     a: F,
// ) -> util::DisplayableClosureOnce<F> {
//     util::DisplayableClosureOnce::new(a)
// }

// ///
// /// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
// ///
// pub fn disp_mut<F: FnMut(&mut fmt::Formatter) -> fmt::Result>(
//     a: F,
// ) -> util::DisplayableClosureMut<F> {
//     util::DisplayableClosureMut::new(a)
// }
