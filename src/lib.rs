//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.
//!
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
pub mod tick_fmt;

pub mod util;

pub mod prelude {
    pub use super::PlotNumContextExt;
}
pub mod ctx {
    use super::*;
    pub use util::f64_::Defaultf64Context as f64;
    pub use util::integer::i128::Defaulti128Context as i128;
    pub use util::integer::unix_timestamp::DefaultUnixTimeContext as UnixTime;
}

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
    LineFillRaw,
}

struct Plot<'a, X: PlotNum, Y: PlotNum> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<X, Y> + 'a>,
}

///
/// Default SVG Header for a Poloto graph.
///
pub const SVG_HEADER: &str = r##"<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"##;

///
/// Default SVG end tag.
///
pub const SVG_END: &str = "</svg>";

/// Default light theme
pub const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto { \
    stroke-linecap:round; \
    stroke-linejoin:round; \
    font-family: 'Tahoma', sans-serif; \
    background-color: AliceBlue;\
    } \
    .poloto_scatter{stroke-width:7} \
    .poloto_line{stroke-width:2} \
    .poloto_text{fill: black;} \
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none} \
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
    background-color: #262626;\
    } \
    .poloto_scatter{stroke-width:7} \
    .poloto_line{stroke-width:2} \
    .poloto_text{fill: white;} \
    .poloto_axis_lines{stroke: white;stroke-width:3;fill:none;stroke-dasharray:none} \
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
pub fn plot<'a, X: PlotNumContext, Y: PlotNumContext>(
    xcontext: X,
    ycontext: Y,
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a, X, Y> {
    Plotter::new(xcontext, ycontext, title, xname, yname)
}

/*
trait MyFmt<X: PlotNum> {
    fn write(
        &self,
        formatter: &mut std::fmt::Formatter,
        value: X,
        step: X::UnitData,
        fmt: FmtFull,
    ) -> std::fmt::Result;
}

struct Foo<A, X>(A, PhantomData<X>);
impl<X: PlotNum, A: Fn(&mut std::fmt::Formatter, X, X::UnitData, FmtFull) -> std::fmt::Result>
    Foo<A, X>
{
    fn new(a: A) -> Foo<A, X> {
        Foo(a, PhantomData)
    }
}
impl<X: PlotNum, A: Fn(&mut std::fmt::Formatter, X, X::UnitData, FmtFull) -> std::fmt::Result>
    MyFmt<X> for Foo<A, X>
{
    fn write(
        &self,
        formatter: &mut std::fmt::Formatter,
        value: X,
        step: X::UnitData,
        fmt: FmtFull,
    ) -> std::fmt::Result {
        (self.0)(formatter, value, step, fmt)
    }
}
*/

/// Keeps track of plots.
/// User supplies iterators that will be iterated on when
/// render is called.
///
/// * The svg element belongs to the `poloto` css class.
/// * The title,xname,yname,legend text SVG elements belong to the `poloto_text` class.
/// * The axis line SVG elements belong to the `poloto_axis_lines` class.
/// * The background belongs to the `poloto_background` class.
///
pub struct Plotter<'a, X: PlotNumContext + 'a, Y: PlotNumContext + 'a> {
    title: Box<dyn fmt::Display + 'a>,
    xname: Box<dyn fmt::Display + 'a>,
    yname: Box<dyn fmt::Display + 'a>,
    plots: Vec<Plot<'a, X::Num, Y::Num>>,
    xmarkers: Vec<X::Num>,
    ymarkers: Vec<Y::Num>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    xcontext: X,
    ycontext: Y,
}

impl<'a, X: PlotNumContext, Y: PlotNumContext> Plotter<'a, X, Y> {
    ///
    /// Create a plotter with the specified element.
    ///
    /// ```
    /// let mut p = poloto::Plotter::new("title", "x", "y");
    /// p.line("",[[1,1]]);
    /// ```
    pub fn new(
        xcontext: X,
        ycontext: Y,
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
            xcontext,
            ycontext,
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
        I::Item: Plottable<X::Num, Y::Num>,
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
        I::Item: Plottable<X::Num, Y::Num>,
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

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line_fill_raw("", &data);
    /// ```
    pub fn line_fill_raw<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable<X::Num, Y::Num>,
    {
        self.plots.push(Plot {
            plot_type: PlotType::LineFillRaw,
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
        I::Item: Plottable<X::Num, Y::Num>,
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
        I::Item: Plottable<X::Num, Y::Num>,
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
    pub fn xmarker(&mut self, marker: X::Num) -> &mut Self {
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
    pub fn ymarker(&mut self, marker: Y::Num) -> &mut Self {
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

    /*
    ///
    /// Move a plotter out from behind a mutable reference leaving
    /// an empty plotter.
    ///
    pub fn move_into(&mut self) -> Plotter<'a, X, Y> {
        let mut empty = crate::Plotter::new("", "", "");
        core::mem::swap(&mut empty, self);
        empty
    }
    */

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
    pub fn render<T: std::fmt::Write>(&mut self, a: T) -> fmt::Result {
        render::render(self, a)
    }
}

///
/// Make a graph with a svg tag and a simple css theme.
///
/// ```
/// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
/// let mut plotter = poloto::plot("title", "x", "y");
/// plotter.line("", &data);
/// let mut k=String::new();
/// poloto::simple_theme(&mut k,plotter);
/// ```
pub fn simple_theme<T: std::fmt::Write, X: PlotNumContext, Y: PlotNumContext>(
    mut a: T,
    mut p: Plotter<X, Y>,
) -> std::fmt::Result {
    write!(
        &mut a,
        "{}<style>{}</style>{}{}",
        SVG_HEADER,
        STYLE_CONFIG_LIGHT_DEFAULT,
        disp(|a| p.render(a)),
        SVG_END
    )
}

///
/// Make a graph with a svg tag and a simple dark css theme.
///
/// ```
/// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
/// let mut plotter = poloto::plot("title", "x", "y");
/// plotter.line("", &data);
/// let mut k=String::new();
/// poloto::simple_theme_dark(&mut k,plotter);
/// ```
pub fn simple_theme_dark<T: std::fmt::Write, X: PlotNumContext, Y: PlotNumContext>(
    mut a: T,
    mut p: Plotter<X, Y>,
) -> std::fmt::Result {
    write!(
        &mut a,
        "{}<style>{}</style>{}{}",
        SVG_HEADER,
        STYLE_CONFIG_DARK_DEFAULT,
        disp(|a| p.render(a)),
        SVG_END
    )
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
/// Leverage rust's display format system using `RefCell` under the hood.
///
pub fn disp<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(a: F) -> DisplayableClosureOnce<F> {
    DisplayableClosureOnce::new(a)
}

use std::cell::RefCell;

///
/// Wrap a mutable closure in a `RefCell` to allow it to be called inside of `fmt::Display::fmt`
///
pub struct DisplayableClosureOnce<F>(pub RefCell<Option<F>>);

impl<F: FnOnce(&mut fmt::Formatter) -> fmt::Result> DisplayableClosureOnce<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosureOnce(RefCell::new(Some(a)))
    }
}
impl<F: FnOnce(&mut fmt::Formatter) -> fmt::Result> fmt::Display for DisplayableClosureOnce<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(f) = (self.0.borrow_mut()).take() {
            (f)(formatter)
        } else {
            Ok(())
        }
    }
}

///
/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`Croppable`]
///
pub trait DiscNum: PlotNum {
    /// Create a hole value.
    fn hole() -> Self;
}

pub struct WithNumTicks<T: PlotNumContext> {
    t: T,
    num: u32,
}
impl<P: PlotNumContext> PlotNumContext for WithNumTicks<P> {
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        self.t.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.t.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.t.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        self.t.fmt_tick(formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        Some(self.num)
    }
}

pub struct WithFmt<T, F> {
    pub t: T,
    pub func: F,
}
impl<
        P: PlotNumContext,
        F: FnMut(&mut dyn std::fmt::Write, P::Num, P::UnitData, FmtFull) -> std::fmt::Result,
    > PlotNumContext for WithFmt<P, F>
{
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        self.t.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.t.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.t.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        mut formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        (self.func)(&mut formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.t.ideal_num_ticks()
    }
}

pub struct NoDash<T>(pub T);

impl<P: PlotNumContext> PlotNumContext for NoDash<P> {
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        let mut t = self.0.compute_ticks(ideal_num_steps, range, dash);
        t.dash_size = None;
        t
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.0.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.0.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        self.0.fmt_tick(formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.0.ideal_num_ticks()
    }
}

pub trait PlotNumContextExt: PlotNumContext + Sized {
    fn no_dash(self) -> NoDash<Self> {
        NoDash(self)
    }

    fn with_fmt<F>(self, func: F) -> WithFmt<Self, F>
    where
        F: FnMut(&mut dyn std::fmt::Write, Self::Num, Self::UnitData, FmtFull) -> std::fmt::Result,
    {
        WithFmt { t: self, func }
    }

    fn with_ideal_num_ticks(self, num: u32) -> WithNumTicks<Self> {
        WithNumTicks { t: self, num }
    }
}
impl<T: PlotNumContext> PlotNumContextExt for T {}

pub trait PlotNumContext {
    type UnitData: Copy;
    type Num: PlotNum;
    type TickIter: Iterator<Item = Tick<Self::Num>>;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter>;

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2];

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64;

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        mut formatter: T,
        val: Self::Num,
        _step: Self::UnitData,
        _draw_full: FmtFull,
    ) -> std::fmt::Result {
        write!(formatter, "{}", val)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        None
    }
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
}

pub struct DashInfo {
    //The ideal dash size in the drawing area
    pub ideal_dash_size: f64,

    //The total drawing area
    pub max: f64,
}

pub enum FmtFull {
    Full,
    Tick,
}

///
/// One interval tick
///
#[derive(Debug, Clone, Copy)]
pub struct Tick<I> {
    pub position: I,
    /// If [`TickInfo::display_relative`] is `None`, then this has the same value as [`Tick::position`]
    pub value: I,
}

///
/// Information on the properties of all the interval ticks for one dimension.
///
#[derive(Debug, Clone)]
pub struct TickInfo<I, D, IT: Iterator<Item = Tick<I>>> {
    pub unit_data: D,

    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    //pub ticks: Vec<Tick<I>>,
    pub ticks: IT,

    /// The number of dashes between two ticks must be a multiple of this number.
    //pub dash_multiple: u32,
    pub dash_size: Option<f64>,

    /// If we want to display the tick values relatively, this will
    /// have the base start to start with.
    pub display_relative: Option<I>,
}
