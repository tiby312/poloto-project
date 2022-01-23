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

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::formatm;
    pub use super::plottable::crop::Croppable;
    pub use super::simple_theme::SimpleTheme;
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

/*
/// The demsions of the svg graph `[800,500]`.
pub const DIMENSIONS: [usize; 2] = [800, 500];
*/

///
/// Create a Plotter
///
pub fn plot<'a, X: PlotNum, Y: PlotNum>(
    title: impl PlotterNameFmt<X, Y> + 'a,
    xname: impl PlotterNameFmt<X, Y> + 'a,
    yname: impl PlotterNameFmt<X, Y> + 'a,
) -> Plotter<'a, X, Y> {
    Plotter::new(title, xname, yname, default_tick_fmt(), default_tick_fmt())
}

use polotofmt::*;
pub mod polotofmt {
    use super::*;

    ///
    /// Allows to override the default tick formatting using information
    /// such as min and max bounds and step information.
    ///
    pub trait PlotterTickFmt<X: PlotNum> {
        fn fmt_self(&mut self, val: X, data: DataSingle<X>) -> std::fmt::Result;
    }

    pub fn default_tick_fmt<X: PlotNum>() -> impl PlotterTickFmt<X> {
        tick_fmt_ext(|mut v: X, mut d| v.val_fmt(d.writer, d.ff, &mut d.step))
    }
    pub fn tick_fmt_ext<X: PlotNum>(
        func: impl FnMut(X, DataSingle<X>) -> std::fmt::Result,
    ) -> impl PlotterTickFmt<X> {
        impl<X: PlotNum, F> PlotterTickFmt<X> for F
        where
            F: FnMut(X, DataSingle<X>) -> std::fmt::Result,
        {
            fn fmt_self(&mut self, val: X, data: DataSingle<X>) -> std::fmt::Result {
                (self)(val, data)
            }
        }

        func
    }

    ///
    /// Allows to format either the title,xaxis label, or yaxis label
    /// using information such as the min and max bounds or step information.
    ///
    pub trait PlotterNameFmt<X: PlotNum, Y: PlotNum> {
        fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result;
    }

    impl<T: std::fmt::Display, X: PlotNum, Y: PlotNum> PlotterNameFmt<X, Y> for T {
        fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result {
            write!(data.writer, "{}", self)
        }
    }

    pub fn name_ext<X: PlotNum, Y: PlotNum, F: FnMut(Data<X, Y>) -> std::fmt::Result>(
        func: F,
    ) -> impl PlotterNameFmt<X, Y> {
        pub struct NoDisp<F>(pub F);

        impl<X: PlotNum, Y: PlotNum, F> PlotterNameFmt<X, Y> for NoDisp<F>
        where
            F: FnMut(Data<X, Y>) -> std::fmt::Result,
        {
            fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result {
                (self.0)(data)
            }
        }

        NoDisp(func)
    }

    pub struct DataSingle<'a, X: PlotNum> {
        pub writer: &'a mut dyn std::fmt::Write,
        pub bound: [X; 2],
        pub step: X::StepInfo,
        pub ff: FmtFull,
    }
    pub struct Data<'a, X: PlotNum, Y: PlotNum> {
        pub writer: &'a mut dyn std::fmt::Write,
        pub boundx: [X; 2],
        pub boundy: [Y; 2],
        pub stepx: X::StepInfo,
        pub stepy: Y::StepInfo,
    }
}

struct TickResult<X: PlotNum, Y: PlotNum> {
    pub tickx: TickInfo<X>,
    pub ticky: TickInfo<Y>,
}

struct PlotterRes<'a, X: PlotNum, Y: PlotNum> {
    plots: Vec<Plot<'a, X, Y>>,
    boundx: [X; 2],
    boundy: [Y; 2],
    title: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    xname: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    yname: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    xtick_fmt: Box<dyn PlotterTickFmt<X> + 'a>,
    ytick_fmt: Box<dyn PlotterTickFmt<Y> + 'a>,
    dash_x: bool,
    dash_y: bool,
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
    title: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    xname: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    yname: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    xtick_fmt: Box<dyn PlotterTickFmt<X> + 'a>,
    ytick_fmt: Box<dyn PlotterTickFmt<Y> + 'a>,
    plots: Vec<Plot<'a, X, Y>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    dash_x: bool,
    dash_y: bool,
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
        title: impl PlotterNameFmt<X, Y> + 'a,
        xname: impl PlotterNameFmt<X, Y> + 'a,
        yname: impl PlotterNameFmt<X, Y> + 'a,
        xtick_fmt: impl PlotterTickFmt<X> + 'a,
        ytick_fmt: impl PlotterTickFmt<Y> + 'a,
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
            xtick_fmt: Box::new(xtick_fmt),
            ytick_fmt: Box::new(ytick_fmt),
            dash_x: true,
            dash_y: true,
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

    fn find_bounds(&mut self) -> PlotterRes<'a, X, Y> {
        let mut pp = self.move_into();

        let (boundx, boundy) = num::find_bounds(
            pp.plots.iter_mut().flat_map(|x| x.plots.iter_first()),
            pp.xmarkers,
            pp.ymarkers,
        );

        PlotterRes {
            title: pp.title,
            xname: pp.xname,
            yname: pp.yname,
            plots: pp.plots,
            xtick_fmt: pp.xtick_fmt,
            ytick_fmt: pp.ytick_fmt,
            boundx,
            boundy,
            dash_x: pp.dash_x,
            dash_y: pp.dash_y,
        }
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

    pub fn xmarker(&mut self, a: X) -> &mut Self {
        self.xmarkers.push(a);
        self
    }

    pub fn ymarker(&mut self, a: Y) -> &mut Self {
        self.ymarkers.push(a);
        self
    }

    pub fn no_dash_x(&mut self) -> &mut Self {
        self.dash_x = false;
        self
    }
    pub fn no_dash_y(&mut self) -> &mut Self {
        self.dash_y = false;
        self
    }

    ///
    /// Move a plotter out from behind a mutable reference leaving
    /// an empty plotter.
    ///
    pub fn move_into(&mut self) -> Plotter<'a, X, Y> {
        let mut empty = crate::plot("", "", "");
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
    pub fn render<T: std::fmt::Write>(&mut self, a: T) -> fmt::Result {
        let data = self.find_bounds();

        //knowldge of canvas dim
        let canvas = render::Canvas::new();

        //compute step info
        let ticks = canvas.gen_ticks(&data);

        canvas.render(a, data, ticks)
    }
}

pub mod simple_theme {
    use super::*;

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

    ///
    /// Create a simple theme.
    ///
    pub trait SimpleTheme {
        fn simple_theme<T: fmt::Write>(&mut self, a: T) -> std::fmt::Result;
        fn simple_theme_dark<T: fmt::Write>(&mut self, a: T) -> std::fmt::Result;
    }

    impl<X: PlotNum, Y: PlotNum> SimpleTheme for Plotter<'_, X, Y> {
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
}

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::disp_const(move |w| write!(w,$($arg)*))
    }
}

///
/// Leverage rust's display format system using `RefCell` under the hood.
///
pub fn disp<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(
    a: F,
) -> util::DisplayableClosureOnce<F> {
    util::DisplayableClosureOnce::new(a)
}

pub fn disp_const<F: Fn(&mut fmt::Formatter) -> fmt::Result>(a: F) -> util::DisplayableClosure<F> {
    util::DisplayableClosure::new(a)
}
