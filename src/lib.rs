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

//pub use crop::Crop;
//pub use crop::Croppable;
mod crop;

mod render;

pub use util::interval_float as default_val_formatter;
mod util;

use std::fmt;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;




trait PlotTrait<X:PlotNumber,Y:PlotNumber> {
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result;

    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X,Y)>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X,Y)>;
}

use std::marker::PhantomData;

use fmt::Display;
struct PlotStruct<X:PlotNumber,Y:PlotNumber,I: Iterator<Item = (X,Y)> + Clone, F: Display> {
    first: I,
    second: I,
    func: F,
    _p:PhantomData<(X,Y)>
}

impl<X:PlotNumber,Y:PlotNumber,I: Iterator<Item = (X,Y)> + Clone, F: Display> PlotStruct<X,Y,I, F> {
    fn new(it: I, func: F) -> Self {
        let it2 = it.clone();
        PlotStruct {
            first: it,
            second: it2,
            func,
            _p:PhantomData
        }
    }
}

impl<X:PlotNumber,Y:PlotNumber,D: Iterator<Item = (X,Y)> + Clone, F: Display> PlotTrait<X,Y> for PlotStruct<X,Y,D, F> {
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result {
        write!(a, "{}", self.func)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X,Y)> {
        &mut self.first
    }

    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X,Y)> {
        &mut self.second
    }
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a,X:PlotNumber,Y:PlotNumber> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<X,Y> + 'a>,
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

/// Default light theme
pub const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto { \
    stroke-linecap:round; \
    stroke-linejoin:round; \
    font-family: 'Tahoma', sans-serif; \
    stroke-width:2; \
    } \
    .scatter{stroke-width:7} \
    .poloto_text{fill: black;} \
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none} \
    .poloto_background{background-color: AliceBlue;} \
    .poloto0stroke{stroke:  blue;} \
    .poloto1stroke{stroke:  red;} \
    .poloto2stroke{stroke:  green;} \
    .poloto3stroke{stroke:  gold;} \
    .poloto4stroke{stroke:  aqua;} \
    .poloto5stroke{stroke:  lime;} \
    .poloto6stroke{stroke:  orange;} \
    .poloto7stroke{stroke:  chocolate;} \
    .poloto0fill{fill:blue;} \
    .poloto1fill{fill:red;} \
    .poloto2fill{fill:green;} \
    .poloto3fill{fill:gold;} \
    .poloto4fill{fill:aqua;} \
    .poloto5fill{fill:lime;} \
    .poloto6fill{fill:orange;} \
    .poloto7fill{fill:chocolate;}";

/// Default dark theme
pub const STYLE_CONFIG_DARK_DEFAULT: &str = ".poloto { \
    stroke-linecap:round; \
    stroke-linejoin:round; \
    font-family: 'Tahoma', sans-serif; \
    stroke-width:2; \
    } \
    .scatter{stroke-width:7} \
    .poloto_text{fill: white;} \
    .poloto_axis_lines{stroke: white;stroke-width:3;fill:none;stroke-dasharray:none} \
    .poloto_background{background-color: #262626;} \
    .poloto0stroke{stroke:  blue;} \
    .poloto1stroke{stroke:  red;} \
    .poloto2stroke{stroke:  green;} \
    .poloto3stroke{stroke:  gold;} \
    .poloto4stroke{stroke:  aqua;} \
    .poloto5stroke{stroke:  lime;} \
    .poloto6stroke{stroke:  orange;} \
    .poloto7stroke{stroke:  chocolate;} \
    .poloto0fill{fill:blue;} \
    .poloto1fill{fill:red;} \
    .poloto2fill{fill:green;} \
    .poloto3fill{fill:gold;} \
    .poloto4fill{fill:aqua;} \
    .poloto5fill{fill:lime;} \
    .poloto6fill{fill:orange;} \
    .poloto7fill{fill:chocolate;}";

/// The demsions of the svg graph `[800,500]`.
pub const DIMENSIONS: [usize; 2] = [800, 500];

/// Iterators that are passed to the [`Plotter`] plot functions must produce
/// items that implement this trait.
pub trait Plottable<X:PlotNumber,Y:PlotNumber> {
    /// Produce one plot
    fn make_plot(self) -> (X,Y);
}

impl<T:PlotNumber> Plottable<T,T> for [T; 2] {
    fn make_plot(self) -> (T,T) {
        let [x, y] = self;
        (x,y)
    }
}

impl<T:PlotNumber> Plottable<T,T> for &[T; 2] {
    fn make_plot(self) -> (T,T) {
        let [x, y] = *self;
        (x,y)
    }
}



impl<A: PlotNumber, B: PlotNumber> Plottable<A,B> for (A, B) {
    fn make_plot(self) -> (A,B) {
        self
    }
}

impl<A: PlotNumber, B: PlotNumber> Plottable<A,B> for &(A, B) {
    fn make_plot(self) -> (A,B) {
        *self
    }
}


/*
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

impl<A: AsF64, B: AsF64> Plottable for (A, B) {
    fn make_plot(self) -> [f64; 2] {
        let (x, y) = self;
        [x.as_f64(), y.as_f64()]
    }
}

impl<A: AsF64, B: AsF64> Plottable for &(A, B) {
    fn make_plot(self) -> [f64; 2] {
        let (x, y) = self;
        [x.as_f64(), y.as_f64()]
    }
}
*/

pub use util::PlotNumber;
///
/// Create a Plotter
///
pub fn plot<'a,X:PlotNumber,Y:PlotNumber>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a,X,Y> {
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
pub struct Plotter<'a,X:PlotNumber+'a,Y:PlotNumber+'a> {
    title: Box<dyn fmt::Display + 'a>,
    xname: Box<dyn fmt::Display + 'a>,
    yname: Box<dyn fmt::Display + 'a>,
    plots: Vec<Plot<'a,X,Y>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
}

impl<'a,X:PlotNumber,Y:PlotNumber> Plotter<'a,X,Y> {
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
    ) -> Plotter<'a,X,Y> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: Vec::new(),
            xmarkers: Vec::new(),
            ymarkers: Vec::new(),
            num_css_classes: Some(8),
            preserve_aspect: false
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
        I::Item: Plottable<X,Y>,
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
        I::Item: Plottable<X,Y>,
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
        I::Item: Plottable<X,Y>,
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
        I::Item: Plottable<X,Y>,
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
    pub fn xmarker(&mut self, marker: X) -> &mut Self {
        self.xmarkers.push(marker);
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
    pub fn ymarker(&mut self, marker: Y) -> &mut Self {
        self.ymarkers.push(marker);
        self
    }

    ///
    /// Preserve the aspect ratio by drawing a smaller graph in the same area.
    ///
    pub fn preserve_aspect(&mut self) -> &mut Self {
        self.preserve_aspect = true;
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
    /// Move a plotter out from behind a mutable reference leaving
    /// an empty plotter.
    ///
    pub fn move_into(&mut self) -> Plotter<'a,X,Y> {
        let mut empty = crate::Plotter::new("", "", "");
        core::mem::swap(&mut empty, self);
        empty
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
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```
    pub fn render<T: std::fmt::Write>(&mut self, a: T) -> T {
        render::render(self, a)
    }

    ///
    /// Make a graph with a svg tag and a simple css theme.
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
            d.put_raw(format_args!(
                "<style>{}</style>",
                STYLE_CONFIG_LIGHT_DEFAULT
            ));
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
            d.put_raw(format_args!("<style>{}</style>", STYLE_CONFIG_DARK_DEFAULT));
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

pub struct Renderer {}
impl Renderer {
    pub fn render<T: fmt::Write>(&mut self, a: T) -> T {
        let mut w = tagger::new(a);
        default_svg(&mut w, tagger::no_attr(), |d| {
            d.put_raw(format_args!("<style>{}</style>", STYLE_CONFIG_DARK_DEFAULT));
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

/*
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
*/
