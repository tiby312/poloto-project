//!
//! Build blocks for making custom plots
//!
use super::*;

///The number of unique colors.
pub const NUM_COLORS: usize = 8;

///Used internally to implement [`Names`]
pub struct NamesStruct<A, B, C, D> {
    title: A,
    xname: B,
    yname: C,
    header: D,
}
impl<A: Display, B: Display, C: Display, D: Display> Names for NamesStruct<A, B, C, D> {
    fn write_header(&self, fm: &mut fmt::Formatter) -> fmt::Result {
        self.header.fmt(fm)
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
pub trait Names {
    fn write_header(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_title(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_xname(&self, fm: &mut fmt::Formatter) -> fmt::Result;
    fn write_yname(&self, fm: &mut fmt::Formatter) -> fmt::Result;
}

///Contains building blocks for create the default svg an styling tags from scratch.
pub mod default_tags {
    use core::fmt;

    ///The class of the svg tag.
    pub const CLASS: &str = "poloto";
    ///The width of the svg tag.
    pub const WIDTH: f64 = 800.0;
    ///The height of the svg tag.
    pub const HEIGHT: f64 = 500.0;
    ///The xmlns: `http://www.w3.org/2000/svg`
    pub const XMLNS: &str = "http://www.w3.org/2000/svg";

    ///Write default svg tag attributes.
    pub fn default_svg_attrs<'a, 'b, T: fmt::Write>(
        w: &'a mut tagger::AttributeWriter<'b, T>,
    ) -> Result<&'a mut tagger::AttributeWriter<'b, T>, fmt::Error> {
        use tagger::prelude::*;

        w.attr("class", CLASS)?
            .attr("width", WIDTH)?
            .attr("height", HEIGHT)?
            .with_attr("viewBox", wr!("0 0 {} {}", WIDTH, HEIGHT))?
            .attr("xmlns", XMLNS)
    }
}
///Create a custom style poloto style
pub struct StyleBuilder<A, B, C> {
    text_color: A,
    back_color: B,
    colors: [C; NUM_COLORS],
}
impl Default for StyleBuilder<&'static str, &'static str, &'static str> {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleBuilder<&'static str, &'static str, &'static str> {
    pub fn new() -> Self {
        StyleBuilder {
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
        }
    }
}
impl<A: Display, B: Display, C: Display> StyleBuilder<A, B, C> {
    pub fn with_text_color<X: Display>(self, text_color: X) -> StyleBuilder<X, B, C> {
        StyleBuilder {
            text_color,
            back_color: self.back_color,
            colors: self.colors,
        }
    }
    pub fn with_back_color<X: Display>(self, back_color: X) -> StyleBuilder<A, X, C> {
        StyleBuilder {
            text_color: self.text_color,
            back_color,
            colors: self.colors,
        }
    }

    pub fn with_colors<X: Display>(self, colors: [X; NUM_COLORS]) -> StyleBuilder<A, B, X> {
        StyleBuilder {
            text_color: self.text_color,
            back_color: self.back_color,
            colors,
        }
    }
    pub fn build_with_css_variables(self) -> impl Display {
        let StyleBuilder {
            text_color,
            back_color,
            colors,
        } = self;
        moveable_format(move |w| {
            write!(
                w,
                r###"<style>.poloto {{
                    font-family: "Arial";
                    stroke-width:2;
                    }}
                    .poloto_text{{fill: var(--poloto_fg_color,{0});  }}
                    .poloto_axis_lines{{stroke: var(--poloto_fg_color,{0});stoke-width:3;fill:none}}
                    .poloto_background{{fill: var(--poloto_bg_color,{1}); }}
                    .poloto0stroke{{stroke:  var(--poloto_color0,{2}); }}
                    .poloto1stroke{{stroke:  var(--poloto_color1,{3}); }}
                    .poloto2stroke{{stroke:  var(--poloto_color2,{4}); }}
                    .poloto3stroke{{stroke:  var(--poloto_color3,{5}); }}
                    .poloto4stroke{{stroke:  var(--poloto_color4,{6}); }}
                    .poloto5stroke{{stroke:  var(--poloto_color5,{7}); }}
                    .poloto6stroke{{stroke:  var(--poloto_color6,{8}); }}
                    .poloto7stroke{{stroke:  var(--poloto_color7,{9}); }}
                    .poloto0fill{{fill:var(--poloto_color0,{2});}}
                    .poloto1fill{{fill:var(--poloto_color1,{3});}}
                    .poloto2fill{{fill:var(--poloto_color2,{4});}}
                    .poloto3fill{{fill:var(--poloto_color3,{5});}}
                    .poloto4fill{{fill:var(--poloto_color4,{6});}}
                    .poloto5fill{{fill:var(--poloto_color5,{7});}}
                    .poloto6fill{{fill:var(--poloto_color6,{8});}}
                    .poloto7fill{{fill:var(--poloto_color7,{9});}}</style>"###,
                text_color,
                back_color,
                colors[0],
                colors[1],
                colors[2],
                colors[3],
                colors[4],
                colors[5],
                colors[6],
                colors[7]
            )
        })
    }
    pub fn build(self) -> impl Display {
        let StyleBuilder {
            text_color,
            back_color,
            colors,
        } = self;
        moveable_format(move |w| {
            write!(
                w,
                r###"<style>.poloto {{
                font-family: "Arial";
                stroke-width:2;
                }}
                .poloto_text{{fill: {0};  }}
                .poloto_axis_lines{{stroke: {0};stoke-width:3;fill:none}}
                .poloto_background{{fill: {1}; }}
                .poloto0stroke{{stroke:  {2}; }}
                .poloto1stroke{{stroke:  {3}; }}
                .poloto2stroke{{stroke:  {4}; }}
                .poloto3stroke{{stroke:  {5}; }}
                .poloto4stroke{{stroke:  {6}; }}
                .poloto5stroke{{stroke:  {7}; }}
                .poloto6stroke{{stroke:  {8}; }}
                .poloto7stroke{{stroke:  {9}; }}
                .poloto0fill{{fill:{2};}}
                .poloto1fill{{fill:{3};}}
                .poloto2fill{{fill:{4};}}
                .poloto3fill{{fill:{5};}}
                .poloto4fill{{fill:{6};}}
                .poloto5fill{{fill:{7};}}
                .poloto6fill{{fill:{8};}}
                .poloto7fill{{fill:{9};}}</style>"###,
                text_color,
                back_color,
                colors[0],
                colors[1],
                colors[2],
                colors[3],
                colors[4],
                colors[5],
                colors[6],
                colors[7],
            )
        })
    }
}

///Insert svg data after the svg element, but before the plot elements.
pub struct DataBuilder<D: Display> {
    header: D,
}

impl Default for DataBuilder<&'static str> {
    fn default() -> Self {
        Self::new()
    }
}

impl DataBuilder<&'static str> {
    pub fn new() -> Self {
        DataBuilder { header: "" }
    }
}
impl<D: Display> DataBuilder<D> {
    ///Push the default poloto css styling.
    pub fn push_css_default(self) -> DataBuilder<impl Display> {
        DataBuilder {
            header: concatenate_display("", self.header, StyleBuilder::new().build()),
        }
    }

    /// Instead of the default style, use one that adds variables.
    ///
    /// This injects what is produced by [`StyleBuilder::build_with_css_variables`] instead of
    /// the default [`StyleBuilder::build`].
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
    pub fn push_default_css_with_variable(self) -> DataBuilder<impl Display> {
        DataBuilder {
            header: concatenate_display(
                "",
                self.header,
                StyleBuilder::new().build_with_css_variables(),
            ),
        }
    }
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
    pub fn push(self, a: impl fmt::Display) -> DataBuilder<impl Display> {
        DataBuilder {
            header: concatenate_display("", self.header, a),
        }
    }
    fn finish(self) -> D {
        self.header
    }
}

///If [`plot`] isn't good enough, use this struct for more control.
pub struct PlotterBuilder<D: fmt::Display> {
    data: DataBuilder<D>,
    svgtag: bool,
}
impl Default for PlotterBuilder<&'static str> {
    fn default() -> Self {
        Self::new()
    }
}
impl PlotterBuilder<&'static str> {
    pub fn new() -> Self {
        PlotterBuilder {
            data: DataBuilder::new(),
            svgtag: true,
        }
    }
    pub fn with_data<J: Display>(self, data: DataBuilder<J>) -> PlotterBuilder<J> {
        PlotterBuilder {
            data,
            svgtag: self.svgtag,
        }
    }
    pub fn with_svg(mut self, svg: bool) -> Self {
        self.svgtag = svg;
        self
    }
}

impl<'a, D: Display + 'a> PlotterBuilder<D> {
    pub fn build<A: Display + 'a, B: Display + 'a, C: Display + 'a>(
        self,
        title: A,
        xname: B,
        yname: C,
    ) -> Plotter<'a, NamesStruct<A, B, C, D>> {
        let svgtag = if self.svgtag {
            SvgTagOption::Svg
        } else {
            SvgTagOption::NoSvg
        };

        Plotter {
            names: NamesStruct {
                title,
                xname,
                yname,
                header: self.data.finish(),
            },
            plots: Vec::new(),
            svgtag,
        }
    }
}
