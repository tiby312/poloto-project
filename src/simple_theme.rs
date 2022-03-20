//!
//! A simple dark and light css theme.
//!
//!
use super::*;

///
/// Default SVG Header for a Poloto graph.
///
pub const SVG_HEADER: &str = r##"<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"##;

///
/// The default svg dimentions for simple theme.
///
pub const DIM: [f64; 2] = [800.0, 500.0];

///
/// Default SVG end tag.
///
pub const SVG_END: &str = "</svg>";

/// Default light theme
pub const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto{\
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

/// Default dark theme
pub const STYLE_CONFIG_DARK_DEFAULT: &str = ".poloto{\
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

///
/// Allow passing Option<f64> or f64 to [`write_header`]
///
pub trait IntoOpt: Copy {
    fn create(self) -> Option<f64>;
}
impl IntoOpt for Option<f64> {
    fn create(self) -> Option<f64> {
        self
    }
}

impl IntoOpt for f64 {
    fn create(self) -> Option<f64> {
        Some(self)
    }
}

///
/// Write the svg header with the specified width and viewport.
///
pub fn write_header<T: std::fmt::Write>(
    mut w: T,
    dim: [f64; 2],
    viewbox: [f64; 2],
) -> std::fmt::Result {
    write!(w, "<svg class=\"poloto\" ")?;
    write!(w, "width=\"{}\" ", dim[0])?;
    write!(w, "height=\"{}\" ", dim[1])?;
    write!(
        w,
        "viewBox=\"0 0 {} {}\" xmlns=\"http://www.w3.org/2000/svg\">",
        viewbox[0], viewbox[1]
    )
}

///
/// Create a simple theme.
///
pub trait SimpleTheme {
    fn simple_theme<T: fmt::Write>(self, a: T) -> std::fmt::Result;
    fn simple_theme_dark<T: fmt::Write>(self, a: T) -> std::fmt::Result;
}

impl<P: build::PlotIterator<X = B::X, Y = B::Y>, K: Borrow<Canvas>, B: BaseFmt> SimpleTheme
    for Plotter<P, K, B>
{
    fn simple_theme<T: fmt::Write>(self, mut a: T) -> std::fmt::Result {
        let dim = self.get_dim();

        write!(
            a,
            "{}<style>{}</style>{}{}",
            disp_const(|w| write_header(w, dim, dim)),
            STYLE_CONFIG_LIGHT_DEFAULT,
            disp(|a| self.render(a)),
            SVG_END
        )
    }

    fn simple_theme_dark<T: fmt::Write>(self, mut a: T) -> std::fmt::Result {
        let dim = self.get_dim();

        write!(
            a,
            "{}<style>{}</style>{}{}",
            disp_const(|w| write_header(w, dim, dim)),
            STYLE_CONFIG_DARK_DEFAULT,
            disp(|a| self.render(a)),
            SVG_END
        )
    }
}

///
/// Based on a svg viewport and a desired width, determine the height.
///
pub fn determine_height_from_width(viewport: [f64; 2], width: f64) -> f64 {
    let [xx, yy] = viewport;
    width * (yy / xx)
}



