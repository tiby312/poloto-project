//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/target/assets) folder.
//!
//! Poloto provides by default 3 impls of [`HasDefaultTicks`] for the following types:
//!
//! * [`i128`] - decimal/scientific notation ticks.
//! * [`f64`] - decimal/scientific notation ticks.
//! * [`UnixTime`](num::timestamp::UnixTime) - date/time
//!
//! The above types have the advantage of automatically selecting reasonable
//! tick intervals. The user can change the formatting of the ticks while still using
//! the ticks that were selected.
//!
//! However, sometimes you may want more control on the ticks, or want to use a type
//! other than [`i128`]/[`f64`]/[`UnixTime`](num::timestamp::UnixTime). One way would be to write your own function that returns a [`TickDistGen`].
//! Alternatively you can use the [`ticks::from_iter`] function that just takes an iterator of ticks and returns a [`TickDistGen`].
//! This puts more responsibility on the user to pass a decent distribution of ticks. This should only really be used when the user
//! knows up front the min and max values of that axis. This is typically the case for
//! at least one of the axis, typically the x axis. [See step example](https://github.com/tiby312/poloto/blob/master/examples/custom_ticks.rs)

use hypermelon::attr;
use hypermelon::elem;

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

pub mod build;
pub mod plotnum;
pub mod render;
pub mod ticks;
pub mod util;
use plotnum::*;
pub mod num;

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

use render::*;

mod simple;

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
            use $crate::build::IntoPlotIterator;
            let mut a=$a.create();
            $(
                let a=a.chain($x.create());
            )*
            a
        }
    };
}

///
/// Start plotting!
///
pub fn data<
    X: PlotNum + HasDefaultTicks,
    Y: PlotNum + HasDefaultTicks,
    P: build::PlotIterator<X = X, Y = Y>,
    J: build::IntoPlotIterator<P = P>,
>(
    plots: J,
) -> Stage1<P, X::DefaultTicks, Y::DefaultTicks> {
    render::Stage1::from_parts(
        plots.create(),
        X::default_ticks(),
        Y::default_ticks(),
        render_opt(),
    )
}

///
/// shorthand for [`Header::new()`]
///
pub fn header() -> Header<()> {
    Header::new()
}
