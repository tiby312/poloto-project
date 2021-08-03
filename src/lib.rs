//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.
//!
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

pub use crop::Crop;
pub use crop::Croppable;
mod crop;

mod render;
mod util;
use std::fmt;

///The number of unique colors.
const NUM_COLORS: usize = 8;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

trait PlotTrait {
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result;

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
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result {
        write!(a, "{}", self.func)
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
/// The default svg tag
///
/// They are as follows:
/// `class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg"`
///
pub fn default_svg<T: std::fmt::Write, K>(
    e: &mut tagger::ElemWriter<T>,
    extra: impl FnOnce(&mut tagger::AttrWriter<T>),
    func: impl FnOnce(&mut tagger::ElemWriter<T>) -> K,
) -> K {
    e.elem("svg", |d| {
        d.attr("width", DIMENSIONS[0])
            .attr("class", "poloto")
            .attr("height", DIMENSIONS[1])
            .attr(
                "viewBox",
                format_args!("0 0 {} {}", DIMENSIONS[0], DIMENSIONS[1]),
            )
            .attr("xmlns", "http://www.w3.org/2000/svg");
        extra(d);
    })
    .build(|d| func(d))
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

/// The demsions of the svg graph `[800,500]`.
pub const DIMENSIONS: [usize; 2] = [800, 500];

/// Iterators that are passed to the [`Plotter`] plot functions must produce
/// items that implement this trait.
pub trait Plottable {
    /// Produce one plot
    fn make_plot(self) -> [f64; 2];
}

/// Convert other primitive types to f64 using this trait.
/// Precision loss is considered acceptable, since this is just for visual human eyes.
pub trait AsF64: Copy {
    fn as_f64(&self) -> f64;
}

impl<T: AsF64> AsF64 for &T {
    fn as_f64(&self) -> f64 {
        AsF64::as_f64(*self)
    }
}
macro_rules! impl_into_plotnum {
    ($U: ty ) => {
        impl AsF64 for $U {
            fn as_f64(&self) -> f64 {
                *self as f64
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

impl<T: AsF64> Plottable for [T; 2] {
    fn make_plot(self) -> [f64; 2] {
        let [x, y] = self;
        [x.as_f64(), y.as_f64()]
    }
}

impl<T: AsF64> Plottable for &[T; 2] {
    fn make_plot(self) -> [f64; 2] {
        let [x, y] = self;
        [x.as_f64(), y.as_f64()]
    }
}

impl<T: AsF64> Plottable for (T, T) {
    fn make_plot(self) -> [f64; 2] {
        let (x, y) = self;
        [x.as_f64(), y.as_f64()]
    }
}

impl<T: AsF64> Plottable for &(T, T) {
    fn make_plot(self) -> [f64; 2] {
        let (x, y) = self;
        [x.as_f64(), y.as_f64()]
    }
}

///
/// Create a Plotter
///
pub fn plot<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a> {
    Plotter::new(title, xname, yname)
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
    title: Box<dyn fmt::Display + 'a>,
    xname: Box<dyn fmt::Display + 'a>,
    yname: Box<dyn fmt::Display + 'a>,
    plots: Vec<Plot<'a>>,
    xmarkers: Vec<f64>,
    ymarkers: Vec<f64>,
}

impl<'a> Plotter<'a> {
    ///
    /// Create a plotter with the specified element.
    ///
    /// ```
    /// let p = poloto::Plotter::new("title", "x", "y");
    /// ```
    pub fn new(
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<'a> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: Vec::new(),
            xmarkers: Vec::new(),
            ymarkers: Vec::new(),
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

    /// Add x values that the scaled graph must fit.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    ///
    /// // Include origin in the graph.
    /// plotter.xmarker(0).ymarker(0);
    /// ```
    pub fn xmarker<A: AsF64>(&mut self, marker: A) -> &mut Self {
        self.xmarkers.push(marker.as_f64());
        self
    }

    /// Add y values that the scaled graph must fit.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    ///
    /// // Include origin in the graph.
    /// plotter.xmarker(0).ymarker(0);
    /// ```
    pub fn ymarker<A: AsF64>(&mut self, marker: A) -> &mut Self {
        self.ymarkers.push(marker.as_f64());
        self
    }

    ///
    /// Use the plot iterators to write out the graph elements.
    ///
    /// Panics if the render fails.
    ///
    /// In order to meet a more flexible builder pattern, instead of consuming the Plotter,
    /// this function will mutable borrow the Plotter and leave it with empty data.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let mut k=String::new();
    /// plotter.render(tagger::new(&mut k));
    /// ```
    pub fn render<T: std::fmt::Write, K>(&mut self, mut a: K) -> K
    where
        K: std::borrow::BorrowMut<tagger::ElemWriter<T>>,
    {
        render::render(self, a.borrow_mut());
        a
    }

    pub fn simple_theme<T: std::fmt::Write, K>(&mut self, mut a: K) -> K
    where
        K: std::borrow::BorrowMut<tagger::ElemWriter<T>>,
    {
        default_svg(a.borrow_mut(), tagger::no_attr(), |d| {
            d.put_raw(HTML_CONFIG_LIGHT_DEFAULT);
            self.render(d);
        });
        a
    }

    pub fn simple_theme_dark<T: std::fmt::Write, K>(&mut self, mut a: K) -> K
    where
        K: std::borrow::BorrowMut<tagger::ElemWriter<T>>,
    {
        default_svg(a.borrow_mut(), tagger::no_attr(), |d| {
            d.put_raw(HTML_CONFIG_DARK_DEFAULT);
            self.render(d);
        });
        a
    }
}

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::moveable_format(move |w| write!(w,$($arg)*))
    }
}

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
pub fn moveable_format(func: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl fmt::Display {
    struct Foo<F>(F);
    impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Foo<F> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            (self.0)(formatter)
        }
    }
    Foo(func)
}
