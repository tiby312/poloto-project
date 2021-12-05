//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.
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

use std::fmt;

pub use tagger::upgrade_write;

pub use crop::Crop;
pub use crop::Croppable;
mod crop;

mod render;

pub mod util;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

trait PlotTrait<X: PlotNum, Y: PlotNum> {
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result;

    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
}

use std::marker::PhantomData;

use fmt::Display;
struct PlotStruct<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)> + Clone, F: Display> {
    first: I,
    second: I,
    func: F,
    _p: PhantomData<(X, Y)>,
}

impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)> + Clone, F: Display>
    PlotStruct<X, Y, I, F>
{
    fn new(it: I, func: F) -> Self {
        let it2 = it.clone();
        PlotStruct {
            first: it,
            second: it2,
            func,
            _p: PhantomData,
        }
    }
}

impl<X: PlotNum, Y: PlotNum, D: Iterator<Item = (X, Y)> + Clone, F: Display> PlotTrait<X, Y>
    for PlotStruct<X, Y, D, F>
{
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result {
        write!(a, "{}", self.func)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X, Y)> {
        &mut self.first
    }

    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X, Y)> {
        &mut self.second
    }
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a, X: PlotNum, Y: PlotNum> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<X, Y> + 'a>,
}

///
/// Default SVG Header for a Poloto graph.
///
pub const SVG_HEADER: &str = r##"<svg class="poloto_background poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"##;

///
/// Default SVG end tag.
///
pub const SVG_END: &str = "</svg>";

/// Default light theme
pub const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto { \
    stroke-linecap:round; \
    stroke-linejoin:round; \
    font-family: 'Tahoma', sans-serif; \
    stroke-width:2; \
    } \
    .poloto_scatter{stroke-width:7} \
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
    .poloto_scatter{stroke-width:7} \
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

/*
/// The demsions of the svg graph `[800,500]`.
pub const DIMENSIONS: [usize; 2] = [800, 500];
*/

/// Iterators that are passed to the [`Plotter`] plot functions must produce
/// items that implement this trait.
pub trait Plottable<X: PlotNum, Y: PlotNum> {
    /// Produce one plot
    fn make_plot(self) -> (X, Y);
}

impl<T: PlotNum> Plottable<T, T> for [T; 2] {
    fn make_plot(self) -> (T, T) {
        let [x, y] = self;
        (x, y)
    }
}

impl<T: PlotNum> Plottable<T, T> for &[T; 2] {
    fn make_plot(self) -> (T, T) {
        let [x, y] = *self;
        (x, y)
    }
}

impl<A: PlotNum, B: PlotNum> Plottable<A, B> for (A, B) {
    fn make_plot(self) -> (A, B) {
        self
    }
}

impl<A: PlotNum, B: PlotNum> Plottable<A, B> for &(A, B) {
    fn make_plot(self) -> (A, B) {
        *self
    }
}

///
/// Create a Plotter
///
pub fn plot<'a, X: PlotNum, Y: PlotNum>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a, X, Y> {
    Plotter::new(title, xname, yname)
}

trait MyFmt<X: PlotNum> {
    fn write(
        &self,
        formatter: &mut std::fmt::Formatter,
        value: X,
        step: Option<X>,
    ) -> std::fmt::Result;
}

struct Foo<A, X>(A, PhantomData<X>);
impl<X: PlotNum, A: Fn(&mut std::fmt::Formatter, X, Option<X>) -> std::fmt::Result> Foo<A, X> {
    fn new(a: A) -> Foo<A, X> {
        Foo(a, PhantomData)
    }
}
impl<X: PlotNum, A: Fn(&mut std::fmt::Formatter, X, Option<X>) -> std::fmt::Result> MyFmt<X>
    for Foo<A, X>
{
    fn write(
        &self,
        formatter: &mut std::fmt::Formatter,
        value: X,
        step: Option<X>,
    ) -> std::fmt::Result {
        (self.0)(formatter, value, step)
    }
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
pub struct Plotter<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    title: Box<dyn fmt::Display + 'a>,
    xname: Box<dyn fmt::Display + 'a>,
    yname: Box<dyn fmt::Display + 'a>,
    plots: Vec<Plot<'a, X, Y>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    xtick_fmt: Box<dyn MyFmt<X> + 'a>,
    ytick_fmt: Box<dyn MyFmt<Y> + 'a>,
}

impl<'a, X: PlotNum, Y: PlotNum> Plotter<'a, X, Y> {
    ///
    /// Create a plotter with the specified element.
    ///
    /// ```
    /// let mut p = poloto::Plotter::new("title", "x", "y");
    /// p.line("",[[1,1]]);
    /// ```
    pub fn new(
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<'a, X, Y> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: Vec::new(),
            xmarkers: Vec::new(),
            ymarkers: Vec::new(),
            num_css_classes: Some(8),
            preserve_aspect: false,
            xtick_fmt: Box::new(Foo::<_, X>::new(move |a, b, c| b.fmt_tick(a, c))),
            ytick_fmt: Box::new(Foo::<_, Y>::new(move |a, b, c| b.fmt_tick(a, c))),
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
        I::Item: Plottable<X, Y>,
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
        I::Item: Plottable<X, Y>,
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
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
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
        I::Item: Plottable<X, Y>,
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
        I::Item: Plottable<X, Y>,
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
    /// plotter.xmarker(0.0).ymarker(0.0);
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
    /// plotter.xmarker(0.0).ymarker(0.0);
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
    pub fn move_into(&mut self) -> Plotter<'a, X, Y> {
        let mut empty = crate::Plotter::new("", "", "");
        core::mem::swap(&mut empty, self);
        empty
    }

    ///
    /// Overrides the [`PlotNum::fmt_tick`] function.
    /// The callback function provided will get called on each
    /// interval tick to be drawn. The callback function is passed
    /// the value of the interval, as well as the step-size.
    /// This function is also called to display `k` and `j` values
    /// for when the magnitudes of of the plots are extreme.
    /// In those cases, the step-size is not provided and `None` is passed.
    ///
    pub fn xinterval_fmt(
        &mut self,
        a: impl Fn(&mut std::fmt::Formatter, X, Option<X>) -> std::fmt::Result + 'a,
    ) -> &mut Self {
        self.xtick_fmt = Box::new(Foo::new(a));
        self
    }

    ///
    /// Overrides the [`PlotNum::fmt_tick`] function.
    /// The callback function provided will get called on each
    /// interval tick to be drawn. The callback function is passed
    /// the value of the interval, as well as the step-size.
    /// This function is also called to display `k` and `j` values
    /// for when the magnitudes of of the plots are extreme.
    /// In those cases, the step-size is not provided and `None` is passed.
    ///
    pub fn yinterval_fmt(
        &mut self,
        a: impl Fn(&mut std::fmt::Formatter, Y, Option<Y>) -> std::fmt::Result + 'a,
    ) -> &mut Self {
        self.ytick_fmt = Box::new(Foo::new(a));
        self
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
    pub fn render<T: std::fmt::Write>(&mut self, a: T) {
        render::render(self, a);
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
    pub fn simple_theme<T: std::fmt::Write>(&mut self, mut a: T) {
        write!(
            &mut a,
            "{}<style>{}</style>",
            SVG_HEADER, STYLE_CONFIG_LIGHT_DEFAULT
        )
        .unwrap();
        self.render(&mut a);
        write!(&mut a, "{}", SVG_END).unwrap();
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
    pub fn simple_theme_dark<T: std::fmt::Write>(&mut self, mut a: T) {
        write!(
            &mut a,
            "{}<style>{}</style>",
            SVG_HEADER, STYLE_CONFIG_DARK_DEFAULT
        )
        .unwrap();
        self.render(&mut a);
        write!(&mut a, "{}", SVG_END).unwrap();
    }
}
/*
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
*/

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
/// Leverage rust's display format system using `RefCell` under the hood.
///
pub fn disp_mut<F: FnMut(&mut fmt::Formatter)>(a: F) -> DisplayableClosureMut<F> {
    DisplayableClosureMut::new(a)
}

use std::cell::RefCell;

///
/// Wrap a mutable closure in a `RefCell` to allow it to be called inside of `fmt::Display::fmt`
///
pub struct DisplayableClosureMut<F>(pub RefCell<F>);

impl<F: FnMut(&mut fmt::Formatter)> DisplayableClosureMut<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosureMut(RefCell::new(a))
    }
}
impl<F: FnMut(&mut fmt::Formatter)> fmt::Display for DisplayableClosureMut<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self.0.borrow_mut())(formatter);
        Ok(())
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

///
/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`Croppable`]
///
pub trait DiscNum: PlotNum {
    /// Create a hole value.
    fn hole() -> Self;
}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy + std::fmt::Display {
    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool {
        false
    }

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self>;

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range() -> [Self; 2];

    /// Provided a min and max range, scale the current value against max.
    fn scale(&self, val: [Self; 2], max: f64) -> f64;

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        _step: Option<Self>,
    ) -> std::fmt::Result {
        write!(formatter, "{}", self)
    }

    /// Compute the exact size of the ticks.
    /// This can be overridden such that it always returns `None`.
    /// This way there will be no dashes, and it will just be a solid line.
    fn dash_size(
        ideal_dash_size: f64,
        tick_info: &TickInfo<Self>,
        range: [Self; 2],
        max: f64,
    ) -> Option<f64>;
}

///
/// One interval tick
///
#[derive(Clone)]
pub struct Tick<I> {
    pub position: I,
    /// If [`TickInfo::display_relative`] is `None`, then this has the same value as [`Tick::position`]
    pub value: I,
}

///
/// Information on the properties of all the interval ticks for one dimension.
///
#[derive(Clone)]
pub struct TickInfo<I> {
    /// List of the position of each tick to be displayed.
    pub ticks: Vec<Tick<I>>,
    /// The difference between two adjacent ticks
    pub step: I,
    /// The starting tick position
    pub start_step: I,
    /// The number of dashes between two ticks must be a multiple of this number.
    pub dash_multiple: u32,

    /// If we want to display the tick values relatively, this will
    /// have the base start to start with.
    pub display_relative: Option<I>,
}
impl<I> TickInfo<I> {
    pub fn map<J>(self, func: impl Fn(I) -> J) -> TickInfo<J> {
        TickInfo {
            ticks: self
                .ticks
                .into_iter()
                .map(|x| Tick {
                    position: func(x.position),
                    value: func(x.value),
                })
                .collect(),
            step: func(self.step),
            start_step: func(self.start_step),
            dash_multiple: self.dash_multiple,
            display_relative: self.display_relative.map(|x| func(x)),
        }
    }
}
