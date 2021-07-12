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
mod build;
mod util;
pub use build::default_tags;
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

/// Default theme using css variables (with light theme defaults if the variables are not set).
pub const HTML_CONFIG_CSS_VARIABLE_DEFAULT: &str = r###"<style>.poloto {
    font-family: "Arial";
    stroke-width:2;
    }
    .poloto_text{fill: var(--poloto_fg_color,black);  }
    .poloto_axis_lines{stroke: var(--poloto_fg_color,black);stoke-width:3;fill:none}
    .poloto_background{fill: var(--poloto_bg_color,aliceblue); }
    .poloto0stroke{stroke:  var(--poloto_color0,blue); }
    .poloto1stroke{stroke:  var(--poloto_color1,red); }
    .poloto2stroke{stroke:  var(--poloto_color2,green); }
    .poloto3stroke{stroke:  var(--poloto_color3,gold); }
    .poloto4stroke{stroke:  var(--poloto_color4,aqua); }
    .poloto5stroke{stroke:  var(--poloto_color5,brown); }
    .poloto6stroke{stroke:  var(--poloto_color6,lime); }
    .poloto7stroke{stroke:  var(--poloto_color7,chocolate); }
    .poloto0fill{fill:var(--poloto_color0,blue);}
    .poloto1fill{fill:var(--poloto_color1,red);}
    .poloto2fill{fill:var(--poloto_color2,green);}
    .poloto3fill{fill:var(--poloto_color3,gold);}
    .poloto4fill{fill:var(--poloto_color4,aqua);}
    .poloto5fill{fill:var(--poloto_color5,brown);}
    .poloto6fill{fill:var(--poloto_color6,lime);}
    .poloto7fill{fill:var(--poloto_color7,chocolate);}</style>"###;

/// Default light theme
pub const HTML_CONFIG_LIGHT_DEFAULT: &str = r###"<style>.poloto {
    font-family: "Arial";
    stroke-width:2;
    }
    .poloto_text{fill: black;  }
    .poloto_axis_lines{stroke: black;stoke-width:3;fill:none}
    .poloto_background{fill: aliceblue; }
    .poloto0stroke{stroke:  blue; }
    .poloto1stroke{stroke:  red; }
    .poloto2stroke{stroke:  green; }
    .poloto3stroke{stroke:  gold; }
    .poloto4stroke{stroke:  aqua; }
    .poloto5stroke{stroke:  brown; }
    .poloto6stroke{stroke:  lime; }
    .poloto7stroke{stroke:  chocolate; }
    .poloto0fill{fill:blue;}
    .poloto1fill{fill:red;}
    .poloto2fill{fill:green;}
    .poloto3fill{fill:gold;}
    .poloto4fill{fill:aqua;}
    .poloto5fill{fill:brown;}
    .poloto6fill{fill:lime;}
    .poloto7fill{fill:chocolate;}</style>"###;

/// Default dark theme
pub const HTML_CONFIG_DARK_DEFAULT: &str = r###"<style>.poloto {
    font-family: "Arial";
    stroke-width:2;
    }
    .poloto_text{fill: white;  }
    .poloto_axis_lines{stroke: white;stoke-width:3;fill:none}
    .poloto_background{fill: black; }
    .poloto0stroke{stroke:  blue; }
    .poloto1stroke{stroke:  red; }
    .poloto2stroke{stroke:  green; }
    .poloto3stroke{stroke:  gold; }
    .poloto4stroke{stroke:  aqua; }
    .poloto5stroke{stroke:  brown; }
    .poloto6stroke{stroke:  lime; }
    .poloto7stroke{stroke:  chocolate; }
    .poloto0fill{fill:blue;}
    .poloto1fill{fill:red;}
    .poloto2fill{fill:green;}
    .poloto3fill{fill:gold;}
    .poloto4fill{fill:aqua;}
    .poloto5fill{fill:brown;}
    .poloto6fill{fill:lime;}
    .poloto7fill{fill:chocolate;}</style>"###;

/// Create a [`Plotter`] with the specified title,xname,yname, and custom html
/// Consider using some of the default html tags.
pub fn plot_with_html<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
    style: impl Display + 'a,
) -> Plotter<'a, impl Names> {
    build::PlotterBuilder::new()
        .with_header(style)
        .build(title, xname, yname)
}

/// Convenience function for `plot_with_html(title,xnam,yname,HTML_CONFIG_LIGHT_DEFAULT)`
pub fn plot<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a, impl Names> {
    plot_with_html(title, xname, yname, HTML_CONFIG_LIGHT_DEFAULT)
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

    /// When render is called, do not add the default svg tag at the
    /// start and end.
    pub fn without_svg(&mut self) -> &mut Self {
        self.svgtag = SvgTagOption::NoSvg;
        self
    }

    /// Render to a `String`
    pub fn render_to_string(self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        self.render(&mut s)?;
        Ok(s)
    }

    /// Render to a `std::io::Write`
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
                    let (svg, ()) = writer.write(|w| default_svg_attrs(w)?.empty_ok())?;

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
