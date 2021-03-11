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
mod util;

///The poloto prelude.
pub mod prelude {
    pub use super::iter::PlotIterator;
    pub use super::move_format;
}
use core::fmt;

mod render;
pub use render::StyleBuilder;

use iter::DoubleIterator;

///Contains the [`DoubleIterator`] trait and three different
///implementers of it.
pub mod iter;

///Contains building blocks for create the default svg an styling tags from scratch.
pub mod default_tags {
    pub use super::render::NUM_COLORS;
    use core::fmt;

    ///The class of the svg tag.
    pub const CLASS: &str = "poloto";
    ///The width of the svg tag.
    pub const WIDTH: f64 = 800.0;
    ///The height of the svg tag.
    pub const HEIGHT: f64 = 500.0;
    ///The xmlns: `http://www.w3.org/2000/svg`
    pub const XMLNS: &str = "http://www.w3.org/2000/svg";

    ///Write default svg tag attributes.
    pub fn default_svg_attrs<'a, 'b, T: fmt::Write>(
        w: &'a mut tagger::AttributeWriter<'b, T>,
    ) -> Result<&'a mut tagger::AttributeWriter<'b, T>, fmt::Error> {
        use tagger::prelude::*;

        w.attr("class", CLASS)?
            .attr("width", WIDTH)?
            .attr("height", HEIGHT)?
            .with_attr("viewBox", wr!("0 0 {} {}", WIDTH, HEIGHT))?
            .attr("xmlns", XMLNS)
    }
}

trait PlotTrait {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result;
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
}

use fmt::Display;
struct Wrapper2<D: DoubleIterator, F: Display> {
    a: Option<D>,
    b: Option<D::Next>,
    func: F,
}

impl<I: DoubleIterator<Item = [f64; 2]>, F: Display> Wrapper2<I, F> {
    fn new(it: I, func: F) -> Self {
        Wrapper2 {
            a: Some(it),
            b: None,
            func,
        }
    }
}

impl<D: DoubleIterator<Item = [f64; 2]>, F: Display> PlotTrait for Wrapper2<D, F> {
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

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.

//Its important to note that most of the time when this library is used,
//every run through the code is first accompanied by one compilation of the code.
//So inefficiencies in dynamically allocating strings using format!() to then
//be just passed to a writer are not that bad seeing as the solution
//would involve passing a lot of closures around.
pub struct Plotter<'a> {
    names: Box<dyn Names + 'a>,
    plots: Vec<Plot<'a>>,
    data: Box<dyn Display + 'a>,
    svgtag: SvgTagOption,
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
pub fn concatenate_display(
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

struct NamesStruct<A, B, C> {
    title: A,
    xname: B,
    yname: C,
}
impl<A: Display, B: Display, C: Display> Names for NamesStruct<A, B, C> {
    fn write_title(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.title.fmt(fm)
    }
    fn write_xname(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.xname.fmt(fm)
    }
    fn write_yname(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.yname.fmt(fm)
    }
}

trait Names {
    fn write_title(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_xname(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_yname(&self, fm: &mut fmt::Formatter) -> fmt::Result;
}

///Convenience function for [`Plotter::new()`].
pub fn plot<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a> {
    Plotter::new(
        title,
        xname,
        yname,
        true,
        DataBuilder::new().push_css_default(),
    )
}

#[derive(Copy, Clone)]
enum SvgTagOption {
    Svg,
    NoSvg,
}

///Insert svg data after the svg element, but before the plot elements.
pub struct DataBuilder<D: Display> {
    style: D,
}

impl Default for DataBuilder<&'static str> {
    fn default() -> Self {
        Self::new()
    }
}

impl DataBuilder<&'static str> {
    pub fn new() -> Self {
        DataBuilder { style: "" }
    }
}
impl<D: Display> DataBuilder<D> {
    ///Push the default poloto css styling.
    pub fn push_css_default(self) -> DataBuilder<impl Display> {
        DataBuilder {
            style: concatenate_display("", self.style, StyleBuilder::new().build()),
        }
    }

    /// Instead of the default style, use one that adds variables.
    ///
    /// This injects what is produced by [`StyleBuilder::build_with_css_variables`] instead of
    /// the default [`StyleBuilder::build`].
    ///
    /// If you embed the generated svg into a html file,
    /// then you can add this example:
    /// ```css
    /// .poloto{
    ///    --poloto_bg_color:"black";
    ///    --poloto_fg_color:"white;
    ///    --poloto_color0:"red";
    ///    --poloto_color1:"green";
    ///    --poloto_color2:"yellow";
    ///    --poloto_color3:"orange";
    ///    --poloto_color4:"purple";
    ///    --poloto_color5:"pink";
    ///    --poloto_color6:"aqua";
    ///    --poloto_color7:"red";
    /// }
    /// ```  
    /// By default these variables are not defined, so the svg falls back on some default colors.
    pub fn push_default_css_with_variable(self) -> DataBuilder<impl Display> {
        DataBuilder {
            style: concatenate_display(
                "",
                self.style,
                StyleBuilder::new().build_with_css_variables(),
            ),
        }
    }
    /// User can inject some svg elements using this function.
    /// They will be inserted right after the svg and default svg tags.
    ///
    /// You can override the css in regular html if you embed the generated svg.
    /// This gives you a lot of flexibility giving your the power to dynamically
    /// change the theme of your svg.
    ///
    /// However, if you want to embed the svg as an image, you lose this ability.
    /// If embedding as IMG is desired, instead the user can insert a custom style into the generated svg itself.
    ///
    pub fn push(self, a: impl fmt::Display) -> DataBuilder<impl Display> {
        DataBuilder {
            style: concatenate_display("", self.style, a),
        }
    }
    fn finish(self) -> D {
        self.style
    }
}

///Plotter Builder. [`Plotter::new`] can do everything this does.
///But sometimes you want to pass around a builder instead of a list of arguments.
pub struct PlotterBuilder<D: fmt::Display> {
    data: DataBuilder<D>,
    svgtag: bool,
}
impl Default for PlotterBuilder<&'static str> {
    fn default() -> Self {
        Self::new()
    }
}
impl PlotterBuilder<&'static str> {
    pub fn new() -> Self {
        PlotterBuilder {
            data: DataBuilder::new(),
            svgtag: true,
        }
    }
    pub fn with_data<J: Display>(self, data: DataBuilder<J>) -> PlotterBuilder<J> {
        PlotterBuilder {
            data,
            svgtag: self.svgtag,
        }
    }
    pub fn with_svg(mut self, svg: bool) -> Self {
        self.svgtag = svg;
        self
    }
}

impl<'a, D: Display + 'a> PlotterBuilder<D> {
    pub fn build(
        self,
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
    ) -> Plotter<'a> {
        Plotter::new(title, xname, yname, self.svgtag, self.data)
    }
}

impl<'a> Plotter<'a> {
    /// Create a plotter
    ///
    /// # Example
    ///
    /// ```
    /// let plotter = poloto::Plotter::new("title","x","y",true,poloto::DataBuilder::new().push_css_default());
    /// ```
    pub fn new(
        title: impl Display + 'a,
        xname: impl Display + 'a,
        yname: impl Display + 'a,
        svgtag: bool,
        data: DataBuilder<impl Display + 'a>,
    ) -> Plotter<'a> {
        let svgtag = if svgtag {
            SvgTagOption::Svg
        } else {
            SvgTagOption::NoSvg
        };

        Plotter {
            names: Box::new(NamesStruct {
                title,
                xname,
                yname,
            }),
            plots: Vec::new(),
            svgtag,
            data: Box::new(data.finish()),
        }
    }

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
            plots: Box::new(Wrapper2::new(plots, name)),
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
            plots: Box::new(Wrapper2::new(plots, name)),
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
            plots: Box::new(Wrapper2::new(plots, name)),
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
            plots: Box::new(Wrapper2::new(plots, name)),
        });
        self
    }

    pub fn render_to_string(self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        self.render(&mut s)?;
        Ok(s)
    }
    pub fn render_fmt(self, f: &mut fmt::Formatter) -> fmt::Result {
        self.render(f)?;
        Ok(())
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
            data,
        } = self;
        let mut root = tagger::Element::new(writer);

        use default_tags::*;

        match svgtag {
            SvgTagOption::Svg => {
                root.elem("svg", |writer| {
                    let svg = writer.write(|w| default_svg_attrs(w))?;

                    render::render(svg.get_writer(), data, plots, names)?;
                    Ok(svg)
                })?;
            }
            SvgTagOption::NoSvg => {
                render::render(root.get_writer(), data, plots, names)?;
            }
        }
        Ok(root.into_writer())
    }
}
