//!
//! poloto - plot to SVG and style with CSS
//!
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
//! ### Usage
//!
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
//!
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
pub mod prelude {
    pub use core::fmt::Write;
    pub use tagger::wr;
}
use core::fmt;
mod render;

pub mod default_tags {
    use core::fmt;

    pub use super::render::default_styling;

    ///The class of the svg tag.
    pub const CLASS: &str = "poloto";
    ///The width of the svg tag.
    pub const WIDTH: f64 = 800.0;
    ///The height of the svg tag.
    pub const HEIGHT: f64 = 500.0;

    pub const XMLNS: &str = "http://www.w3.org/2000/svg";
    ///Returns a function that will write the attributes.
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

mod iter{
    pub struct TwiceIter<I>{
        inner:I
    }
    impl<I:Iterator+Clone > PlotIter for TwiceIter<I>{
        type Item=I::Item;
        fn first_iter(&mut self,mut func:impl FnMut(&I::Item)){
            for a in self.inner.clone(){
                func(&a);
            }
        }
        fn second_iter(&mut self,mut func:impl FnMut(&I::Item)){
            for a in &mut self.inner{
                func(&a);
            }
        }
    }
    /* TODO add this.
    pub struct FileBufferIter<I>{

    }
    */
    pub struct BufferIter<I:Iterator>{
        inner:I,
        buffer:Vec<I::Item>
    }

    impl<I:Iterator+Clone > PlotIter for BufferIter<I> {
        type Item=I::Item;
        fn first_iter(&mut self,mut func:impl FnMut(&I::Item)){
            for a in &mut self.inner{
                func(&a);
                self.buffer.push(a);
            }
        }
        fn second_iter(&mut self,mut func:impl FnMut(&I::Item)){
            for a in self.buffer.drain(..){
                func(&a);
            }
        }
    }

    pub trait PlotIter{
        type Item;
        fn first_iter(&mut self,func:impl FnMut(&Self::Item));
        fn second_iter(&mut self,func:impl FnMut(&Self::Item));
    }

    impl<I:IntoIterator> PlotIterAdaptor for I {}

    pub trait PlotIterAdaptor:IntoIterator+Sized {
        fn plot_buffer(self)->BufferIter<Self::IntoIter>{
            BufferIter{
                inner:self.into_iter(),
                buffer:Vec::new()
            }
        }
        fn plot_no_buffer(self)->TwiceIter<Self::IntoIter> where Self::IntoIter:Clone{
            TwiceIter{
                inner:self.into_iter()
            }
        }
    }
}


struct Wrapper<I, F, T> {
    it: Option<I>,
    func: Option<F>,
    _p: PhantomData<T>,
}
impl<I: Iterator<Item = [f64; 2]>, F: FnOnce(&mut T) -> fmt::Result, T> Wrapper<I, F, T> {
    fn new(it: I, func: F) -> Self {
        Wrapper {
            it: Some(it),
            func: Some(func),
            _p: PhantomData,
        }
    }
}

impl<'a, I: Iterator<Item = [f64; 2]> + 'a, F: FnOnce(&mut T) -> fmt::Result, T: fmt::Write>
    PlotTrait<'a, T> for Wrapper<I, F, T>
{
    fn write_name(&mut self, a: &mut T) -> fmt::Result {
        self.func.take().unwrap()(a)
    }
    fn iter(&mut self) -> Box<dyn Iterator<Item = [f64; 2]> + 'a> {
        Box::new(self.it.take().unwrap())
    }
}

trait PlotTrait<'a, T: Write> {
    fn write_name(&mut self, a: &mut T) -> fmt::Result;
    fn iter(&mut self) -> Box<dyn Iterator<Item = [f64; 2]> + 'a>;
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a, T> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<'a, T> + 'a>,
}

struct PlotDecomp<'a, T> {
    plot_type: PlotType,
    name_writer: Box<dyn PlotTrait<'a, T> + 'a>,
    plots: Vec<[f64; 2]>,
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
    plots: PlotData<'a, T>,
}

struct PlotData<'a, T>(Vec<Plot<'a, T>>);

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
            plots: PlotData(Vec::new()),
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
    /// use core::fmt::Write;
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.line(|w|write!(w,"cow"),data.iter().map(|&x|x))
    /// ```
    pub fn line(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl IntoIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.0.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(Wrapper::new(plots.into_iter(), name)),
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
    /// use core::fmt::Write;
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.line_fill(|w|write!(w,"cow"),data.iter().map(|&x|x))
    /// ```
    pub fn line_fill(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl IntoIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.0.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(Wrapper::new(plots.into_iter(), name)),
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
    /// use core::fmt::Write;
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.scatter(|w|write!(w,"cow"),data.iter().map(|&x|x))
    /// ```
    pub fn scatter(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl IntoIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.0.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(Wrapper::new(plots.into_iter(), name)),
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
    /// use core::fmt::Write;
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.histogram(|w|write!(w,"cow"),data.iter().map(|&x|x))
    /// ```
    pub fn histogram(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl IntoIterator<Item = [f64; 2]> + 'a,
    ) {
        self.plots.0.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(Wrapper::new(plots.into_iter(), name)),
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

/*
///Function to write to a T that implements `std::fmt::Write`
///Makes a svg tag with the defaults defined in [`default_svg_tag`].
pub fn render_svg<T: Write>(writer: T, a: Plotter) -> fmt::Result {
    let mut root = tagger::Element::new(writer);

    root.elem("svg", |writer| {
        let svg = writer.write(|w| {
            default_svg_tag::default()(w)?;
            Ok(w)
        })?;
        a.render(svg)
    })?;

    Ok(())
}

///Function to write to a T that implements `std::io::Write`
///Makes a svg tag with the defaults defined in [`default_svg_tag`].
pub fn render_svg_io<T: std::io::Write>(writer: T, a: Plotter) -> fmt::Result {
    render_svg(tagger::upgrade(writer), a)
}

///Convenience function to just write to a string.
pub fn render_to_string(a: Plotter) -> Result<String, fmt::Error> {
    let mut s = String::new();
    render_svg(&mut s, a)?;
    Ok(s)
}

*/
