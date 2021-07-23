//!
//! Plot to SVG and style with CSS
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! * Plots containing NaN or Infinity are ignored.
//! * After 8 plots, the colors cycle back and are repeated.
//!

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

mod render;
mod util;
use std::fmt;
use tagger::prelude::*;

///The number of unique colors.
const NUM_COLORS: usize = 8;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

trait PlotTrait {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result;

    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
}

use fmt::Display;
struct PlotStruct<I: Iterator<Item = [f64; 2]> + Clone, F: Display> {
    first: I,
    second: I,
    func: F,
}

impl<I: Iterator<Item = [f64; 2]> + Clone, F: Display> PlotStruct<I, F> {
    fn new(it: I, func: F) -> Self {
        let it2 = it.clone();
        PlotStruct {
            first: it,
            second: it2,
            func,
        }
    }
}

impl<D: Iterator<Item = [f64; 2]> + Clone, F: Display> PlotTrait for PlotStruct<D, F> {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result {
        self.func.fmt(a)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]> {
        &mut self.first
    }

    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]> {
        &mut self.second
    }
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait + 'a>,
}

///
/// The default svg tag element with the default attributes
///
pub fn default_svg<'a>() -> tagger::Element<'a> {
    elem!("svg", default_svg_attr().build())
}

///
/// The default svg tag attributes
///
/// They are as follows:
/// `class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg"`
///
pub fn default_svg_attr<'a>() -> tagger::AttrBuilder<'a> {
    use tagger::prelude::*;

    let mut k = tagger::attr_builder();
    k.attr("class", "poloto")
        .attr("width", DIMENSIONS[0])
        .attr("height", DIMENSIONS[1])
        .attr(
            "viewBox",
            formatm!("0 0 {} {}", DIMENSIONS[0], DIMENSIONS[1]),
        )
        .attr("xmlns", "http://www.w3.org/2000/svg");
    k
}

/// Default theme using css variables (with light theme defaults if the variables are not set).
pub const HTML_CONFIG_CSS_VARIABLE_DEFAULT: &str = "<style>.poloto {\
    stroke-linecap:round;\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .scatter{stroke-width:7}\
    .poloto_text{fill: var(--poloto_fg_color,black);}\
    .poloto_axis_lines{stroke: var(--poloto_fg_color,black);stoke-width:3;fill:none}\
    .poloto_background{fill: var(--poloto_bg_color,aliceblue);}\
    .poloto0stroke{stroke:  var(--poloto_color0,blue);}\
    .poloto1stroke{stroke:  var(--poloto_color1,red);}\
    .poloto2stroke{stroke:  var(--poloto_color2,green);}\
    .poloto3stroke{stroke:  var(--poloto_color3,gold);}\
    .poloto4stroke{stroke:  var(--poloto_color4,aqua);}\
    .poloto5stroke{stroke:  var(--poloto_color5,brown);}\
    .poloto6stroke{stroke:  var(--poloto_color6,lime);}\
    .poloto7stroke{stroke:  var(--poloto_color7,chocolate);}\
    .poloto0fill{fill:var(--poloto_color0,blue);}\
    .poloto1fill{fill:var(--poloto_color1,red);}\
    .poloto2fill{fill:var(--poloto_color2,green);}\
    .poloto3fill{fill:var(--poloto_color3,gold);}\
    .poloto4fill{fill:var(--poloto_color4,aqua);}\
    .poloto5fill{fill:var(--poloto_color5,brown);}\
    .poloto6fill{fill:var(--poloto_color6,lime);}\
    .poloto7fill{fill:var(--poloto_color7,chocolate);}</style>";

/// Default light theme
pub const HTML_CONFIG_LIGHT_DEFAULT: &str = "<style>.poloto {\
    stroke-linecap:round;\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .scatter{stroke-width:7}\
    .poloto_text{fill: black;}\
    .poloto_axis_lines{stroke: black;stoke-width:3;fill:none}\
    .poloto_background{fill: aliceblue;}\
    .poloto0stroke{stroke:  blue;}\
    .poloto1stroke{stroke:  red;}\
    .poloto2stroke{stroke:  green;}\
    .poloto3stroke{stroke:  gold;}\
    .poloto4stroke{stroke:  aqua;}\
    .poloto5stroke{stroke:  brown;}\
    .poloto6stroke{stroke:  lime;}\
    .poloto7stroke{stroke:  chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:brown;}\
    .poloto6fill{fill:lime;}\
    .poloto7fill{fill:chocolate;}</style>";

/// Default dark theme
pub const HTML_CONFIG_DARK_DEFAULT: &str = "<style>.poloto {\
    stroke-linecap:round;\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .scatter{stroke-width:7}\
    .poloto_text{fill: white;}\
    .poloto_axis_lines{stroke: white;stoke-width:3;fill:none}\
    .poloto_background{fill: black;}\
    .poloto0stroke{stroke:  blue;}\
    .poloto1stroke{stroke:  red;}\
    .poloto2stroke{stroke:  green;}\
    .poloto3stroke{stroke:  gold;}\
    .poloto4stroke{stroke:  aqua;}\
    .poloto5stroke{stroke:  brown;}\
    .poloto6stroke{stroke:  lime;}\
    .poloto7stroke{stroke:  chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:brown;}\
    .poloto6fill{fill:lime;}\
    .poloto7fill{fill:chocolate;}</style>";

const DIMENSIONS: [usize; 2] = [800, 500];

/// Iterators that are passed to the [`Plotter`] plot functions must produce
/// items that implement this trait.
pub trait Plottable {
    /// Produce one plot
    fn make_plot(self) -> [f64; 2];
}

/// Convert other primitive types to f64 using this trait.
/// Precision loss is considered acceptable, since this is just for visual human eyes.
pub trait IntoF64: Copy {
    fn into_f64(self) -> f64;
}

macro_rules! impl_into_plotnum {
    ($U: ty ) => {
        impl IntoF64 for $U {
            fn into_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

impl_into_plotnum!(f32);
impl_into_plotnum!(f64);
impl_into_plotnum!(i8);
impl_into_plotnum!(u8);
impl_into_plotnum!(i16);
impl_into_plotnum!(u16);
impl_into_plotnum!(i32);
impl_into_plotnum!(u32);
impl_into_plotnum!(i64);
impl_into_plotnum!(u64);
impl_into_plotnum!(i128);
impl_into_plotnum!(u128);
impl_into_plotnum!(isize);
impl_into_plotnum!(usize);

impl<T: IntoF64> Plottable for [T; 2] {
    fn make_plot(self) -> [f64; 2] {
        let [x, y] = self;
        [x.into_f64(), y.into_f64()]
    }
}

impl<T: IntoF64> Plottable for &[T; 2] {
    fn make_plot(self) -> [f64; 2] {
        let [x, y] = self;
        [x.into_f64(), y.into_f64()]
    }
}

impl<T: IntoF64> Plottable for (T, T) {
    fn make_plot(self) -> [f64; 2] {
        let (x, y) = self;
        [x.into_f64(), y.into_f64()]
    }
}

impl<T: IntoF64> Plottable for &(T, T) {
    fn make_plot(self) -> [f64; 2] {
        let (x, y) = self;
        [x.into_f64(), y.into_f64()]
    }
}

///
/// Create a Plotter with preset styling and svg tag.
///
/// Shorthand for
/// ```
/// use tagger::prelude::*;
/// let p =poloto::Plotter::new(poloto::default_svg().add(single!(poloto::HTML_CONFIG_LIGHT_DEFAULT)),"title","xname","yname");
/// ```
///
pub fn plot<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a> {
    Plotter::new(
        default_svg().add(single!(HTML_CONFIG_LIGHT_DEFAULT)),
        title,
        xname,
        yname,
    )
}

/// Keeps track of plots.
/// User supplies iterators that will be iterated on when
/// render is called.
///
/// * The svg element belongs to the `poloto` css class.
/// * The title,xname,yname,legend text SVG elements belong to the `poloto_text` class.
/// * The axis line SVG elements belong to the `poloto_axis_lines` class.
/// * The background belongs to the `poloto_background` class.
///
pub struct Plotter<'a> {
    element: tagger::Element<'a>,
    title: Box<dyn fmt::Display + 'a>,
    xname: Box<dyn fmt::Display + 'a>,
    yname: Box<dyn fmt::Display + 'a>,
    plots: Vec<Plot<'a>>,
}

impl<'a> Plotter<'a> {
    ///
    /// Create a plotter with the specified element.
    ///
    /// ```
    /// let svg = poloto::default_svg();
    /// let p = poloto::Plotter::new(svg, "title", "x", "y");
    /// ```
    pub fn new(
        element: tagger::Element<'a>,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<'a> {
        Plotter {
            element,
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: Vec::new(),
        }
    }
    /// Create a line from plots using a SVG polyline element.
    /// The element belongs to the `.poloto[N]stroke` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// ```
    pub fn line<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line_fill("", &data);
    /// ```
    pub fn line_fill<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.scatter("", &data);
    /// ```
    pub fn scatter<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.histogram("", &data);
    /// ```
    pub fn histogram<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    ///
    /// Use the plot iterators and generate out a [`tagger::Element`] which implements [`std::fmt::Display`]
    ///
    /// Panics if the render fails.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// println!("{}",plotter.render());
    /// ```
    pub fn render(&mut self) -> tagger::Element<'a> {
        render::render(self).unwrap()
    }

    ///
    /// Use the plot iterators and generate out a [`tagger::Element`] which implements [`std::fmt::Display`]
    ///
    /// Returns a fmt::Error if the render fails.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let s = plotter.try_render().unwrap();
    /// println!("{}",s);
    /// ```
    pub fn try_render(&mut self) -> Result<tagger::Element<'a>, fmt::Error> {
        render::render(self)
    }
}
