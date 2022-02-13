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

use std::fmt as sfmt;

pub use tagger::upgrade_write;

pub mod plottable;
use plottable::Plottable;

mod render;
pub mod util;

pub mod plotnum;
use plotnum::*;
pub mod num;
pub mod simple_theme;

use fmt::*;
pub mod fmt;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::formatm;
    //pub use super::plotnum::ext::PlotNumContextExt;
    //pub use super::plotnum::HasDefaultContext;
    pub use super::plottable::crop::Croppable;
    pub use super::simple_theme::SimpleTheme;
}

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

trait PlotTrait<X: PlotNum, Y: PlotNum> {
    fn write_name(&self, a: &mut dyn sfmt::Write) -> sfmt::Result;

    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
}

use std::marker::PhantomData;

use sfmt::Display;
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
    fn write_name(&self, a: &mut dyn sfmt::Write) -> sfmt::Result {
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

#[derive(Copy, Clone, Debug)]
pub struct Bound<X> {
    pub min: X,
    pub max: X,
}

impl<X: PlotNum> Bound<X> {
    pub fn default_context(&self) -> X::DefaultContext {
        X::DefaultContext::new(self)
    }
}
impl<X: PlotNum> Bound<X> {
    ///
    /// Create a [`Steps`].
    ///
    pub fn steps<I: Iterator<Item = X>, F: FnMut(&mut dyn sfmt::Write, X) -> sfmt::Result>(
        &self,
        steps: I,
        func: F,
    ) -> Steps<X, I, F> {
        Steps::new(self, steps, func)
    }
}

//TODO only needs to implement iterator
pub struct DataResult<'a, X: PlotNum, Y: PlotNum> {
    plots: Vec<Plot<'a, X, Y>>,
    boundx: Bound<X>,
    boundy: Bound<Y>,
}
impl<'a, X: PlotNum, Y: PlotNum> DataResult<'a, X, Y> {
    pub fn boundx(&self) -> Bound<X> {
        self.boundx
    }
    pub fn boundy(&self) -> Bound<Y> {
        self.boundy
    }

    pub fn plot(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<'a, X::DefaultContext, Y::DefaultContext> {
        let x = X::DefaultContext::new(&self.boundx);
        let y = Y::DefaultContext::new(&self.boundy);
        self.plot_with(title, xname, yname, x, y)
    }

    pub fn plot_with<XC: PlotNumContext<Num = X>, YC: PlotNumContext<Num = Y>>(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
        x: XC,
        y: YC,
    ) -> Plotter<'a, XC, YC> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: self,
            num_css_classes: Some(8),
            preserve_aspect: false,
            xcontext: Some(x),
            ycontext: Some(y),
        }
    }
}

pub fn data<'a, X: PlotNum, Y: PlotNum>() -> Data<'a, X, Y> {
    Data::new()
}

pub struct Data<'a, X: PlotNum, Y: PlotNum> {
    plots: Vec<Plot<'a, X, Y>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
}
impl<'a, X: PlotNum, Y: PlotNum> Data<'a, X, Y> {
    pub fn new() -> Self {
        Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
        }
    }

    pub fn xmarker(&mut self, a: X) {
        self.xmarkers.push(a);
    }

    pub fn ymarker(&mut self, a: Y) {
        self.ymarkers.push(a);
    }

    pub fn plot(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<'a, X::DefaultContext, Y::DefaultContext> {
        self.build().plot(title, xname, yname)
    }

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
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y",f64::default_ctx(),f64::default_ctx());
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
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y",f64::default_ctx(),f64::default_ctx());
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
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y",f64::default_ctx(),f64::default_ctx());
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
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y",f64::default_ctx(),f64::default_ctx());
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

    pub fn build(mut self) -> DataResult<'a, X, Y> {
        let (boundx, boundy) = util::find_bounds(
            self.plots.iter_mut().flat_map(|x| x.plots.iter_first()),
            self.xmarkers,
            self.ymarkers,
        );

        let boundx = Bound {
            min: boundx[0],
            max: boundx[1],
        };
        let boundy = Bound {
            min: boundy[0],
            max: boundy[1],
        };

        DataResult {
            plots: self.plots,
            boundx,
            boundy,
        }
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
pub struct Plotter<'a, X: PlotNumContext + 'a, Y: PlotNumContext + 'a> {
    title: Box<dyn Display + 'a>,
    xname: Box<dyn Display + 'a>,
    yname: Box<dyn Display + 'a>,
    plots: DataResult<'a, X::Num, Y::Num>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    xcontext: Option<X>,
    ycontext: Option<Y>,
}

impl<'a, X: PlotNumContext, Y: PlotNumContext> Plotter<'a, X, Y> {
    pub fn with_xtick<XX: PlotNumContext<Num = X::Num>>(self, a: XX) -> Plotter<'a, XX, Y> {
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
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y",f64::default_ctx(),f64::default_ctx());
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
    /// Use this if you want to embed a svg into your html.
    /// You will just have to add your own svg sag and then supply styling.
    ///
    /// Panics if the render fails.
    ///
    /// In order to meet a more flexible builder pattern, instead of consuming the Plotter,
    /// this function will mutable borrow the Plotter and leave it with empty data.
    ///
    /// ```
    /// use poloto::prelude::*;
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y",f64::default_ctx(),f64::default_ctx());
    /// plotter.line("", &data);
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```
    pub fn render<T: std::fmt::Write>(&mut self, mut a: T) -> sfmt::Result {
        assert!(self.xcontext.is_some());
        assert!(self.ycontext.is_some());

        let xcontext = self.xcontext.as_mut().unwrap();
        let ycontext = self.ycontext.as_mut().unwrap();

        let boundx = [self.plots.boundx.min, self.plots.boundx.max];
        let boundy = [self.plots.boundy.min, self.plots.boundy.max];

        //knowldge of canvas dim
        let mut canvas = render::Canvas::with_options(self.preserve_aspect, self.num_css_classes);

        if let Some(a) = xcontext.ideal_num_ticks() {
            canvas.ideal_num_xsteps = a;
        }
        if let Some(a) = ycontext.ideal_num_ticks() {
            canvas.ideal_num_ysteps = a;
        }

        //TODO put this somewhere?
        let ideal_dash_size = 20.0;

        let tickx = xcontext.compute_ticks(
            canvas.ideal_num_xsteps,
            DashInfo {
                ideal_dash_size,
                max: canvas.scalex,
            },
        );

        let ticky = ycontext.compute_ticks(
            canvas.ideal_num_ysteps,
            DashInfo {
                ideal_dash_size,
                max: canvas.scaley,
            },
        );

        let mut data = render::Data {
            boundx,
            boundy,
            tickx,
            ticky,
        };

        canvas.render_plots(&mut a, self, &data)?;

        canvas.render_base(a, self, &mut data)
    }

    ///
    /// Retrieve the contexts pass in at construction.
    ///
    pub fn into_contexts(self) -> (X, Y) {
        (self.xcontext.unwrap(), self.ycontext.unwrap())
    }
}

/// Shorthand for `disp_const(move |w|write!(w,...))`
/// Similar to `std::format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::disp_const(move |w| write!(w,$($arg)*))
    }
}

///
/// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
///
pub fn disp<F: FnOnce(&mut sfmt::Formatter) -> sfmt::Result>(
    a: F,
) -> util::DisplayableClosureOnce<F> {
    util::DisplayableClosureOnce::new(a)
}

///
/// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
///
pub fn disp_mut<F: FnMut(&mut sfmt::Formatter) -> sfmt::Result>(
    a: F,
) -> util::DisplayableClosureMut<F> {
    util::DisplayableClosureMut::new(a)
}

///
/// Convert a closure to a object that implements Display
///
pub fn disp_const<F: Fn(&mut sfmt::Formatter) -> sfmt::Result>(
    a: F,
) -> util::DisplayableClosure<F> {
    util::DisplayableClosure::new(a)
}

///
/// A distribution of steps manually specified by the user via an iterator.
///
/// Considering using contexts that automatically pick a good step distribution
/// before resulting to using this.
///
pub struct Steps<N, I, F> {
    pub bound: Bound<N>,
    pub steps: I,
    pub func: F,
}

impl<J: PlotNum, I: Iterator<Item = J>, F: FnMut(&mut dyn sfmt::Write, J) -> sfmt::Result>
    Steps<J, I, F>
{
    pub fn new(bound: &Bound<J>, steps: I, func: F) -> Steps<J, I, F> {
        Steps {
            bound: *bound,
            steps,
            func,
        }
    }
}

impl<J: PlotNum, I: Iterator<Item = J>, F: FnMut(&mut dyn sfmt::Write, J) -> sfmt::Result>
    PlotNumContext for Steps<J, I, F>
{
    type StepInfo = ();
    type Num = J;

    fn tick_fmt(
        &mut self,
        writer: &mut dyn sfmt::Write,
        val: J,
        _info: &Self::StepInfo,
    ) -> std::fmt::Result {
        (self.func)(writer, val)
    }

    fn where_fmt(&mut self, _writer: &mut dyn std::fmt::Write, _val: J) -> std::fmt::Result {
        unreachable!();
    }

    fn compute_ticks(&mut self, _ideal_num_steps: u32, _dash: DashInfo) -> TickInfo<J, ()> {
        let ticks: Vec<_> = (&mut self.steps)
            .skip_while(|&x| x < self.bound.min)
            .take_while(|&x| x <= self.bound.max)
            .map(|x| Tick {
                value: x,
                position: x,
            })
            .collect();

        assert!(
            ticks.len() >= 2,
            "Atleast two ticks must be created for the given data range."
        );

        TickInfo {
            unit_data: (),
            ticks,
            dash_size: None,
            display_relative: None,
        }
    }
}
