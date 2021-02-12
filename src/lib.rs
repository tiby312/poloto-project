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
//!    --poloto_color8:"blue";
//! }
//! ```  
//! By default these variables are not defined, so the svg falls back on some default colors.
//! With this, it will change the colors. 
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
//! ### How do I do I make the histogram corners rounded?
//!
//! You can't right now, but with SVG2 with Geometry Properties, you'll be able to do.
//!
//! ```css
//! .poloto2stroke{rx:5;ry:5}
//! ```
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
    name: String,
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<'a> + 'a>,
}

struct PlotDecomp {
    name: String,
    plot_type: PlotType,
    plots: Vec<[f32; 2]>,
}

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
pub struct Plotter<'a> {
    title: String,
    xname: String,
    yname: String,
    plots: Vec<Plot<'a>>,
    doc: Document,
}

///Shorthand constructor.
pub fn plot<'a>(title: impl ToString, xname: impl ToString, yname: impl ToString) -> Plotter<'a> {
    Plotter::new(title, xname, yname)
}

impl<'a> Plotter<'a> {
    ///Create a plotter
    pub fn new(title: impl ToString, xname: impl ToString, yname: impl ToString) -> Plotter<'a> {
        let doc = Document::new()
            .set("width", render::WIDTH)
            .set("height", render::HEIGHT)
            .set("viewBox", (0, 0, render::WIDTH, render::HEIGHT))
            .set("class", "poloto");

        Plotter {
            title: title.to_string(),
            plots: Vec::new(),
            xname: xname.to_string(),
            yname: yname.to_string(),
            doc,
        }
    }

    pub fn line<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: impl ToString, plots: I) {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    pub fn line_fill<I: IntoIterator<Item = [f32; 2]> + 'a>(
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

    pub fn scatter<I: IntoIterator<Item = [f32; 2]> + 'a>(
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
    pub fn histogram<I: IntoIterator<Item = [f32; 2]> + 'a>(
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

    ///You can override the css in regular html if you embed the generated svg.
    ///This gives you a lot of flexibility giving your the power to dynamically
    ///change the theme of your svg.
    ///
    ///However, if you want to embed the svg as an image, you lose this ability.
    ///If embedding as IMG is desired, instead the user can insert a custom style into the generated svg itself.
    ///
    ///All the plot functions don't actually add anything to the document until a  `render` function is called.
    ///So calls to this will append elements to the start of the document.
    pub fn append<N: svg::Node>(&mut self, a: N) {
        use svg::Node;
        self.doc.append(a);
    }

    pub fn render_to_document(self) -> Document {
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
