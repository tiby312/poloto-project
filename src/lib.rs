//!
//! poloto - plot to SVG and style with CSS
//!
//! ### Usage
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! * Plots containing NaN or Infinity are ignored.
//! * After 6 plots, the colors cycle back and are repeated.
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

///Contains the [`DoubleIterator`] trait and three different
///implementers of it.
pub mod iter;

///Contains building blocks for create the default svg an styling tags from scratch.
pub mod default_tags {
    use core::fmt;

    pub use super::render::default_styling;
    pub use super::render::default_styling_variables;

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

        move |w| {
            w.attr("class", CLASS)?
                .attr("width", WIDTH)?
                .attr("height", HEIGHT)?
                .with_attr("viewBox", wr!("0 0 {} {}", WIDTH, HEIGHT))?
                .attr("xmlns", XMLNS)?;
            Ok(())
        }
    }
}

struct SvgData<T, F: FnOnce(&mut T) -> fmt::Result> {
    inner: Option<F>,
    _p: PhantomData<T>,
}
impl<T: fmt::Write, F: FnOnce(&mut T) -> fmt::Result> TextWriter<T> for SvgData<T, F> {
    fn write_name(&mut self, a: &mut T) -> fmt::Result {
        (self.inner.take().unwrap())(a)
    }
}

trait TextWriter<T: fmt::Write> {
    fn write_name(&mut self, a: &mut T) -> fmt::Result;
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

impl<D: DoubleIterator<Item = [f64; 2]>, F: FnOnce(&mut T) -> fmt::Result, T: fmt::Write>
    PlotTrait<T> for Wrapper2<D, F, T>
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
    data: Vec<Box<dyn TextWriter<T> + 'a>>,
    css_variables: bool,
    text_color: &'a str,
    back_color: &'a str,
    colors: [&'a str; render::NUM_COLORS],
    nostyle: bool,
    nosvgtag: bool,
}

///Convenience function for [`Plotter::new()`]
pub fn plot<'a, T: fmt::Write + 'a>(writer: T) -> Plotter<'a, T> {
    Plotter::new(writer)
}

///Convenience function for plotting with writers that only implement [`std::io::Write`].
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
            css_variables: false,
            text_color: "black",
            back_color: "aliceblue",
            colors: [
                "blue",
                "red",
                "green",
                "gold",
                "aqua",
                "brown",
                "lime",
                "chocolate",
            ],
            nostyle: false,
            nosvgtag: false,
            data: Vec::new(),
        }
    }

    /// Create a plotter with no outer svg tag. This is useful
    /// when you want to create your own svg tag with additional attributes.
    /// The default attributes can be retrived from the [`default_tags`] module.
    ///
    /// # Example
    ///
    /// ```
    /// let mut s=String::new();
    /// let plotter = poloto::Plotter::with_no_svg_tag(&mut s);
    /// ```
    pub fn with_no_svg_tag(writer: T) -> Plotter<'a, T> {
        let mut s = Plotter::new(writer);
        s.nosvgtag = true;
        s
    }

    /// Create a plotter with no outer svg tag or default style tag.
    /// The default style can be found in the [`default_tags`] module.
    ///
    /// # Example
    ///
    /// ```
    /// let mut s=String::new();
    /// let plotter = poloto::Plotter::with_no_svg_style_tags(&mut s);
    /// ```
    pub fn with_no_svg_style_tags(writer: T) -> Plotter<'a, T> {
        let mut s = Plotter::new(writer);
        s.nosvgtag = true;
        s.nostyle = true;
        s
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
    /// plotter.line(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn line(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
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
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.line_fill(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn line_fill(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
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
    /// let mut s=String::new();
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.scatter(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn scatter(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
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
    /// let mut plotter = poloto::Plotter::new(&mut s);
    /// plotter.histogram(|w|write!(w,"cow"),data.iter().map(|&x|x).twice_iter());
    /// ```
    pub fn histogram(
        &mut self,
        name: impl FnOnce(&mut T) -> fmt::Result + 'a,
        plots: impl DoubleIterator<Item = [f64; 2]> + 'a,
    ) -> &mut Self {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(Wrapper2::new(plots.into_iter(), name)),
        });
        self
    }
    /* Deliberately disable these. The user should use css to override the default colors.
        /// Hardcode into the svg the text colors.
        pub fn with_text_color(&mut self, s: &'a str) -> &mut Self {
            self.text_color = s;
            self
        }

        /// Hardcode into the svg the background colors.
        pub fn with_back_color(&mut self, s: &'a str) -> &mut Self {
            self.back_color = s;
            self
        }

        /// Hardcode into the svg the plot colors.
        pub fn with_plot_colors(&mut self, colors: &[&'a str; 8]) -> &mut Self {
            self.colors = *colors;
            self
        }
    */

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
    pub fn with_raw_text(&mut self, func: impl FnOnce(&mut T) -> fmt::Result + 'a) -> &mut Self {
        self.data.push(Box::new(SvgData {
            inner: Some(func),
            _p: PhantomData,
        }));
        self
    }

    /// Instead of the default style, use one that adds variables.
    ///
    /// This injects [`default_tags::default_styling_variables`] instead of
    /// the default [`default_tags::default_styling`].
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
    pub fn with_css_variables(&mut self) -> &mut Self {
        self.css_variables = true;
        self
    }

    /// Render the svg to the writer.
    ///
    /// Up until now, nothing has been written to the writer. We
    /// have just accumulated a list of commands and closures. This call will
    /// actually call all the closures and consume all the plot iterators.
    pub fn render<A, B, C>(self, title: A, xname: B, yname: C) -> Result<T, fmt::Error>
    where
        A: FnOnce(&mut T) -> fmt::Result,
        B: FnOnce(&mut T) -> fmt::Result,
        C: FnOnce(&mut T) -> fmt::Result,
    {
        let Plotter {
            writer,
            plots,
            css_variables,
            text_color,
            back_color,
            colors,
            nostyle,
            nosvgtag,
            data,
        } = self;
        let mut root = tagger::Element::new(writer);

        use default_tags::*;

        if nosvgtag {
            if !nostyle {
                if css_variables {
                    default_styling_variables(&mut root, text_color, back_color, colors)?;
                } else {
                    default_styling(&mut root, text_color, back_color, colors)?;
                }
            }

            render::render(root.get_writer(), data, plots, title, xname, yname)?;
        } else {
            root.elem("svg", |writer| {
                let mut svg = writer.write(|w| {
                    default_svg_attrs()(w)?;

                    Ok(w)
                })?;
                if !nostyle {
                    if css_variables {
                        default_styling_variables(&mut svg, text_color, back_color, colors)?;
                    } else {
                        default_styling(&mut svg, text_color, back_color, colors)?;
                    }
                }

                render::render(svg.get_writer(), data, plots, title, xname, yname)?;
                Ok(svg)
            })?;
        }
        Ok(root.into_writer())
    }
}
