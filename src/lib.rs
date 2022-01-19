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

pub mod plottable;
use plottable::Plottable;

mod render;
pub mod util;

pub mod plotnum;
use plotnum::*;
pub mod num;

pub mod context_ext;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::context_ext::PlotNumContextExt;
    pub use super::formatm;
    pub use super::plotnum::HasDefaultCtx;
    pub use super::plottable::crop::Croppable;
    pub use super::SimpleTheme;
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

///
/// Create a Plotter
///
pub fn plot<'a, X: HasDefaultCtx, Y: HasDefaultCtx>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a, X, Y, X::DefaultContext, Y::DefaultContext> {
    Plotter::new(
        title,
        xname,
        yname,
        X::DefaultContext::default(),
        Y::DefaultContext::default(),
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
pub struct Plotter<
    'a,
    X: PlotNum + 'a,
    Y: PlotNum + 'a,
    XC: PlotNumContext<Num = X>,
    YC: PlotNumContext<Num = Y>,
> {
    title: Box<dyn fmt::Display + 'a>,
    xname: Box<dyn fmt::Display + 'a>,
    yname: Box<dyn fmt::Display + 'a>,
    plots: Vec<Plot<'a, X, Y>>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,

    //Only none after move_into() is called on this object.
    //if render() is called after move_into() is called, it will panic.
    pub xcontext: Option<XC>,
    pub ycontext: Option<YC>,
}

impl<'a, X: PlotNum, Y: PlotNum, XC: PlotNumContext<Num = X>, YC: PlotNumContext<Num = Y>>
    Plotter<'a, X, Y, XC, YC>
{
    pub fn move_into(&mut self) -> Self {
        let mut dummy = Plotter {
            title: Box::new(""),
            xname: Box::new(""),
            yname: Box::new(""),
            plots: Vec::new(),
            num_css_classes: self.num_css_classes,
            preserve_aspect: self.preserve_aspect,
            xcontext: None,
            ycontext: None,
        };

        std::mem::swap(&mut dummy, self);

        dummy
    }

    pub fn with_xcontext<XC2: PlotNumContext<Num = X>>(self, a: XC2) -> Plotter<'a, X, Y, XC2, YC> {
        Plotter {
            title: self.title,
            xname: self.xname,
            yname: self.yname,
            plots: self.plots,
            num_css_classes: self.num_css_classes,
            preserve_aspect: self.preserve_aspect,
            xcontext: Some(a),
            ycontext: self.ycontext,
        }
    }

    pub fn with_ycontext<YC2: PlotNumContext<Num = Y>>(self, a: YC2) -> Plotter<'a, X, Y, XC, YC2> {
        Plotter {
            title: self.title,
            xname: self.xname,
            yname: self.yname,
            plots: self.plots,
            num_css_classes: self.num_css_classes,
            preserve_aspect: self.preserve_aspect,
            xcontext: self.xcontext,
            ycontext: Some(a),
        }
    }

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
        xcontext: XC,
        ycontext: YC,
    ) -> Plotter<'a, X, Y, XC, YC> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: Vec::new(),
            num_css_classes: Some(8),
            preserve_aspect: false,
            xcontext: Some(xcontext),
            ycontext: Some(ycontext),
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
        I::Item: Plottable<X, Y>,
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
        assert!(self.xcontext.is_some());
        assert!(self.ycontext.is_some());

        render::render(self, a)
    }
}

pub trait SimpleTheme {
    fn simple_theme<T: fmt::Write>(&mut self, a: T) -> std::fmt::Result;
    fn simple_theme_dark<T: fmt::Write>(&mut self, a: T) -> std::fmt::Result;
}

impl<X: PlotNum, Y: PlotNum, XC: PlotNumContext<Num = X>, YC: PlotNumContext<Num = Y>> SimpleTheme
    for Plotter<'_, X, Y, XC, YC>
{
    fn simple_theme<T: std::fmt::Write>(&mut self, mut a: T) -> std::fmt::Result {
        write!(
            &mut a,
            "{}<style>{}</style>{}{}",
            SVG_HEADER,
            STYLE_CONFIG_LIGHT_DEFAULT,
            disp(|a| self.render(a)),
            SVG_END
        )
    }

    fn simple_theme_dark<T: std::fmt::Write>(&mut self, mut a: T) -> std::fmt::Result {
        write!(
            &mut a,
            "{}<style>{}</style>{}{}",
            SVG_HEADER,
            STYLE_CONFIG_DARK_DEFAULT,
            disp(|a| self.render(a)),
            SVG_END
        )
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
