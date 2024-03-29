//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/target/assets) folder.
//!
//! Poloto provides by default 2 impls of [`HasDefaultTicks`] for the following types:
//!
//! * [`i128`] - decimal/scientific notation ticks.
//! * [`f64`] - decimal/scientific notation ticks.
//!
//! The above types have the advantage of automatically selecting reasonable
//! tick intervals. The user can change the formatting of the ticks while still using
//! the ticks that were selected.
//!
//! However, sometimes you may want more control on the ticks, or want to use a type
//! other than [`i128`]/[`f64`]. One way would be to write your own function that returns a [`TickDistGen`].
//! Alternatively you can use the [`ticks::from_iter`] function that just takes an iterator of ticks and returns a [`TickDistGen`].
//! This puts more responsibility on the user to pass a decent distribution of ticks. This should only really be used when the user
//! knows up front the min and max values of that axis. This is typically the case for
//! at least one of the axis, typically the x axis. [See step example](https://github.com/tiby312/poloto-project/blob/master/poloto/examples/custom_ticks.rs)

use build::PlotRes;
use build::Point;
use tagu::attr;
use tagu::elem;

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../../README.md"));
}

use std::fmt;

pub mod build;
pub mod plotnum;
pub mod render;
pub mod ticks;
pub mod util;
use plotnum::*;
pub mod num;

use tagu::build as hbuild;
use tagu::prelude::*;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::build::crop::Croppable;
    pub use super::build::output_zip::OutputZip;
    pub use super::build::PlotIterator;
    pub use super::plots;
}

use fmt::Display;

use ticks::*;

use render::*;

mod simple;

///
/// Macro to chain multiple plots together instead of calling chain repeatedly.
///
#[macro_export]
macro_rules! plots {
    ($a:expr)=>{
        $a
    };
    ( $a:expr,$( $x:expr ),* ) => {
        {

            let mut a=$a;
            $(

                let k=$x;
                let a={
                    use $crate::build::PlotIterator;
                    a.chain(k)
                };

            )*
            a
        }
    };
}

pub fn frame_build() -> RenderFrame {
    frame().build()
}

pub fn frame() -> RenderFrameBuilder {
    RenderFrameBuilder::default()
}

///
/// Start plotting!
///
#[deprecated(note = "Use poloto::frame().data()")]
pub fn data<X: PlotNum, Y: PlotNum, L: Point<X = X, Y = Y>, J: build::PlotIterator<L = L>>(
    plots: J,
) -> Stage1<PlotRes<J::P, L>, X::DefaultTicks, Y::DefaultTicks>
where
    X: HasDefaultTicks,
    Y: HasDefaultTicks,
{
    render::Stage1::from_parts(plots, X::default_ticks(), Y::default_ticks(), frame_build())
}

///
/// shorthand for [`Header::new()`]
///
pub fn header() -> Header<()> {
    Header::new()
}
