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
use core::marker::PhantomData;

use core::fmt::Write;


mod util;

mod render;

struct Wrapper<'a, I: IntoIterator<Item = [f32; 2]> + 'a>(Option<I>, PhantomData<&'a I>);

impl<'a, I: IntoIterator<Item = [f32; 2]> + 'a> PlotTrait<'a> for Wrapper<'a, I> {
    #[inline(always)]
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a> {
        Box::new(
            self.0
                .take()
                .unwrap()
                .into_iter()
                .filter(|[x, y]| !(x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite())),
        )
    }
}

trait PlotTrait<'a> {
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a>;
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a> {
    name: &'a str,
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<'a> + 'a>,
}

struct PlotDecomp<'a> {
    name: &'a str,
    plot_type: PlotType,
    plots: Vec<[f32; 2]>,
}

use tagger::element_move::FlatElement;

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
pub struct Plotter<'a,T:Write> {
    element:FlatElement<T>,
    title: &'a str,
    xname: &'a str,
    yname: &'a str,
    plots: Vec<Plot<'a>>
}

/// Shorthand constructor.
///
/// # Example
///
/// ```
/// let plotter = poloto::plot("Number of Cows per Year","Year","Cow");
/// ```
pub fn plot<'a,T:Write>(writer:T,title: &'a str, xname: &'a str, yname: &'a str) -> Plotter<'a,T> {
    Plotter::new(writer,title, xname, yname)
}

pub fn plot_io<'a,T:std::io::Write>(writer:T,title: &'a str, xname: &'a str, yname: &'a str) -> Plotter<'a,tagger::WriterAdaptor<T>> {
    Plotter::new(tagger::upgrade_writer(writer),title, xname, yname)
}

impl<'a,T:Write> Plotter<'a,T> {
    /// Create a plotter
    ///
    /// # Example
    ///
    /// ```
    /// let plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// ```
    pub fn new(writer:T,title: &'a str, xname: &'a str, yname: &'a str) -> Plotter<'a,T> {
        
        let root=tagger::root(writer);
        let svg=root.tag_build_flat("svg")
        .set("class","poloto")
        .set("height",render::HEIGHT)
        .set("width",render::WIDTH)
        .set("viewBox",format!("0 0 {} {}",render::WIDTH,render::HEIGHT))
        .set("xmlns","http://www.w3.org/2000/svg")
        .end();

        Plotter {
            element:svg,
            title,
            plots: Vec::new(),
            xname,
            yname
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
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
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
        name: &'a str,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            name,
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
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
        name: &'a str,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            name,
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
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
        name: &'a str,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            name,
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
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
    pub fn get_svg_element(&mut self)->&mut FlatElement<T>{
        &mut self.element
    }
    
    pub fn render(self){
        render::render(self);
    }

}
