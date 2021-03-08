//!
//! poloto - plot to SVG and style with CSS
//!
//! ### Usage
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! * Plots containing NaN or Infinity are ignored.
//! * After 6 plots, the colors cycle back and are repeated.
//! 
//!
//! ### Units
//!
//! Poloto will first print intervals is normal decimal at the precision required to capture the differences
//! in the step size between the intervals. If the magnitude of a number is detected to be too big or small, it
//! may switch to scientific notation, still at the required precision. It will only switch if the scientific
//! notation version is actually less characters than the normal decimal format which is not always the case
//! when you consider the precision that might be required to capture the step size.
//!
//! Even with the above system, there are cases where the numbers all have a really big magnitude, but
//! are all really close together (small step size). In this case, there isnt a really good way to format it.
//! In this instance, poloto will fall back to making the number relative to the first number.
//!
//! ### How do I change the color of the plots?
//!
//! You can doing it by overriding the css. If you embed the generated svg into a html file,
//! then you can add this example:
//! ```css
//! .poloto{
//!    --poloto_bg_color:"black";
//!    --poloto_fg_color:"white;
//!    --poloto_color0:"red";
//!    --poloto_color1:"green";
//!    --poloto_color2:"yellow";
//!    --poloto_color3:"orange";
//!    --poloto_color4:"purple";
//!    --poloto_color5:"pink";
//!    --poloto_color6:"aqua";
//!    --poloto_color7:"red";
//! }
//! ```  
//! By default these variables are not defined, so the svg falls back on some default colors.
//!
//! ### Can I change the styling of the plots?
//!
//! Yes! You can harness the power of CSS both in the svg, or outside
//! in html with an embeded svg. Some things you can do:
//!
//! * Change the color scheme to fit your html theme.
//! * Highlight one plot, make it dashed, or add hover effect
//! * Animate things using @keyframes
//!
//! Depending on whether you are adding a new style attribute or overriding
//! an existing one, you might have to increase the specificty of your css clause to make sure it overrides
//! the svg css clause.
//! ### Why not scale the intervals to end nicely with the ends of the axis lines?
//!
//! Doing this you would have to either have more dead space, or exclude
//! plots that the user would expect to get plotted. Neither of these sounded
//! better than the option of just having the intervals stop not necessarily
//! at the end of the axis lines.
//!
//! ### Example
//!
//! See the graphs in this report: [broccoli_book](https://tiby312.github.io/broccoli_report/)
//!
use core::fmt::Write;
use core::marker::PhantomData;

pub use tagger;
mod util;

///The poloto prelude.
pub mod prelude {
    pub use super::iter::PlotIterator;
    pub use core::fmt::Write;
    pub use tagger::wr;
}
use core::fmt;
mod render;

use iter::DoubleIterator;

///Contains the [`DoubleIterator`] trait and tree different
///implementers of it.
pub mod iter;

///Contains building blocks for create the default svg an styling tags from scratch.
pub mod default_tags {
    use core::fmt;

    pub use super::render::default_styling;

    ///The class of the svg tag.
    pub const CLASS: &str = "poloto";
    ///The width of the svg tag.
    pub const WIDTH: f64 = 800.0;
    ///The height of the svg tag.
    pub const HEIGHT: f64 = 500.0;
    ///The xmlns: `http://www.w3.org/2000/svg`
    pub const XMLNS: &str = "http://www.w3.org/2000/svg";

    ///Returns a function that will write default svg tag attributes.
    pub fn default_svg_attrs<T: fmt::Write>(
    ) -> impl FnOnce(&mut tagger::AttributeWriter<T>) -> Result<(), fmt::Error> {
        use tagger::prelude::*;

        |w| {
            w.attr("class", CLASS)?
                .attr("width", WIDTH)?
                .attr("height", HEIGHT)?
                .with_attr("viewBox", wr!("0 0 {} {}", WIDTH, HEIGHT))?
                .attr("xmlns", XMLNS)?;
            Ok(())
        }
    }
    use core::fmt::Write;
    ///Add the svg tag and css styling.
    pub fn default_svg_and_styling<T: Write>(
        writer: T,
        func: impl FnOnce(&mut tagger::Element<T>) -> Result<&mut tagger::Element<T>, fmt::Error>,
    ) -> Result<T, fmt::Error> {
        let mut root = tagger::Element::new(writer);

        root.elem("svg", |writer| {
            let mut svg = writer.write(|w| {
                default_svg_attrs()(w)?;

                Ok(w)
            })?;
            default_styling(&mut svg)?;
            func(svg)
        })?;

        Ok(root.into_writer())
    }
}

trait PlotTrait<T: fmt::Write> {
    fn write_name(&mut self, a: &mut T) -> fmt::Result;
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
}

struct Wrapper2<D: DoubleIterator, F, T> {
    a: Option<D>,
    b: Option<D::Next>,
    func: Option<F>,
    _p: PhantomData<T>,
}

impl<I: DoubleIterator<Item = [f64; 2]>, F: FnOnce(&mut T) -> fmt::Result, T> Wrapper2<I, F, T> {
    fn new(it: I, func: F) -> Self {
        Wrapper2 {
            a: Some(it),
            b: None,
            func: Some(func),
            _p: PhantomData,
        }
    }
}

impl<D: DoubleIterator<Item = [f64; 2]>, F: FnOnce(&mut T) -> fmt::Result, T: fmt::Write> PlotTrait<T>
    for Wrapper2<D, F, T>
{
    fn write_name(&mut self, a: &mut T) -> fmt::Result {
        self.func.take().unwrap()(a)
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

struct Plot<'a, T> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<T> + 'a>,
}

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.

//Its important to note that most of the time when this library is used,
//every run through the code is first accompanied by one compilation of the code.
//So inefficiencies in dynamically allocating strings using format!() to then
//be just passed to a writer are not that bad seeing as the solution
//would involve passing a lot of closures around.
pub struct Plotter<'a, T> {
    writer: T,
    plots: Vec<Plot<'a, T>>,
}

///Convenience function for [`Plotter::new()`]
pub fn plot<'a, T: fmt::Write + 'a>(writer: T) -> Plotter<'a, T> {
    Plotter::new(writer)
}

///Convenience function for
///
/// Instead of this
/// ```
/// let plotter = poloto::plot(tagger::upgrade(std::io::stdout()));
/// ```
/// You can call this function like this
/// ```
/// let plotter = poloto::plot_io(std::io::stdout());
/// ```
pub fn plot_io<'a, T: std::io::Write + 'a>(writer: T) -> Plotter<'a, tagger::WriterAdaptor<T>> {
    Plotter::new(tagger::upgrade(writer))
}

impl<'a, T: fmt::Write + 'a> Plotter<'a, T> {
    /// Create a plotter
    ///
    /// # Example
    ///
    /// ```
    /// let mut s=String::new();
    /// let plotter = poloto::Plotter::new(&mut s);
    /// ```
    pub fn new(writer: T) -> Plotter<'a, T> {
        Plotter {
            writer,
            plots: Vec::new(),
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
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.line(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter())
    /// ```
    pub fn line(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
        })
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
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.line_fill(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter())
    /// ```
    pub fn line_fill(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
        })
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
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.scatter(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter())
    /// ```
    pub fn scatter(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
        })
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
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.histogram(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter())
    /// ```
    pub fn histogram(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
        })
    }

    ///You can override the css in regular html if you embed the generated svg.
    ///This gives you a lot of flexibility giving your the power to dynamically
    ///change the theme of your svg.
    ///
    ///However, if you want to embed the svg as an image, you lose this ability.
    ///If embedding as IMG is desired, instead the user can insert a custom style into the generated svg itself.
    ///
    ///All the plot functions don't actually add anything to the document until a  `render` function is called.
    ///So calls to this will append elements to the start of the document.
    ///
    pub fn render_no_default_tags(
        self,
        title: impl FnOnce(&mut T) -> fmt::Result,
        xname: impl FnOnce(&mut T) -> fmt::Result,
        yname: impl FnOnce(&mut T) -> fmt::Result,
    ) -> Result<T, fmt::Error> {
        let Plotter { mut writer, plots } = self;

        render::render(&mut writer, plots, title, xname, yname)?;
        Ok(writer)
    }

    pub fn render(
        self,
        title: impl FnOnce(&mut T) -> fmt::Result,
        xname: impl FnOnce(&mut T) -> fmt::Result,
        yname: impl FnOnce(&mut T) -> fmt::Result,
    ) -> Result<T, fmt::Error> {
        let Plotter { writer, plots } = self;

        default_tags::default_svg_and_styling(writer, |e| {
            render::render(e.get_writer(), plots, title, xname, yname)?;
            Ok(e)
        })
    }
}
