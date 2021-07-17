//!
//! Plot to SVG and style with CSS
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! * Plots containing NaN or Infinity are ignored.
//! * After 8 plots, the colors cycle back and are repeated.
//!

mod render;
mod util;

use num_traits::AsPrimitive;

use std::fmt;

///The number of unique colors.
const NUM_COLORS: usize = 8;

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

///Used internally to implement [`Names`]
struct NamesStruct<A, B, C, D, E, F> {
    title: A,
    xname: B,
    yname: C,
    header: D,
    body: E,
    footer: F,
}
impl<A: Display, B: Display, C: Display, D: Display, E: Display, F: Display> Names
    for NamesStruct<A, B, C, D, E, F>
{
    fn write_header(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.header.fmt(fm)
    }
    fn write_body(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.body.fmt(fm)
    }
    fn write_footer(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.footer.fmt(fm)
    }
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

///Used internally to write out the header/title/xname/yname.
trait Names {
    fn write_header(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_body(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_footer(&self, fm: &mut fmt::Formatter) -> fmt::Result;

    fn write_title(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_xname(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_yname(&self, fm: &mut fmt::Formatter) -> fmt::Result;
}

trait PlotTrait {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result;
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]>;
}

use fmt::Display;
struct PlotStruct<I: Iterator<Item = [f64; 2]> + Clone, F: Display> {
    first: I,
    second: I,
    func: F,
}

impl<I: Iterator<Item = [f64; 2]> + Clone, F: Display> PlotStruct<I, F> {
    fn new(it: I, func: F) -> Self {
        let it2 = it.clone();
        PlotStruct {
            first: it,
            second: it2,
            func,
        }
    }
}

impl<D: Iterator<Item = [f64; 2]> + Clone, F: Display> PlotTrait for PlotStruct<D, F> {
    fn write_name(&self, a: &mut fmt::Formatter) -> fmt::Result {
        self.func.fmt(a)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = [f64; 2]> {
        &mut self.first
    }

    fn iter_second(&mut self) -> &mut dyn Iterator<Item = [f64; 2]> {
        &mut self.second
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

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
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
pub const HTML_CONFIG_CSS_VARIABLE_DEFAULT: &str = "<style>.poloto {\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .poloto_text{fill: var(--poloto_fg_color,black);}\
    .poloto_axis_lines{stroke: var(--poloto_fg_color,black);stoke-width:3;fill:none}\
    .poloto_background{fill: var(--poloto_bg_color,aliceblue);}\
    .poloto0stroke{stroke:  var(--poloto_color0,blue);}\
    .poloto1stroke{stroke:  var(--poloto_color1,red);}\
    .poloto2stroke{stroke:  var(--poloto_color2,green);}\
    .poloto3stroke{stroke:  var(--poloto_color3,gold);}\
    .poloto4stroke{stroke:  var(--poloto_color4,aqua);}\
    .poloto5stroke{stroke:  var(--poloto_color5,brown);}\
    .poloto6stroke{stroke:  var(--poloto_color6,lime);}\
    .poloto7stroke{stroke:  var(--poloto_color7,chocolate);}\
    .poloto0fill{fill:var(--poloto_color0,blue);}\
    .poloto1fill{fill:var(--poloto_color1,red);}\
    .poloto2fill{fill:var(--poloto_color2,green);}\
    .poloto3fill{fill:var(--poloto_color3,gold);}\
    .poloto4fill{fill:var(--poloto_color4,aqua);}\
    .poloto5fill{fill:var(--poloto_color5,brown);}\
    .poloto6fill{fill:var(--poloto_color6,lime);}\
    .poloto7fill{fill:var(--poloto_color7,chocolate);}</style>";

/// Default light theme
pub const HTML_CONFIG_LIGHT_DEFAULT: &str = "<style>.poloto {\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .poloto_text{fill: black;}\
    .poloto_axis_lines{stroke: black;stoke-width:3;fill:none}\
    .poloto_background{fill: aliceblue;}\
    .poloto0stroke{stroke:  blue;}\
    .poloto1stroke{stroke:  red;}\
    .poloto2stroke{stroke:  green;}\
    .poloto3stroke{stroke:  gold;}\
    .poloto4stroke{stroke:  aqua;}\
    .poloto5stroke{stroke:  brown;}\
    .poloto6stroke{stroke:  lime;}\
    .poloto7stroke{stroke:  chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:brown;}\
    .poloto6fill{fill:lime;}\
    .poloto7fill{fill:chocolate;}</style>";

/// Default dark theme
pub const HTML_CONFIG_DARK_DEFAULT: &str = "<style>.poloto {\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .poloto_text{fill: white;}\
    .poloto_axis_lines{stroke: white;stoke-width:3;fill:none}\
    .poloto_background{fill: black;}\
    .poloto0stroke{stroke:  blue;}\
    .poloto1stroke{stroke:  red;}\
    .poloto2stroke{stroke:  green;}\
    .poloto3stroke{stroke:  gold;}\
    .poloto4stroke{stroke:  aqua;}\
    .poloto5stroke{stroke:  brown;}\
    .poloto6stroke{stroke:  lime;}\
    .poloto7stroke{stroke:  chocolate;}\
    .poloto0fill{fill:blue;}\
    .poloto1fill{fill:red;}\
    .poloto2fill{fill:green;}\
    .poloto3fill{fill:gold;}\
    .poloto4fill{fill:aqua;}\
    .poloto5fill{fill:brown;}\
    .poloto6fill{fill:lime;}\
    .poloto7fill{fill:chocolate;}</style>";

/// The default SVG Header tag
pub const SVG_HEADER_DEFAULT: &str = r###"<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"###;
/// The default SVG Header: attributes only.
pub const SVG_HEADER_DEFAULT_WITHOUT_TAG: &str = r###"class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg""###;

/// The default SVG ending tag.
pub const SVG_FOOTER_DEFAULT: &str = "</svg>";

/// Iterators that are passed to the [`Plotter`] plot functions must produce
/// items that implement this trait.
pub trait Plottable {
    /// Produce one plot
    fn make_plot(self) -> [f64; 2];
}

impl<T: AsPrimitive<f64>> Plottable for [T; 2] {
    fn make_plot(self) -> [f64; 2] {
        let [x, y] = self;
        [x.as_(), y.as_()]
    }
}

impl<T: AsPrimitive<f64>> Plottable for &[T; 2] {
    fn make_plot(self) -> [f64; 2] {
        let [x, y] = self;
        [x.as_(), y.as_()]
    }
}

/// Shorthand for `plot_with_html_raw(title,xname,yname,SVG_HEADER_DEFAULT,body,SVG_FOOT_DEFAULT);`
pub fn plot_with_html<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
    body: impl Display + 'a,
) -> Plotter<'a> {
    Plotter {
        names: Box::new(NamesStruct {
            title,
            xname,
            yname,
            header: SVG_HEADER_DEFAULT,
            body,
            footer: SVG_FOOTER_DEFAULT,
        }),
        plots: Vec::new(),
    }
}

/// Create a [`Plotter`] with the specified title,xname,yname, and custom header,body, and footer.
/// Consider using some of the default html tags.
pub fn plot_with_html_raw<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
    header: impl Display + 'a,
    body: impl Display + 'a,
    footer: impl Display + 'a,
) -> Plotter<'a> {
    Plotter {
        names: Box::new(NamesStruct {
            title,
            xname,
            yname,
            header,
            body,
            footer,
        }),
        plots: Vec::new(),
    }
}

/// Shorthand for `plot_with_html(title,xname,yname,HTML_CONFIG_LIGHT_DEFAULT);`
pub fn plot<'a>(
    title: impl Display + 'a,
    xname: impl Display + 'a,
    yname: impl Display + 'a,
) -> Plotter<'a> {
    plot_with_html(title, xname, yname, HTML_CONFIG_LIGHT_DEFAULT)
}

/// Keeps track of plots.
/// User supplies iterators that will be iterated on when
/// render is called.
pub struct Plotter<'a> {
    names: Box<dyn Names + 'a>,
    plots: Vec<Plot<'a>>,
}

impl<'a> Plotter<'a> {
    /// Create a line from plots.
    /// Can be stylized using the `.poloto[N]stroke` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// ```
    pub fn line<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a line from plots that will be filled underneath.
    /// Can be stylized using the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line_fill("", &data);
    /// ```
    pub fn line_fill<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a scatter plot from plots. Can be stylized using the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.scatter("", &data);
    /// ```
    pub fn scatter<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a histogram from plots.
    /// Each bar's left side will line up with a point. Can be stylized using the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.histogram("", &data);
    /// ```
    pub fn histogram<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable,
    {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Render to a `String`
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let s:String = plotter.render_to_string().unwrap();
    /// ```
    pub fn render_to_string(self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        self.render(&mut s)?;
        Ok(s)
    }

    /// Render to a `std::io::Write`
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// plotter.render_io(std::io::stdout()).unwrap();
    /// ```
    pub fn render_io<T: std::io::Write>(self, writer: T) -> fmt::Result {
        self.render(tagger::upgrade(writer))
    }

    /// Render the svg to the writer.
    ///
    /// Up until now, nothing has been written to the writer. We
    /// have just accumulated a list of commands and closures. This call will
    /// actually call all the closures and consume all the plot iterators.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let mut s = String::new();
    /// plotter.render(&mut s).unwrap();
    /// ```
    pub fn render<T: fmt::Write>(self, writer: T) -> fmt::Result {
        let mut root = tagger::Element::new(writer);

        render::render(root.get_writer(), self)?;

        Ok(())
    }
}
