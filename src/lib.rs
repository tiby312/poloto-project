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

mod util;

use core::fmt;
mod render;

/// [`render_svg`] creates its own svg tag, and then calls [`Plotter::render`].
/// The default attributes set in that svg tag are in this module.
pub mod default_svg_tag {
    use core::fmt;

    ///The class of the svg tag.
    pub const CLASS: &str = "poloto";
    ///The width of the svg tag.
    pub const WIDTH: f32 = 800.0;
    ///The height of the svg tag.
    pub const HEIGHT: f32 = 500.0;

    ///Returns a function that will write the attributes.
    pub fn default<T: fmt::Write>(
    ) -> impl FnOnce(&mut tagger::AttributeWriter<T>) -> Result<(), fmt::Error> {
        use tagger::prelude::*;

        |w| {
            w.attr("class", CLASS)?
                .attr("width", WIDTH)?
                .attr("height", HEIGHT)?
                .with_attr("viewBox", wr!("0 0 {} {}", WIDTH, HEIGHT))?
                .attr("xmlns", "http://www.w3.org/2000/svg")?;
            Ok(())
        }
    }
}

struct Wrapper<I: Iterator<Item = [f32; 2]>> {
    it: I,
}
impl<'a, I: Iterator<Item = [f32; 2]> + 'a> Wrapper<I> {
    fn new(it: I) -> Self {
        Wrapper { it }
    }
}

impl<'a, I: Iterator<Item = [f32; 2]> + 'a> PlotTrait<'a> for Wrapper<I> {
    #[inline(always)]
    fn get_iter_mut(&mut self) -> &mut (dyn Iterator<Item = [f32; 2]> + 'a) {
        &mut self.it
    }
}

trait PlotTrait<'a> {
    fn get_iter_mut(&mut self) -> &mut (dyn Iterator<Item = [f32; 2]> + 'a);
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a> {
    plot_type: PlotType,
    name: String,
    plots: Box<dyn PlotTrait<'a> + 'a>,
}

struct PlotDecomp {
    plot_type: PlotType,
    name: String,
    plots: Vec<[f32; 2]>,
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
    plots: Vec<Plot<'a>>,
    title: &'a str,
    xname: &'a str,
    yname: &'a str,
}

///Convenience function for [`Plotter::new()`]
pub fn plot<'a>(title: &'a str, xname: &'a str, yname: &'a str) -> Plotter<'a> {
    Plotter::new(title, xname, yname)
}

impl<'a> Plotter<'a> {
    /// Create a plotter
    ///
    /// # Example
    ///
    /// ```
    /// let plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// ```
    pub fn new(title: &'a str, xname: &'a str, yname: &'a str) -> Plotter<'a> {
        Plotter {
            plots: Vec::new(),
            title,
            xname,
            yname,
        }
    }

    /// Create a line from plots.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.line("cow",data.iter().map(|&x|x))
    /// ```
    pub fn line<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: impl ToString, plots: I) {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            name: name.to_string(),
            plots: Box::new(Wrapper::new(plots.into_iter())),
        })
    }

    /// Create a line from plots that will be filled underneath.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.line_fill("cow",data.iter().map(|&x|x))
    /// ```
    pub fn line_fill<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            name: name.to_string(),
            plots: Box::new(Wrapper::new(plots.into_iter())),
        })
    }

    /// Create a scatter plot from plots.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.scatter("cow",data.iter().map(|&x|x))
    /// ```
    pub fn scatter<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            name: name.to_string(),
            plots: Box::new(Wrapper::new(plots.into_iter())),
        })
    }

    /// Create a histogram from plots.
    /// Each bar's left side will line up with a point
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.histogram("cow",data.iter().map(|&x|x))
    /// ```
    pub fn histogram<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            name: name.to_string(),
            plots: Box::new(Wrapper::new(plots.into_iter())),
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
    /// # Example
    ///
    /// ```
    /// fn main()->std::fmt::Result{
    ///     use tagger::prelude::*;
    ///     let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    ///     ];
    ///     let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    ///     plotter.line("cow",data.iter().map(|&x|x));
    ///
    ///     let mut buffer=String::new();
    ///     let mut root=tagger::Element::new(&mut buffer);
    ///
    ///     root.elem("svg",|writer|{
    ///         let svg=writer.write(|w|{
    ///             poloto::default_svg_tag::default()(w)?;
    ///             Ok(w)
    ///         })?;
    ///
    ///         // Make the line purple.
    ///         svg.elem_no_attr("style",|w|{
    ///             write_ret!(w,"{}","<style>.poloto{--poloto_color0:purple;}</style>")
    ///         })?;
    ///     
    ///         plotter.render(svg)
    ///     })?;
    ///     println!("{}",buffer);
    ///     Ok(())
    /// }
    /// ```
    pub fn render<T: Write>(
        self,
        el: &mut tagger::Element<T>,
    ) -> Result<&mut tagger::Element<T>, fmt::Error> {
        render::render(self, el)
    }
}

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

///Convenience function to just write to a string.
pub fn render_to_string(a: Plotter) -> Result<String, fmt::Error> {
    let mut s = String::new();
    render_svg(&mut s, a)?;
    Ok(s)
}
