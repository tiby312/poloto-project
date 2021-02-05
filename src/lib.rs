//!
//! plotato - A very simple plotter
//!
//! The rise of SVG has made the need to an intermediate representatin of a graph
//! not necessary. Instead of using .gplot files, just render to SVG graph directly
//! using this crate. 
//!
//! plotato is meant to be used to graph fairly 'obvious/general' trends
//! in the data. The user is not meant to be able to extract data
//! or notice subtle differences in data. The desired use case is
//! to allow inserting a nice graph into a webpage generated via
//! [mdBook](https://rust-lang.github.io/mdBook/).  
//!
//! For this reason, a lot
//! of options that are normally provided in a plotting library
//! aren't provided. For example, you can't change the thickness
//! of the lines. Instead the user is encouraged, to pick a good
//! set of data points to "zoom in" on a trend they want to show.
//!
//! ### How do I change the color of the plots?
//!
//! You can doing it by overriding the css. If you embed the generated svg into a html file,
//! then you can add this example css styling to override the default in the svg file:
//! ```css
//! .plotato{
//!    font-family: "Open Sans", sans-serif;
//!    --fg_color:"black";
//!    --bg_color:"white;
//!    --plot_color0:"red";
//!    --plot_color1:"green";
//!    --plot_color2:"yellow";
//!    --plot_color3:"orange";
//!    --plot_color4:"purple";
//!    --plot_color5:"pink";
//!    --plot_color6:"aqua";
//! }
//! ```  
//! The default coloring uses a css class selector, so using a id select overrides it.
//! Using this method, you can have a specific themes for each mdBook theme.
//!
//! ### Usage
//!
//! * Plots containing NaN or Infinity are ignored.
//!
//! ### Why use scientific notation?
//!
//! Its the most dense and consistent formatting. Also easiest to implement.
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
use svg::node;
use svg::node::element;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Polyline;
use svg::Document;

mod util;

mod render;

struct Wrapper<'a, I: Iterator<Item = [f32; 2]> + Clone + 'a>(Option<I>, PhantomData<&'a I>);

impl<'a, I: Iterator<Item = [f32; 2]> + Clone + 'a> PlotTrait<'a> for Wrapper<'a, I> {
    #[inline(always)]
    fn ref_iter(&self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a> {
        Box::new(
            self.0
                .as_ref()
                .unwrap()
                .clone()
                .filter(|[x, y]| !(x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite())),
        )
    }

    #[inline(always)]
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a> {
        Box::new(
            self.0
                .take()
                .unwrap()
                .filter(|[x, y]| !(x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite())),
        )
    }
}

trait PlotTrait<'a> {
    fn ref_iter(&self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a>;
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a>;
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a> {
    name: String,
    plots: Box<dyn PlotTrait<'a> + 'a>,
    plot_type: PlotType,
}

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
///Each Iterator will be iterated through twice by doing one call to clone().
///once to find min max bounds, second to construct plot    
pub struct Plotter<'a> {
    title: String,
    xname: String,
    yname: String,
    plots: Vec<Plot<'a>>,
    style:Option<element::Style>
}

///Shorthand constructor.
pub fn plot<'a>(title: impl ToString, xname: impl ToString, yname: impl ToString) -> Plotter<'a> {
    Plotter::new(title, xname, yname)
}

impl<'a> Plotter<'a> {
    ///Create a plotter
    pub fn new(title: impl ToString, xname: impl ToString, yname: impl ToString) -> Plotter<'a> {
        Plotter {
            title: title.to_string(),
            plots: Vec::new(),
            xname: xname.to_string(),
            yname: yname.to_string(),
            style:None
        }
    }

    ///You can override the css in regular html if you embed the generated svg.
    ///This gives you a lot of flexibility giving your the power to dynamically
    ///change the theme of your svg.
    ///
    ///However, if you want to embed the svg as an image, you lose this ability.
    ///If embedding as IMG is desired, instead the user can insert a custom style into the generated svg itself.
    pub fn custom_style(&mut self, text:impl ToString){
        self.style=Some(element::Style::new(text.to_string()));
    }


    pub fn line<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    pub fn line_fill<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }


    pub fn scatter<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    ///Each bar's left side will line up with a point
    pub fn histogram<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    pub fn into_document(self) -> Document {
        render::render(self)
    }

    pub fn render_to_file(self, filename: &str) -> Result<(), std::io::Error> {
        let doc = render::render(self);
        svg::save(filename, &doc)
    }

    pub fn render<T: std::io::Write>(self, target: T) -> Result<(), std::io::Error> {
        let doc = render::render(self);
        svg::write(target, &doc)
    }

}
