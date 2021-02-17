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

pub use render::HEIGHT;
pub use render::WIDTH;

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
    name: &'a str,
    plots: Box<dyn PlotTrait<'a> + 'a>,
}

struct PlotDecomp<'a> {
    plot_type: PlotType,
    name: &'a str,
    plots: Vec<[f32; 2]>,
}

//use tagger::element_borrow::Element;
//use tagger::element_move::FlatElement;
///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
///BELOW IMPORTANT:::
///We hold the writer for type inference for the _fmt functions
///We don't actually write anything to it until render() is called.
///This way you don't need to handle formatting errors in multipe places
///Only when you call render.

///Its important to note that most of the time when this library is used,
///if run through the code is first accompanied by one compilation of the code.
///So inefficiencies in dynamically allocating strings using format!() to then
///be just passed to a writer are not that bad seeing as the solution
///would involve passing a lot of closures around.
///Also plotting is used a lot by data scientists, and I wanted the code
///to be approachable to them.
pub struct Plotter<'a> {
    plots: Vec<Plot<'a>>,
    title: &'a str,
    xname: &'a str,
    yname: &'a str,
}

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
    pub fn line<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: &'a str, plots: I) {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            name,
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
    pub fn line_fill<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: &'a str, plots: I) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            name,
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
    pub fn scatter<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: &'a str, plots: I) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            name,
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
    pub fn histogram<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: &'a str, plots: I) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            name,
            plots: Box::new(Wrapper::new(plots.into_iter())),
        })
    }

    pub fn render<T: Write>(self, el: &mut tagger::elem::Single<T>) -> fmt::Result {
        render::render(self, el)
    }

    /*
    pub fn render<T: Write>(self, mut writer: T,func:impl FnOnce(&mut Element<T>)) -> fmt::Result {

        let mut svg=tagger::new_element!(
            &mut writer,
            "<svg class='poloto' height='{h}' width='{w}' viewBox='0 0 {w} {h}' xmlns='http://www.w3.org/2000/svg'>",
            w=render::WIDTH,
            h=render::HEIGHT)?;

            func(&mut svg);

        render::render(self, &mut svg)?;


        tagger::end!(svg,"</svg>")


    }

    ///Panics unlike other render functions.
    pub fn render_to_string(self,func:impl FnOnce(&mut Element<&mut String>)) -> Result<String, fmt::Error> {
        let mut s = String::new();
        self.render(&mut s,func)?;
        Ok(s)
    }
    */

    /*
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
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// // Make the line purple.
    /// plotter.append(svg::node::Text::new("<style>.poloto{--poloto_color0:purple;}</style>"));
    /// plotter.line("cow",data.iter().map(|&x|x));
    /// ```
    */
}


///Create the default SVG tag.
pub fn default_header<T:Write>(w:T)->Result<tagger::elem::ElementStack<T>,fmt::Error>{
    
    let svg=tagger::elem::ElementStack::new(w,format_args!("<svg class='poloto' height='{h}' width='{w}' viewBox='0 0 {w} {h}' xmlns='http://www.w3.org/2000/svg'>",
    w=render::WIDTH,
    h=render::HEIGHT),"</svg>")?;
    Ok(svg)
}
pub fn render_to_string(a: Plotter) -> Result<String, fmt::Error> {
    let mut s = String::new();
    render_svg(&mut s, a)?;
    Ok(s)
}
pub fn render_svg_io<T: std::io::Write>(writer: T, a: Plotter) -> fmt::Result {
    render_svg(tagger::upgrade(writer), a)
}
pub fn render_svg<T: Write>(writer: T, a: Plotter) -> fmt::Result {
    let mut stack=default_header(writer)?;
    a.render( &mut stack.borrow() )?;
    stack.finish()?;
    Ok(())
}
