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

///If [`plot`] isn't good enough, use this struct for more control.
pub struct PlotterBuilder<D: fmt::Display> {
    header: D,
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
            header: "",
            svgtag: true,
        }
    }
    pub fn with_header<J: Display>(self, header: J) -> PlotterBuilder<J> {
        PlotterBuilder {
            header,
            svgtag: self.svgtag,
        }
    }
}

impl<D: Display> PlotterBuilder<D> {
    pub fn build<'a, A: Display, B: Display, C: Display>(
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
                header: self.header,
            },
            plots: Vec::new(),
            svgtag,
        }
    }
}
