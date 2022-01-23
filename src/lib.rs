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
pub mod simple_theme;

use polotofmt::*;
pub mod polotofmt;

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

struct Plot<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<X, Y> + 'a>,
}

///
/// Create a Plotter
///
pub fn plot<'a, X: PlotNum + 'a, Y: PlotNum + 'a>(
    title: impl PlotterNameFmt<X, Y> + 'a,
    xname: impl PlotterNameSingleFmt<X> + 'a,
    yname: impl PlotterNameSingleFmt<Y> + 'a,
) -> Plotter<'a, X, Y> {
    Plotter::new(title, AxisBuilder::new(xname), AxisBuilder::new(yname))
}

//TODO remove this?
struct TickResult<X: PlotNum, Y: PlotNum> {
    pub tickx: TickInfo<X>,
    pub ticky: TickInfo<Y>,
}

///
/// Specify option for the x and y axis.
///
pub struct AxisBuilder<'a, X: PlotNum> {
    dash: bool,
    ideal_num: Option<u32>,
    markers: Vec<X>,
    name: Box<dyn PlotterNameSingleFmt<X> + 'a>,
    tick_fmt: Box<dyn PlotterTickFmt<X> + 'a>,
}

impl<'a, X: PlotNum + 'a> AxisBuilder<'a, X> {
    pub fn new(name: impl PlotterNameSingleFmt<X> + 'a) -> Self {
        AxisBuilder {
            dash: true,
            ideal_num: None,
            markers: vec![],
            name: Box::new(name),
            tick_fmt: Box::new(default_tick_fmt()),
        }
    }
    pub fn new_ext(
        name: impl PlotterNameSingleFmt<X> + 'a,
        tick_fmt: impl PlotterTickFmt<X> + 'a,
    ) -> Self {
        AxisBuilder {
            dash: true,
            ideal_num: None,
            markers: vec![],
            name: Box::new(name),
            tick_fmt: Box::new(tick_fmt),
        }
    }
    pub fn marker(&mut self, a: X) -> &mut Self {
        self.markers.push(a);
        self
    }
    pub fn with_ideal_num(&mut self, a: u32) -> &mut Self {
        self.ideal_num = Some(a);
        self
    }
    pub fn with_name(&mut self, a: impl PlotterNameSingleFmt<X> + 'a) -> &mut Self {
        self.name = Box::new(a);
        self
    }

    pub fn with_tick_fmt(&mut self, a: impl PlotterTickFmt<X> + 'a) -> &mut Self {
        self.tick_fmt = Box::new(a);
        self
    }
    pub fn no_dash(&mut self) -> &mut Self {
        self.dash = false;
        self
    }
}

//TODO remove this?
struct PlotterRes<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    plots: Vec<Plot<'a, X, Y>>,
    boundx: [X; 2],
    boundy: [Y; 2],
    title: Box<dyn PlotterNameFmt<X, Y> + 'a>,
    xaxis: AxisBuilder<'a, X>,
    yaxis: AxisBuilder<'a, Y>,
    preserve_aspect: bool,
    num_css_classes: Option<usize>,
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
    plots: Vec<Plot<'a, X, Y>>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    xaxis: AxisBuilder<'a, X>,
    yaxis: AxisBuilder<'a, Y>,
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
        xaxis: AxisBuilder<'a, X>,
        yaxis: AxisBuilder<'a, Y>,
    ) -> Plotter<'a, X, Y> {
        Plotter {
            title: Box::new(title),
            plots: Vec::new(),
            num_css_classes: Some(8),
            preserve_aspect: false,
            xaxis,
            yaxis,
        }
    }

    pub fn with_name(&mut self, a: impl PlotterNameFmt<X, Y> + 'a) -> &mut Self {
        self.title = Box::new(a);
        self
    }
    pub fn xaxis(&mut self) -> &mut AxisBuilder<'a, X> {
        &mut self.xaxis
    }

    pub fn yaxis(&mut self) -> &mut AxisBuilder<'a, Y> {
        &mut self.yaxis
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
            &pp.xaxis.markers,
            &pp.yaxis.markers,
        );

        PlotterRes {
            title: pp.title,
            plots: pp.plots,
            boundx,
            boundy,
            xaxis: pp.xaxis,
            yaxis: pp.yaxis,
            num_css_classes: pp.num_css_classes,
            preserve_aspect: pp.preserve_aspect,
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
        let mut canvas = render::Canvas::with_options(data.preserve_aspect, data.num_css_classes);

        //compute step info
        let ticks = canvas.gen_ticks(&data);

        canvas.render(a, data, ticks)
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

///
/// Convert a closure to a object that implements Display
///
pub fn disp_const<F: Fn(&mut fmt::Formatter) -> fmt::Result>(a: F) -> util::DisplayableClosure<F> {
    util::DisplayableClosure::new(a)
}
