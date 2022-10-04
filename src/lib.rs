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

///
/// Default svg header
///
#[derive(Copy, Clone)]
pub struct Header {
    dim: [f64; 2],
    viewbox: [f64; 2],
}
impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Header {
    pub fn new() -> Self {
        let a = [800.0, 500.0];
        Header { dim: a, viewbox: a }
    }

    pub fn with_viewbox_width(self, width: f64) -> Self {
        let [xx, yy] = self.dim;
        let vh = width * (yy / xx);
        Header {
            dim: self.dim,
            viewbox: [width, vh],
        }
    }

    pub fn get_viewbox(&self) -> [f64; 2] {
        self.viewbox
    }

    pub fn with_dim(self, dim: [f64; 2]) -> Self {
        Header {
            dim,
            viewbox: self.viewbox,
        }
    }
    pub fn with_viewbox(self, viewbox: [f64; 2]) -> Self {
        Header {
            dim: self.dim,
            viewbox,
        }
    }

    pub fn to_string(self) -> (String, String) {
        let mut s = String::new();
        let tail = self
            .render_head(&mut hypermelon::ElemWrite::new(&mut s))
            .unwrap();

        use hypermelon::RenderTail;
        let mut b = String::new();
        tail.render(&mut hypermelon::ElemWrite::new(&mut b))
            .unwrap();

        (s, b)
    }

    pub fn light_theme(self) -> hypermelon::Append<Self, Theme<'static>> {
        self.append(Theme::light())
    }
    pub fn dark_theme(self) -> hypermelon::Append<Self, Theme<'static>> {
        self.append(Theme::light())
    }
}

impl Elem for Header {
    type Tail = hypermelon::build::ElemTail<&'static str>;
    fn render_head(self, w: &mut hypermelon::ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let elem = hypermelon::build::elem("svg").with(attrs!(
            ("class", "poloto"),
            ("width", self.dim[0]),
            ("height", self.dim[1]),
            (
                "viewBox",
                format_move!("{} {} {} {}", 0, 0, self.viewbox[0], self.viewbox[1])
            ),
            ("xmlns", "http://www.w3.org/2000/svg")
        ));

        elem.render_head(w)
    }
}

pub fn header() -> Header {
    Header::new()
}

#[derive(Copy, Clone)]
pub struct Theme<'a> {
    styles: &'a str,
}

impl Theme<'static> {
    pub const fn light() -> Theme<'static> {
        /// Default light theme

        const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto{\
            stroke-linecap:round;\
            stroke-linejoin:round;\
            font-family:Roboto,sans-serif;\
            font-size:16px;\
            }\
            .poloto_background{fill:AliceBlue;}\
            .poloto_scatter{stroke-width:7}\
            .poloto_tick_line{stroke:gray;stroke-width:0.5}\
            .poloto_line{stroke-width:2}\
            .poloto_text{fill: black;}\
            .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}\
            .poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
            .poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
            .poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
            .poloto_legend_text{font-size:20px;dominant-baseline:middle;text-anchor:start;}\
            .poloto0stroke{stroke:blue;}\
            .poloto1stroke{stroke:red;}\
            .poloto2stroke{stroke:green;}\
            .poloto3stroke{stroke:gold;}\
            .poloto4stroke{stroke:aqua;}\
            .poloto5stroke{stroke:lime;}\
            .poloto6stroke{stroke:orange;}\
            .poloto7stroke{stroke:chocolate;}\
            .poloto0fill{fill:blue;}\
            .poloto1fill{fill:red;}\
            .poloto2fill{fill:green;}\
            .poloto3fill{fill:gold;}\
            .poloto4fill{fill:aqua;}\
            .poloto5fill{fill:lime;}\
            .poloto6fill{fill:orange;}\
            .poloto7fill{fill:chocolate;}";

        Theme {
            styles: STYLE_CONFIG_LIGHT_DEFAULT,
        }
    }
    pub const fn dark() -> Theme<'static> {
        /// Default dark theme
        const STYLE_CONFIG_DARK_DEFAULT: &str = ".poloto{\
    stroke-linecap:round;\
    stroke-linejoin:round;\
    font-family:Roboto,sans-serif;\
    font-size:16px;\
    }\
    .poloto_background{fill:#262626;}\
    .poloto_scatter{stroke-width:7}\
    .poloto_tick_line{stroke:dimgray;stroke-width:0.5}\
    .poloto_line{stroke-width:2}\
    .poloto_text{fill: white;}\
    .poloto_axis_lines{stroke: white;stroke-width:3;fill:none;stroke-dasharray:none}\
    .poloto_title{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_xname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto_yname{font-size:24px;dominant-baseline:start;text-anchor:middle;}\
    .poloto0stroke{stroke:blue;}\
    .poloto1stroke{stroke:red;}\
    .poloto2stroke{stroke:green;}\
    .poloto3stroke{stroke:gold;}\
    .poloto4stroke{stroke:aqua;}\
    .poloto5stroke{stroke:lime;}\
    .poloto6stroke{stroke:orange;}\
    .poloto7stroke{stroke:chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:lime;}\
    .poloto6fill{fill:orange;}\
    .poloto7fill{fill:chocolate;}";
        Theme {
            styles: STYLE_CONFIG_DARK_DEFAULT,
        }
    }

    pub const fn get_str(&self) -> &'static str {
        self.styles
    }
}

impl<'a> Elem for Theme<'a> {
    type Tail = hypermelon::build::ElemTail<&'static str>;
    fn render_head(self, w: &mut hypermelon::ElemWrite) -> Result<Self::Tail, fmt::Error> {
        let k = hypermelon::build::elem("style");
        let k = k.append(self.styles);
        k.render_head(w)
    }
}
