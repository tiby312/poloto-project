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

pub use tagger::upgrade_write;

pub use crop::Crop;
pub use crop::Croppable;
mod crop;

mod render;
mod util;
use std::fmt;

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
            .attr("class", "poloto_background poloto")
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
pub const HTML_CONFIG_CSS_VARIABLE_DEFAULT: &str = r###"<style>.poloto {
    stroke-linecap:round;
    stroke-linejoin:round;
    font-family: 'Tahoma', sans-serif;
    stroke-width:2;
    }
    .scatter{stroke-width:7}
    .poloto_text{fill: var(--poloto_fg_color,black);}
    .poloto_axis_lines{stroke: var(--poloto_fg_color,black);stroke-width:3;fill:none;stroke-dasharray:none}
    .poloto_background{background-color: var(--poloto_bg_color,AliceBlue);}
    .poloto0stroke{stroke:  var(--poloto_color0,blue);}
    .poloto1stroke{stroke:  var(--poloto_color1,red);}
    .poloto2stroke{stroke:  var(--poloto_color2,green);}
    .poloto3stroke{stroke:  var(--poloto_color3,gold);}
    .poloto4stroke{stroke:  var(--poloto_color4,aqua);}
    .poloto5stroke{stroke:  var(--poloto_color5,lime);}
    .poloto6stroke{stroke:  var(--poloto_color6,orange);}
    .poloto7stroke{stroke:  var(--poloto_color7,chocolate);}
    .poloto0fill{fill:var(--poloto_color0,blue);}
    .poloto1fill{fill:var(--poloto_color1,red);}
    .poloto2fill{fill:var(--poloto_color2,green);}
    .poloto3fill{fill:var(--poloto_color3,gold);}
    .poloto4fill{fill:var(--poloto_color4,aqua);}
    .poloto5fill{fill:var(--poloto_color5,lime);}
    .poloto6fill{fill:var(--poloto_color6,orange);}
    .poloto7fill{fill:var(--poloto_color7,chocolate);}</style>"###;

/// Default light theme
pub const HTML_CONFIG_LIGHT_DEFAULT: &str = r###"<style>.poloto {
    stroke-linecap:round;
    stroke-linejoin:round;
    font-family: 'Tahoma', sans-serif;
    stroke-width:2;
    }
    .scatter{stroke-width:7}
    .poloto_text{fill: black;}
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}
    .poloto_background{background-color: AliceBlue;}
    .poloto0stroke{stroke:  blue;}
    .poloto1stroke{stroke:  red;}
    .poloto2stroke{stroke:  green;}
    .poloto3stroke{stroke:  gold;}
    .poloto4stroke{stroke:  aqua;}
    .poloto5stroke{stroke:  lime;}
    .poloto6stroke{stroke:  orange;}
    .poloto7stroke{stroke:  chocolate;}
    .poloto0fill{fill:blue;}
    .poloto1fill{fill:red;}
    .poloto2fill{fill:green;}
    .poloto3fill{fill:gold;}
    .poloto4fill{fill:aqua;}
    .poloto5fill{fill:lime;}
    .poloto6fill{fill:orange;}
    .poloto7fill{fill:chocolate;}</style>"###;

/// Default dark theme
pub const HTML_CONFIG_DARK_DEFAULT: &str = r###"<style>.poloto {
    stroke-linecap:round;
    stroke-linejoin:round;
    font-family: 'Tahoma', sans-serif;
    stroke-width:2;
    }
    .scatter{stroke-width:7}
    .poloto_text{fill: white;}
    .poloto_axis_lines{stroke: white;stroke-width:3;fill:none;stroke-dasharray:none}
    .poloto_background{background-color: #262626;}
    .poloto0stroke{stroke:  blue;}
    .poloto1stroke{stroke:  red;}
    .poloto2stroke{stroke:  green;}
    .poloto3stroke{stroke:  gold;}
    .poloto4stroke{stroke:  aqua;}
    .poloto5stroke{stroke:  lime;}
    .poloto6stroke{stroke:  orange;}
    .poloto7stroke{stroke:  chocolate;}
    .poloto0fill{fill:blue;}
    .poloto1fill{fill:red;}
    .poloto2fill{fill:green;}
    .poloto3fill{fill:gold;}
    .poloto4fill{fill:aqua;}
    .poloto5fill{fill:lime;}
    .poloto6fill{fill:orange;}
    .poloto7fill{fill:chocolate;}</style>"###;

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
    num_css_classes: Option<usize>,
    preserve_aspect:bool
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
            num_css_classes: Some(8),
            preserve_aspect:false
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
    /// Preserve the aspect ratio by drawing a smaller graph in the same area.
    /// 
    pub fn preserve_aspect(&mut self)->&mut Self{
        self.preserve_aspect=true;
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
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// plotter.num_css_class(Some(30));
    /// ```
    ///
    pub fn num_css_class(&mut self, a: Option<usize>) -> &mut Self {
        self.num_css_classes = a;
        self
    }



    ///
    /// Use the plot iterators to write out the graph elements.
    /// Does not add a svg tag, or any styling elements.
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
    /// plotter.render(&mut k);
    /// ```
    pub fn render<T: std::fmt::Write>(&mut self, a: T) -> T {
        render::render(self, a)
    }

    ///
    /// Make a graph with a svg tag and a simple dark css theme.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let mut k=String::new();
    /// plotter.simple_theme(&mut k);
    /// ```
    pub fn simple_theme<T: std::fmt::Write>(&mut self, a: T) -> T {
        let mut w = tagger::new(a);
        default_svg(&mut w, tagger::no_attr(), |d| {
            d.put_raw(minify(HTML_CONFIG_LIGHT_DEFAULT));
            self.render(d.writer());
        });
        w.into_writer()
    }

    ///
    /// Make a graph with a svg tag and a simple dark css theme.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let mut k=String::new();
    /// plotter.simple_theme_dark(&mut k);
    /// ```
    pub fn simple_theme_dark<T: std::fmt::Write>(&mut self, a: T) -> T {
        let mut w = tagger::new(a);
        default_svg(&mut w, tagger::no_attr(), |d| {
            d.put_raw(minify(HTML_CONFIG_DARK_DEFAULT));
            self.render(d.writer());
        });
        w.into_writer()
    }

    pub fn simple_with_element<T: std::fmt::Write, B: Display>(&mut self, a: T, b: B) -> T {
        let mut w = tagger::new(a);
        default_svg(&mut w, tagger::no_attr(), |d| {
            d.put_raw(b);
            self.render(d.writer());
        });
        w.into_writer()
    }

}

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::DisplayableClosure::new(move |w| write!(w,$($arg)*))
    }
}

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
pub struct DisplayableClosure<F>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> DisplayableClosure<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosure(a)
    }
}
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for DisplayableClosure<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(formatter)
    }
}

///
/// Remove all whitespace/newlines from a tring.
///
pub fn minify(a: &str) -> impl fmt::Display + '_ + Send + Sync {
    DisplayableClosure::new(move |w| {
        for a in a.split_whitespace() {
            write!(w, "{}", a)?;
        }
        Ok(())
    })
}
