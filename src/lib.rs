//!
//! poloto - plot to SVG and style with CSS
//!
//! ### Usage
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! * Plots containing NaN or Infinity are ignored.
//! * After 8 plots, the colors cycle back and are repeated.
//!
use core::fmt::Write;

pub use tagger;
pub mod build;
mod util;
use build::*;

///The poloto prelude.
pub mod prelude {
    pub use super::iter::PlotIterator;
    pub use super::move_format;
}
use core::fmt;

mod render;

use iter::DoubleIterator;

///Contains the [`DoubleIterator`] trait and three different
///implementers of it.
pub mod iter;

trait PlotTrait {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result;
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
}

use fmt::Display;
struct PlotStruct<D: DoubleIterator, F: Display> {
    a: Option<D>,
    b: Option<D::Next>,
    func: F,
}

impl<I: DoubleIterator<Item = [f64; 2]>, F: Display> PlotStruct<I, F> {
    fn new(it: I, func: F) -> Self {
        PlotStruct {
            a: Some(it),
            b: None,
            func,
        }
    }
}

impl<D: DoubleIterator<Item = [f64; 2]>, F: Display> PlotTrait for PlotStruct<D, F> {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result {
        self.func.fmt(a)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]> {
        self.a.as_mut().unwrap()
    }

    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]> {
        self.b = Some(self.a.take().unwrap().finish_first());
        self.b.as_mut().unwrap()
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

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! move_format {
    ($($arg:tt)*) => {
        $crate::moveable_format(move |w| write!(w,$($arg)*))
    }
}

/*
pub struct DisplayList<'a, T> {
    seperator: T,
    a: Vec<Box<dyn Display + 'a>>,
}
impl<'a, T: fmt::Display> DisplayList<'a, T> {
    pub fn new(seperator: T) -> Self {
        DisplayList {
            seperator,
            a: Vec::new(),
        }
    }
    pub fn add(&mut self, a: impl Display + 'a) {
        self.a.push(Box::new(a));
    }
}
impl<'a, T: fmt::Display> fmt::Display for DisplayList<'a, T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        for a in self.a.iter() {
            a.fmt(formatter)?;
            self.seperator.fmt(formatter)?;
        }
        Ok(())
    }
}
*/

///Concatenate two display objects with the specified spacing inbetween.
fn concatenate_display(
    spacing: impl fmt::Display,
    a: impl fmt::Display,
    b: impl fmt::Display,
) -> impl fmt::Display {
    struct Foo<A, B, C> {
        spacing: A,
        a: B,
        b: C,
    }
    impl<A, B, C> fmt::Display for Foo<A, B, C>
    where
        A: fmt::Display,
        B: fmt::Display,
        C: fmt::Display,
    {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            self.a.fmt(formatter)?;
            self.spacing.fmt(formatter)?;
            self.b.fmt(formatter)
        }
    }
    Foo { spacing, a, b }
}

///Convert a moved closure into a impl fmt::Display.
///This is useful because std's `format_args!()` macro
///has a shorter lifetime.
pub fn moveable_format(func: impl Fn(&mut fmt::Formatter) -> fmt::Result) -> impl fmt::Display {
    struct Foo<F>(F);
    impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Foo<F> {
        fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            (self.0)(formatter)
        }
    }
    Foo(func)
}

///Convenience function for [`PlotterBuilder`] with default css tag, and with svg tag.
///In most cases, these defaults are good enough.
pub fn plot<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a, impl Names> {
    build::PlotterBuilder::new()
        .with_header(build::HeaderBuilder::new().push_css_default().build())
        .build(title, xname, yname)
}

#[derive(Copy, Clone)]
enum SvgTagOption {
    Svg,
    NoSvg,
}

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
//Its important to note that most of the time when this library is used,
//every run through the code is first accompanied by one compilation of the code.
//So inefficiencies in dynamically allocating strings using format!() to then
//be just passed to a writer are not that bad seeing as the solution
//would involve passing a lot of closures around.
pub struct Plotter<'a, D: Names> {
    names: D,
    plots: Vec<Plot<'a>>,
    svgtag: SvgTagOption,
}

impl<'a, D: Names> Plotter<'a, D> {
    /// Create a line from plots.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f64,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// use poloto::prelude::*;
    /// let mut plotter = poloto::plot("title","x","y");
    /// plotter.line("data",data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn line(
        &mut self,
        name: impl Display + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(PlotStruct::new(plots, name)),
        });
        self
    }

    /// Create a line from plots that will be filled underneath.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f64,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// use poloto::prelude::*;
    /// let mut plotter = poloto::plot("title","x","y");
    /// plotter.line_fill("data",data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn line_fill(
        &mut self,
        name: impl Display + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(PlotStruct::new(plots, name)),
        });
        self
    }

    /// Create a scatter plot from plots.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f64,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// use poloto::prelude::*;
    /// let mut plotter = poloto::plot("title","x","y");
    /// plotter.scatter("data",data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn scatter(
        &mut self,
        name: impl Display + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(PlotStruct::new(plots, name)),
        });
        self
    }

    /// Create a histogram from plots.
    /// Each bar's left side will line up with a point
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f64,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// use poloto::prelude::*;
    /// let mut s=String::new();
    /// let mut plotter = poloto::plot("title","x","y");
    /// plotter.histogram("data",data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn histogram(
        &mut self,
        name: impl Display + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(PlotStruct::new(plots, name)),
        });
        self
    }

    pub fn render_to_string(self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        self.render(&mut s)?;
        Ok(s)
    }

    pub fn render_io<T: std::io::Write>(self, writer: T) -> Result<T, fmt::Error> {
        self.render(tagger::upgrade(writer)).map(|x| x.inner)
    }
    /// Render the svg to the writer.
    ///
    /// Up until now, nothing has been written to the writer. We
    /// have just accumulated a list of commands and closures. This call will
    /// actually call all the closures and consume all the plot iterators.
    pub fn render<T: fmt::Write>(self, writer: T) -> Result<T, fmt::Error> {
        let Plotter {
            names,
            plots,
            svgtag,
        } = self;
        let mut root = tagger::Element::new(writer);

        use crate::build::default_tags::*;

        match svgtag {
            SvgTagOption::Svg => {
                root.elem("svg", |writer| {
                    let (svg,()) = writer.write(|w| default_svg_attrs(w)?.empty_ok())?;

                    render::render(svg.get_writer(), plots, names)?;
                    svg.empty_ok()
                })?;
            }
            SvgTagOption::NoSvg => {
                render::render(root.get_writer(), plots, names)?;
            }
        }
        Ok(root.into_writer())
    }
}
