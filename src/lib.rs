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
//!
//! Pipeline:
//! * collect plots ([`data`] function)
//! * compute min/max (call [`Data::build`] and generate a [`DataResult`]).
//! * create tick distributions. (using impls of [`TickGenerator`]) (This step can be done automatically by calling [`DataResult::plot`] instead of [`DataResult::plot_with`])
//! * collect title/xname/yname (on creation of [`Plotter`])
//! * write everything to svg [`Plotter::render`] for no svg tag/css. [`simple_theme::SimpleTheme`] for basic css/svg tag.
//!
//! Poloto provides by default 3 impls of [`TickGenerator`]:
//!
//! * [`num::integer::IntegerTickGen`]
//! * [`num::float::FloatTickGen`]
//! * [`num::timestamp::UnixTimeTickGen`]
//!
//! The above generators have the advantage of automatically selecting reasonable
//! tick intervals. The user can change the formatting of the ticks while still using
//! the ticks that were selected via its automatic methods using [`TickDist::with_tick_fmt`].
//!
//! However, sometimes you may want more control on the ticks, or want to use a type
//! other than [`i128`]/[`f64`]/[`UnixTime`](num::timestamp::UnixTime). One way would be to write your own implementation of [`TickGenerator`].
//! Alternatively you can use the [`Bound::steps`] function that just takes an iterator of ticks.
//! This puts more responsiblity on the user to pass a decent number of ticks. This should only really be used when the user
//! knows up front the min and max values of that axis. This is typically the case for
//! at least one of the axis, typically the x axis. [See marathon example](https://github.com/tiby312/poloto/blob/master/examples/marathon.rs)

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

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::formatm;
    pub use super::plotnum::TickFormat;
    pub use super::plotnum::TickGenerator;
    pub use super::plottable::crop::Croppable;
    pub use super::simple_theme::SimpleTheme;
}

use fmt::Display;
use std::marker::PhantomData;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

trait PlotTrait<X: PlotNum, Y: PlotNum> {
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result;
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
}

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
/// Created once the min and max bounds of all the plots has been computed.
/// Contains in it all the information needed to fed into a [`TickGenerator`].
///
#[derive(Copy, Clone)]
pub struct Bound<X> {
    pub min: X,
    pub max: X,
    pub ideal_num_steps: u32,
    pub dash_info: DashInfo,
}

impl<X: PlotNum> Bound<X> {
    ///
    /// Create the default tick generator associated with a PlotNum.
    ///
    pub fn default_ticks(&self) -> TickDist<<X::DefaultTickGenerator as TickGenerator>::Fmt>
    where
        X::DefaultTickGenerator: TickGenerator<Num = X> + Default,
    {
        X::DefaultTickGenerator::default().generate(*self)
    }
}

impl<X: PlotNum> Bound<X> {
    ///
    /// Create a [`num::Steps`].
    ///
    pub fn steps<I: Iterator<Item = X>, F: FnMut(&mut dyn fmt::Write, &X) -> fmt::Result>(
        &self,
        steps: I,
        func: F,
    ) -> TickDist<num::StepFmt<X, F>> {
        num::Steps::new(steps, func).generate(*self)
    }
}


///
/// Created by [`Data::build`]
/// 
pub struct DataResult<'a, X: PlotNum, Y: PlotNum> {
    plots: Vec<Plot<'a, X, Y>>,
    boundx: Bound<X>,
    boundy: Bound<Y>,
    canvas: render::Canvas,
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
    ) -> Plotter<'a, X, Y>
    where
        X::DefaultTickGenerator: TickGenerator<Num = X> + Default,
        Y::DefaultTickGenerator: TickGenerator<Num = Y> + Default,
    {
        let x = self.boundx.default_ticks();
        let y = self.boundy.default_ticks();
        self.plot_with(title, xname, yname, x, y)
    }

    pub fn plot_with(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
        tickx: TickDist<impl TickFormat<Num = X> + 'a>,
        ticky: TickDist<impl TickFormat<Num = Y> + 'a>,
    ) -> Plotter<'a, X, Y> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: self,
            xcontext: Box::new(tickx.fmt),
            ycontext: Box::new(ticky.fmt),
            tickx: tickx.ticks,
            ticky: ticky.ticks,
        }
    }
}

///
/// Start plotting.
/// 
pub fn data<'a, X: PlotNum, Y: PlotNum>() -> Data<'a, X, Y> {
    Data::default()
}


///
/// Plot collector.
/// 
pub struct Data<'a, X: PlotNum, Y: PlotNum> {
    plots: Vec<Plot<'a, X, Y>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
}
impl<'a, X: PlotNum, Y: PlotNum> Default for Data<'a, X, Y> {
    fn default() -> Self {
        Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
            num_css_classes: Some(8),
            preserve_aspect: false,
        }
    }
}
impl<'a, X: PlotNum, Y: PlotNum> Data<'a, X, Y> {
    pub fn xmarker(&mut self, a: X) -> &mut Self {
        self.xmarkers.push(a);
        self
    }

    pub fn ymarker(&mut self, a: Y) -> &mut Self {
        self.ymarkers.push(a);
        self
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
    pub fn move_into(&mut self) -> Self {
        let mut val = Data {
            plots: vec![],
            xmarkers: vec![],
            ymarkers: vec![],
            num_css_classes: None,
            preserve_aspect: false,
        };

        std::mem::swap(&mut val, self);
        val
    }
    pub fn build(&mut self) -> DataResult<'a, X, Y> {
        let mut val = self.move_into();

        let (boundx, boundy) = util::find_bounds(
            val.plots.iter_mut().flat_map(|x| x.plots.iter_first()),
            val.xmarkers.clone(),
            val.ymarkers.clone(),
        );

        let ideal_dash_size = 20.0;

        let canvas = render::Canvas::with_options(val.preserve_aspect, val.num_css_classes);

        let boundx = Bound {
            min: boundx[0],
            max: boundx[1],
            ideal_num_steps: canvas.ideal_num_xsteps,
            dash_info: DashInfo {
                ideal_dash_size,
                max: canvas.scalex,
            },
        };
        let boundy = Bound {
            min: boundy[0],
            max: boundy[1],
            ideal_num_steps: canvas.ideal_num_ysteps,
            dash_info: DashInfo {
                ideal_dash_size,
                max: canvas.scaley,
            },
        };

        DataResult {
            plots: val.plots,
            boundx,
            boundy,
            canvas,
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
pub struct Plotter<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    title: Box<dyn Display + 'a>,
    xname: Box<dyn Display + 'a>,
    yname: Box<dyn Display + 'a>,
    xcontext: Box<dyn TickFormat<Num = X> + 'a>,
    ycontext: Box<dyn TickFormat<Num = Y> + 'a>,
    plots: DataResult<'a, X, Y>,
    tickx: TickInfo<X>,
    ticky: TickInfo<Y>,
}

impl<'a, X: PlotNum, Y: PlotNum> Plotter<'a, X, Y> {
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
    pub fn render<T: std::fmt::Write>(&mut self, mut a: T) -> fmt::Result {
        render::Canvas::render_plots(&mut a, self)?;
        render::Canvas::render_base(&mut a, self)
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
pub fn disp<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(
    a: F,
) -> util::DisplayableClosureOnce<F> {
    util::DisplayableClosureOnce::new(a)
}

///
/// Leverage rust's display format system using [`std::cell::RefCell`] under the hood.
///
pub fn disp_mut<F: FnMut(&mut fmt::Formatter) -> fmt::Result>(
    a: F,
) -> util::DisplayableClosureMut<F> {
    util::DisplayableClosureMut::new(a)
}

///
/// Convert a closure to a object that implements Display
///
pub fn disp_const<F: Fn(&mut fmt::Formatter) -> fmt::Result>(a: F) -> util::DisplayableClosure<F> {
    util::DisplayableClosure::new(a)
}
