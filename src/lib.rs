//!
//! Plot to SVG and style with CSS
//!
//! You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
//! Documentation at [docs.rs](https://docs.rs/poloto)
//!
//! Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples).
//! The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.
//!
//!

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}

use std::fmt;

pub use tagger::upgrade_write;

pub mod plottable;
use plottable::Plottable;

mod render;
pub mod util;

pub mod plotnum;
use plotnum::*;
pub mod num;

///
/// The poloto prelude.
///
pub mod prelude {
    pub use super::formatm;
    pub use super::plottable::crop::Croppable;
    pub use super::SimpleTheme;
}

///The width of the svg tag.
const WIDTH: f64 = 800.0;
///The height of the svg tag.
const HEIGHT: f64 = 500.0;

trait PlotTrait<X: PlotNum, Y: PlotNum> {
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result;

    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X, Y)>;
}

use std::marker::PhantomData;

use fmt::Display;
struct PlotStruct<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)> + Clone, F: Display> {
    first: I,
    second: I,
    func: F,
    _p: PhantomData<(X, Y)>,
}

impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)> + Clone, F: Display>
    PlotStruct<X, Y, I, F>
{
    fn new(it: I, func: F) -> Self {
        let it2 = it.clone();
        PlotStruct {
            first: it,
            second: it2,
            func,
            _p: PhantomData,
        }
    }
}

impl<X: PlotNum, Y: PlotNum, D: Iterator<Item = (X, Y)> + Clone, F: Display> PlotTrait<X, Y>
    for PlotStruct<X, Y, D, F>
{
    fn write_name(&self, a: &mut dyn fmt::Write) -> fmt::Result {
        write!(a, "{}", self.func)
    }
    fn iter_first(&mut self) -> &mut dyn Iterator<Item = (X, Y)> {
        &mut self.first
    }

    fn iter_second(&mut self) -> &mut dyn Iterator<Item = (X, Y)> {
        &mut self.second
    }
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
    LineFillRaw,
}

struct Plot<'a, X: PlotNum, Y: PlotNum> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<X, Y> + 'a>,
}

///
/// Default SVG Header for a Poloto graph.
///
pub const SVG_HEADER: &str = r##"<svg class="poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"##;

///
/// Default SVG end tag.
///
pub const SVG_END: &str = "</svg>";

/// Default light theme
pub const STYLE_CONFIG_LIGHT_DEFAULT: &str = ".poloto { \
    stroke-linecap:round; \
    stroke-linejoin:round; \
    font-family: 'Tahoma', sans-serif; \
    background-color: AliceBlue;\
    } \
    .poloto_scatter{stroke-width:7} \
    .poloto_line{stroke-width:2} \
    .poloto_text{fill: black;} \
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none} \
    .poloto0stroke{stroke:  blue;} \
    .poloto1stroke{stroke:  red;} \
    .poloto2stroke{stroke:  green;} \
    .poloto3stroke{stroke:  gold;} \
    .poloto4stroke{stroke:  aqua;} \
    .poloto5stroke{stroke:  lime;} \
    .poloto6stroke{stroke:  orange;} \
    .poloto7stroke{stroke:  chocolate;} \
    .poloto0fill{fill:blue;} \
    .poloto1fill{fill:red;} \
    .poloto2fill{fill:green;} \
    .poloto3fill{fill:gold;} \
    .poloto4fill{fill:aqua;} \
    .poloto5fill{fill:lime;} \
    .poloto6fill{fill:orange;} \
    .poloto7fill{fill:chocolate;}";

/// Default dark theme
pub const STYLE_CONFIG_DARK_DEFAULT: &str = ".poloto { \
    stroke-linecap:round; \
    stroke-linejoin:round; \
    font-family: 'Tahoma', sans-serif; \
    background-color: #262626;\
    } \
    .poloto_scatter{stroke-width:7} \
    .poloto_line{stroke-width:2} \
    .poloto_text{fill: white;} \
    .poloto_axis_lines{stroke: white;stroke-width:3;fill:none;stroke-dasharray:none} \
    .poloto0stroke{stroke:  blue;} \
    .poloto1stroke{stroke:  red;} \
    .poloto2stroke{stroke:  green;} \
    .poloto3stroke{stroke:  gold;} \
    .poloto4stroke{stroke:  aqua;} \
    .poloto5stroke{stroke:  lime;} \
    .poloto6stroke{stroke:  orange;} \
    .poloto7stroke{stroke:  chocolate;} \
    .poloto0fill{fill:blue;} \
    .poloto1fill{fill:red;} \
    .poloto2fill{fill:green;} \
    .poloto3fill{fill:gold;} \
    .poloto4fill{fill:aqua;} \
    .poloto5fill{fill:lime;} \
    .poloto6fill{fill:orange;} \
    .poloto7fill{fill:chocolate;}";

/*
/// The demsions of the svg graph `[800,500]`.
pub const DIMENSIONS: [usize; 2] = [800, 500];
*/

///
/// Create a Plotter
///
pub fn plot<'a, X: PlotNum, Y: PlotNum>(
    title: impl PlotterXnameTrait<X, Y> + 'a,
    xname: impl PlotterXnameTrait<X, Y> + 'a,
    yname: impl PlotterXnameTrait<X, Y> + 'a,
) -> Plotter<'a, X, Y> {
    Plotter::new(title, xname, yname, default_tick(), default_tick())
}

#[derive(Copy, Clone)]
pub struct Canvas {
    ideal_num_xsteps: u32,
    ideal_num_ysteps: u32,
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
    aspect_offset: f64,
    scalex2: f64,
    scaley2: f64,
    spacing: f64,
    legendx1: f64,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
}
impl Canvas {
    pub fn new() -> Self {
        Self::with_options(false, None)
    }
    pub fn with_options(preserve_aspect: bool, num_css_classes: Option<usize>) -> Self {
        let ideal_num_xsteps = if preserve_aspect { 4 } else { 6 };

        let ideal_num_ysteps = 5;

        let width = crate::WIDTH as f64;
        let height = crate::HEIGHT as f64;
        let padding = 150.0;
        let paddingy = 100.0;

        let aspect_offset = if preserve_aspect {
            width / 2.0 - height + paddingy * 2.0
        } else {
            0.0
        };

        //The range over which the data will be scaled to fit
        let scalex2 = if preserve_aspect {
            height - paddingy * 2.0
        } else {
            width - padding * 2.0
        };

        let scaley2 = height - paddingy * 2.0;

        let spacing = padding / 3.0;
        let legendx1 = width - padding / 1.2 + padding / 30.0;

        Canvas {
            ideal_num_xsteps,
            ideal_num_ysteps,
            width,
            height,
            padding,
            paddingy,
            aspect_offset,
            scalex2,
            scaley2,
            spacing,
            legendx1,
            num_css_classes,
            preserve_aspect,
        }
    }

    pub fn gen_ticks<X: PlotNum, Y: PlotNum>(
        &self,
        plotter: &PlotterRes<X, Y>,
    ) -> TickResult<X, Y> {
        let tickx = X::compute_ticks(
            self.ideal_num_xsteps,
            plotter.boundx,
            DashInfo {
                ideal_dash_size: 30.0,
                max: self.scalex2,
            },
        );

        let ticky = Y::compute_ticks(
            self.ideal_num_ysteps,
            plotter.boundy,
            DashInfo {
                ideal_dash_size: 30.0,
                max: self.scaley2,
            },
        );

        TickResult { tickx, ticky }
    }

    pub fn render<X: PlotNum, Y: PlotNum>(
        &self,
        writer: impl std::fmt::Write,
        mut plotter: PlotterRes<X, Y>,
        ticks: TickResult<X, Y>,
    ) -> std::fmt::Result {
        let Canvas {
            width,
            height,
            padding,
            paddingy,
            aspect_offset,
            scalex2,
            scaley2,
            spacing,
            legendx1,
            num_css_classes,
            ..
        } = *self;

        let [minx, maxx] = plotter.boundx;
        let [miny, maxy] = plotter.boundy;

        let mut writer = tagger::new(writer);

        let mut color_iter = {
            let max = if let Some(nn) = num_css_classes {
                nn
            } else {
                usize::MAX
            };

            (0..max).cycle()
        };

        for (i, mut p) in plotter.plots.drain(..).enumerate() {
            let legendy1 = paddingy - padding / 8.0 + (i as f64) * spacing;

            let name_exists = writer
                .elem("text", |d| {
                    d.attr("class", "poloto_text poloto_legend_text")?;
                    d.attr("alignment-baseline", "middle")?;
                    d.attr("text-anchor", "start")?;
                    d.attr("font-size", "large")?;
                    d.attr("x", width - padding / 1.2)?;
                    d.attr("y", paddingy + (i as f64) * spacing)
                })?
                .build(|d| {
                    let mut wc = num::WriteCounter::new(d.writer_safe());
                    p.plots.write_name(&mut wc)?;
                    Ok(wc.get_counter() != 0)
                })?;

            let aa = minx.scale([minx, maxx], scalex2);
            let bb = miny.scale([miny, maxy], scaley2);

            struct PlotIter<X, Y> {
                basex_ii: f64,
                basey_ii: f64,
                rangex_ii: [X; 2],
                rangey_ii: [Y; 2],
                maxx_ii: f64,
                maxy_ii: f64,
            }
            impl<X: PlotNum, Y: PlotNum> PlotIter<X, Y> {
                fn gen_iter<'a>(
                    &'a self,
                    p: &'a mut Plot<X, Y>,
                ) -> impl Iterator<Item = [f64; 2]> + 'a {
                    p.plots.iter_second().map(move |(x, y)| {
                        [
                            self.basex_ii + x.scale(self.rangex_ii, self.maxx_ii),
                            self.basey_ii - y.scale(self.rangey_ii, self.maxy_ii),
                        ]
                    })
                }
            }

            let plot_iter = PlotIter {
                basex_ii: aspect_offset + padding - aa,
                basey_ii: height - paddingy + bb,
                rangex_ii: [minx, maxx],
                rangey_ii: [miny, maxy],
                maxx_ii: scalex2,
                maxy_ii: scaley2,
            };

            let colori = color_iter.next().unwrap();

            match p.plot_type {
                PlotType::Line => {
                    if name_exists {
                        writer.single("line", |d| {
                            d.attr(
                                "class",
                                format_args!(
                                    "poloto_line poloto_legend_icon poloto{}stroke poloto{}legend",
                                    colori, colori
                                ),
                            )?;
                            d.attr("stroke", "black")?;
                            d.attr("x1", legendx1)?;
                            d.attr("x2", legendx1 + padding / 3.0)?;
                            d.attr("y1", legendy1)?;
                            d.attr("y2", legendy1)
                        })?;
                    }

                    writer.single("path", |d| {
                        d.attr("class", format_args!("poloto_line poloto{}stroke", colori))?;
                        d.attr("fill", "none")?;
                        d.attr("stroke", "black")?;
                        d.path(|a| render::line(a, plot_iter.gen_iter(&mut p)))
                    })?;
                }
                PlotType::Scatter => {
                    if name_exists {
                        writer.single("line", |d| {
                            d.attr(
                                "class",
                                format_args!(
                                    "poloto_scatter poloto_legend_icon poloto{}stroke poloto{}legend",
                                    colori, colori
                                ),
                            )?;
                            d.attr("stroke", "black")?;
                            d.attr("x1", legendx1 + padding / 30.0)?;
                            d.attr("x2", legendx1 + padding / 30.0)?;
                            d.attr("y1", legendy1)?;
                            d.attr("y2", legendy1)
                        })?;
                    }

                    writer.single("path", |d| {
                        d.attr(
                            "class",
                            format_args!("poloto_scatter poloto{}stroke", colori),
                        )?;
                        d.path(|a| {
                            use tagger::PathCommand::*;
                            for [x, y] in plot_iter
                                .gen_iter(&mut p)
                                .filter(|&[x, y]| x.is_finite() && y.is_finite())
                            {
                                a.put(M(x, y))?;
                                a.put(H_(0))?;
                            }
                            Ok(())
                        })
                    })?;
                }
                PlotType::Histo => {
                    if name_exists {
                        writer.single("rect", |d| {
                            d.attr(
                                "class",
                                format_args!(
                                    "poloto_histo poloto_legend_icon poloto{}fill poloto{}legend",
                                    colori, colori
                                ),
                            )?;
                            d.attr("x", legendx1)?;
                            d.attr("y", legendy1 - padding / 30.0)?;
                            d.attr("width", padding / 3.0)?;
                            d.attr("height", padding / 20.0)?;
                            d.attr("rx", padding / 30.0)?;
                            d.attr("ry", padding / 30.0)
                        })?;
                    }

                    writer
                        .elem("g", |d| {
                            d.attr("class", format_args!("poloto_histo poloto{}fill", colori))
                        })?
                        .build(|writer| {
                            let mut last = None;
                            //TODO dont necesarily filter?
                            for [x, y] in plot_iter
                                .gen_iter(&mut p)
                                .filter(|&[x, y]| x.is_finite() && y.is_finite())
                            {
                                if let Some((lx, ly)) = last {
                                    writer.single("rect", |d| {
                                        d.attr("x", lx)?;
                                        d.attr("y", ly)?;
                                        d.attr(
                                            "width",
                                            (padding * 0.02).max((x - lx) - (padding * 0.02)),
                                        )?;
                                        d.attr("height", height - paddingy - ly)
                                    })?;
                                }
                                last = Some((x, y))
                            }
                            Ok(())
                        })?;
                }
                PlotType::LineFill => {
                    if name_exists {
                        writer.single("rect", |d| {
                            d.attr(
                                "class",
                                format_args!(
                                    "poloto_linefill poloto_legend_icon poloto{}fill poloto{}legend",
                                    colori, colori
                                ),
                            )?;
                            d.attr("x", legendx1)?;
                            d.attr("y", legendy1 - padding / 30.0)?;
                            d.attr("width", padding / 3.0)?;
                            d.attr("height", padding / 20.0)?;
                            d.attr("rx", padding / 30.0)?;
                            d.attr("ry", padding / 30.0)
                        })?;
                    }

                    writer.single("path", |d| {
                        d.attr(
                            "class",
                            format_args!("poloto_linefill poloto{}fill", colori),
                        )?;
                        d.path(|path| {
                            render::line_fill(
                                path,
                                plot_iter.gen_iter(&mut p),
                                height - paddingy,
                                true,
                            )
                        })
                    })?;
                }
                PlotType::LineFillRaw => {
                    if name_exists {
                        writer.single("rect", |d| {
                            d.attr(
                                "class",
                                format_args!(
                                    "poloto_linefillraw poloto_legend_icon poloto{}fill poloto{}legend",
                                    colori, colori
                                ),
                            )?;
                            d.attr("x", legendx1)?;
                            d.attr("y", legendy1 - padding / 30.0)?;
                            d.attr("width", padding / 3.0)?;
                            d.attr("height", padding / 20.0)?;
                            d.attr("rx", padding / 30.0)?;
                            d.attr("ry", padding / 30.0)
                        })?;
                    }

                    writer.single("path", |d| {
                        d.attr(
                            "class",
                            format_args!("poloto_linefillraw poloto{}fill", colori),
                        )?;
                        d.path(|path| {
                            render::line_fill(
                                path,
                                plot_iter.gen_iter(&mut p),
                                height - paddingy,
                                false,
                            )
                        })
                    })?;
                }
            }
        }

        self.draw_base(&mut writer, plotter, ticks)?;

        Ok(())
    }

    pub fn draw_base<X: PlotNum, Y: PlotNum, T: fmt::Write>(
        &self,
        writer: &mut tagger::ElemWriter<T>,
        mut plotter: PlotterRes<X, Y>,
        ticks: TickResult<X, Y>,
    ) -> std::fmt::Result {
        let Canvas {
            width,
            height,
            padding,
            paddingy,
            aspect_offset,
            scalex2,
            scaley2,
            preserve_aspect,
            ..
        } = *self;

        let boundx = plotter.boundx;
        let boundy = plotter.boundy;
        let [minx, maxx] = boundx;
        let [miny, maxy] = boundy;

        let xtick_info = ticks.tickx;
        let ytick_info = ticks.ticky;

        let texty_padding = paddingy * 0.3;
        let textx_padding = padding * 0.1;

        writer
            .elem("text", |d| {
                d.attr("class", "poloto_labels poloto_text poloto_title")?;
                d.attr("alignment-baseline", "start")?;
                d.attr("text-anchor", "middle")?;
                d.attr("font-size", "x-large")?;
                d.attr("x", width / 2.0)?;
                d.attr("y", padding / 4.0)
            })?
            .build(|w| {
                plotter.title.fmt_self(Data {
                    writer: &mut w.writer_safe(),
                    boundx,
                    boundy,
                    stepx: xtick_info.unit_data,
                    stepy: ytick_info.unit_data,
                })
            })?;

        writer
            .elem("text", |d| {
                d.attr("class", "poloto_labels poloto_text poloto_xname")?;
                d.attr("alignment-baseline", "start")?;
                d.attr("text-anchor", "middle")?;
                d.attr("font-size", "x-large")?;
                d.attr("x", width / 2.0)?;
                d.attr("y", height - padding / 8.)
            })?
            .build(|w| {
                plotter.xname.fmt_self(Data {
                    writer: &mut w.writer_safe(),
                    boundx,
                    boundy,
                    stepx: xtick_info.unit_data,
                    stepy: ytick_info.unit_data,
                })
            })?;

        writer
            .elem("text", |d| {
                d.attr("class", "poloto_labels poloto_text poloto_yname")?;
                d.attr("alignment-baseline", "start")?;
                d.attr("text-anchor", "middle")?;
                d.attr("font-size", "x-large")?;
                d.attr(
                    "transform",
                    format_args!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
                )?;
                d.attr("x", padding / 4.0)?;
                d.attr("y", height / 2.0)
            })?
            .build(|w| {
                plotter.yname.fmt_self(Data {
                    writer: &mut w.writer_safe(),
                    boundx,
                    boundy,
                    stepx: xtick_info.unit_data,
                    stepy: ytick_info.unit_data,
                })
            })?;

        let xdash_size: Option<f64> = if plotter.dash_x {
            xtick_info.dash_size
        } else {
            None
        };
        let ydash_size: Option<f64> = if plotter.dash_y {
            ytick_info.dash_size
        } else {
            None
        };

        use tagger::PathCommand::*;

        let first_tickx = xtick_info.ticks[0];

        let first_ticky = ytick_info.ticks[0];

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(base) = xtick_info.display_relative {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "middle")?;
                        d.attr("text-anchor", "start")?;
                        d.attr("x", width * 0.55)?;
                        d.attr("y", paddingy * 0.7)
                    })?
                    .build(|d| {
                        let mut w = d.writer_safe();
                        use std::fmt::Write;
                        write!(w, "Where j = ")?;

                        plotter.xtick_fmt.fmt_self(
                            base,
                            DataSingle {
                                writer: &mut w,
                                bound: boundx,
                                ff: FmtFull::Full,
                                step: xtick_info.unit_data,
                            },
                        )
                    })?;

                "j+"
            } else {
                ""
            };

            //Draw interva`l x text
            for &Tick { position, value } in xtick_info.ticks.iter() {
                let xx = (position.scale([minx, maxx], scalex2)
                    - minx.scale([minx, maxx], scalex2))
                    + padding;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", aspect_offset + xx)?;
                    d.attr("x2", aspect_offset + xx)?;
                    d.attr("y1", height - paddingy)?;
                    d.attr("y2", height - paddingy * 0.95)
                })?;

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "start")?;
                        d.attr("text-anchor", "middle")?;
                        d.attr("x", aspect_offset + xx)?;
                        d.attr("y", height - paddingy + texty_padding)
                    })?
                    .build(|w| {
                        let mut w = w.writer_safe();
                        use std::fmt::Write;
                        write!(w, "{}", extra)?;

                        plotter.xtick_fmt.fmt_self(
                            value,
                            DataSingle {
                                writer: &mut w,
                                bound: boundx,
                                ff: FmtFull::Short,
                                step: xtick_info.unit_data,
                            },
                        )
                        /*
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.xcontext.fmt_tick(
                                w,
                                value,
                                xtick_info.unit_data,
                                FmtFull::Tick
                            ))
                        ))
                        */
                    })?;
            }
        }

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(base) = ytick_info.display_relative {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "middle")?;
                        d.attr("text-anchor", "start")?;
                        d.attr("x", padding)?;
                        d.attr("y", paddingy * 0.7)
                    })?
                    .build(|w| {
                        use std::fmt::Write;
                        let mut w = w.writer_safe();
                        write!(w, "Where k = ")?;

                        plotter.ytick_fmt.fmt_self(
                            base,
                            DataSingle {
                                writer: &mut w,
                                bound: boundy,
                                ff: FmtFull::Full,
                                step: ytick_info.unit_data,
                            },
                        )
                    })?;

                "k+"
            } else {
                ""
            };

            //Draw interval y text
            for &Tick { position, value } in ytick_info.ticks.iter() {
                let yy = height
                    - (position.scale([miny, maxy], scaley2) - miny.scale([miny, maxy], scaley2))
                    - paddingy;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")?;
                    d.attr("stroke", "black")?;
                    d.attr("x1", aspect_offset + padding)?;
                    d.attr("x2", aspect_offset + padding * 0.96)?;
                    d.attr("y1", yy)?;
                    d.attr("y2", yy)
                })?;

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_tick_labels poloto_text")?;
                        d.attr("alignment-baseline", "middle")?;
                        d.attr("text-anchor", "end")?;
                        d.attr("x", aspect_offset + padding - textx_padding)?;
                        d.attr("y", yy)
                    })?
                    .build(|w| {
                        let mut w = w.writer_safe();
                        use std::fmt::Write;
                        write!(w, "{}", extra)?;

                        plotter.ytick_fmt.fmt_self(
                            value,
                            DataSingle {
                                writer: &mut w,
                                bound: boundy,
                                ff: FmtFull::Short,
                                step: ytick_info.unit_data,
                            },
                        )

                        /*
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.ycontext.fmt_tick(
                                w,
                                value,
                                ytick_info.unit_data,
                                FmtFull::Tick
                            )) //TODO need a way to communicate writing base
                        ))
                        */
                    })?;
            }
        }

        let d1 = minx.scale([minx, maxx], scalex2);
        let d2 = first_tickx.position.scale([minx, maxx], scalex2);
        let distance_to_firstx = d2 - d1;

        writer.single("path", |d| {
            d.attr("stroke", "black")?;
            d.attr("fill", "none")?;
            d.attr("class", "poloto_axis_lines")?;
            if let Some(xdash_size) = xdash_size {
                d.attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        xdash_size / 2.0,
                        -distance_to_firstx
                    ),
                )?;
            }
            d.path(|p| {
                p.put(M(padding + aspect_offset, height - paddingy))?;
                if preserve_aspect {
                    p.put(L(
                        height - paddingy / 2.0 + aspect_offset,
                        height - paddingy,
                    ))
                } else {
                    p.put(L(width - padding + aspect_offset, height - paddingy))
                }
            })
        })?;

        let d1 = miny.scale([miny, maxy], scaley2);
        let d2 = first_ticky.position.scale([miny, maxy], scaley2);
        let distance_to_firsty = d2 - d1;

        writer.single("path", |d| {
            d.attr("stroke", "black")?;
            d.attr("fill", "none")?;
            d.attr("class", "poloto_axis_lines")?;
            if let Some(ydash_size) = ydash_size {
                d.attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        ydash_size / 2.0,
                        -distance_to_firsty
                    ),
                )?;
            }
            d.path(|p| {
                p.put(M(aspect_offset + padding, height - paddingy))?;
                p.put(L(aspect_offset + padding, paddingy))
            })
        })?;

        Ok(())
    }
}

pub trait PlotterTickTrait<X: PlotNum> {
    fn fmt_self(&mut self, val: X, data: DataSingle<X>) -> std::fmt::Result;
}

pub fn default_tick<X: PlotNum>() -> impl PlotterTickTrait<X> {
    tick_ext(|mut v: X, mut d| v.val_fmt(d.writer, d.ff, &mut d.step))
}
pub fn tick_ext<X: PlotNum>(
    func: impl FnMut(X, DataSingle<X>) -> std::fmt::Result,
) -> impl PlotterTickTrait<X> {
    impl<X: PlotNum, F> PlotterTickTrait<X> for F
    where
        F: FnMut(X, DataSingle<X>) -> std::fmt::Result,
    {
        fn fmt_self(&mut self, val: X, data: DataSingle<X>) -> std::fmt::Result {
            (self)(val, data)
        }
    }

    func
}

pub trait PlotterXnameTrait<X: PlotNum, Y: PlotNum> {
    fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result;
}

impl<T: std::fmt::Display, X: PlotNum, Y: PlotNum> PlotterXnameTrait<X, Y> for T {
    fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result {
        write!(data.writer, "{}", self)
    }
}

pub fn name_ext<X: PlotNum, Y: PlotNum, F: FnMut(Data<X, Y>) -> std::fmt::Result>(
    func: F,
) -> impl PlotterXnameTrait<X, Y> {
    pub struct NoDisp<F>(pub F);

    impl<X: PlotNum, Y: PlotNum, F> PlotterXnameTrait<X, Y> for NoDisp<F>
    where
        F: FnMut(Data<X, Y>) -> std::fmt::Result,
    {
        fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result {
            (self.0)(data)
        }
    }

    NoDisp(func)
}

pub struct DataSingle<'a, X: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub bound: [X; 2],
    pub step: X::StepInfo,
    pub ff: FmtFull,
}
pub struct Data<'a, X: PlotNum, Y: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub boundx: [X; 2],
    pub boundy: [Y; 2],
    pub stepx: X::StepInfo,
    pub stepy: Y::StepInfo,
}

pub struct TickResult<X: PlotNum, Y: PlotNum> {
    pub tickx: TickInfo<X>,
    pub ticky: TickInfo<Y>,
}

pub struct PlotterRes<'a, X: PlotNum, Y: PlotNum> {
    plots: Vec<Plot<'a, X, Y>>,
    boundx: [X; 2],
    boundy: [Y; 2],
    title: Box<dyn PlotterXnameTrait<X, Y> + 'a>,
    xname: Box<dyn PlotterXnameTrait<X, Y> + 'a>,
    yname: Box<dyn PlotterXnameTrait<X, Y> + 'a>,
    xtick_fmt: Box<dyn PlotterTickTrait<X> + 'a>,
    ytick_fmt: Box<dyn PlotterTickTrait<Y> + 'a>,
    dash_x: bool,
    dash_y: bool,
}

/// Keeps track of plots.
/// User supplies iterators that will be iterated on when
/// render is called.
///
/// * The svg element belongs to the `poloto` css class.
/// * The title,xname,yname,legend text SVG elements belong to the `poloto_text` class.
/// * The axis line SVG elements belong to the `poloto_axis_lines` class.
/// * The background belongs to the `poloto_background` class.
///
pub struct Plotter<'a, X: PlotNum + 'a, Y: PlotNum + 'a> {
    title: Box<dyn PlotterXnameTrait<X, Y> + 'a>,
    xname: Box<dyn PlotterXnameTrait<X, Y> + 'a>,
    yname: Box<dyn PlotterXnameTrait<X, Y> + 'a>,
    xtick_fmt: Box<dyn PlotterTickTrait<X> + 'a>,
    ytick_fmt: Box<dyn PlotterTickTrait<Y> + 'a>,
    plots: Vec<Plot<'a, X, Y>>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
    dash_x: bool,
    dash_y: bool,
}

impl<'a, X: PlotNum, Y: PlotNum> Plotter<'a, X, Y> {
    ///
    /// Create a plotter with the specified element.
    ///
    /// ```
    /// let mut p = poloto::Plotter::new("title", "x", "y");
    /// p.line("",[[1,1]]);
    /// ```
    pub fn new(
        title: impl PlotterXnameTrait<X, Y> + 'a,
        xname: impl PlotterXnameTrait<X, Y> + 'a,
        yname: impl PlotterXnameTrait<X, Y> + 'a,
        xtick_fmt: impl PlotterTickTrait<X> + 'a,
        ytick_fmt: impl PlotterTickTrait<Y> + 'a,
    ) -> Plotter<'a, X, Y> {
        Plotter {
            title: Box::new(title),
            xname: Box::new(xname),
            yname: Box::new(yname),
            plots: Vec::new(),
            xmarkers: Vec::new(),
            ymarkers: Vec::new(),
            num_css_classes: Some(8),
            preserve_aspect: false,
            xtick_fmt: Box::new(xtick_fmt),
            ytick_fmt: Box::new(ytick_fmt),
            dash_x: true,
            dash_y: true,
        }
    }
    /// Create a line from plots using a SVG polyline element.
    /// The element belongs to the `.poloto[N]stroke` css class.
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
        I::Item: Plottable<X, Y>,
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

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
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
        I::Item: Plottable<X, Y>,
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

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line_fill_raw("", &data);
    /// ```
    pub fn line_fill_raw<I>(&mut self, name: impl Display + 'a, plots: I) -> &mut Self
    where
        I: IntoIterator,
        I::IntoIter: Clone + 'a,
        I::Item: Plottable<X, Y>,
    {
        self.plots.push(Plot {
            plot_type: PlotType::LineFillRaw,
            plots: Box::new(PlotStruct::new(
                plots.into_iter().map(|x| x.make_plot()),
                name,
            )),
        });
        self
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
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
        I::Item: Plottable<X, Y>,
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

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
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
        I::Item: Plottable<X, Y>,
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

    pub fn find_bounds(&mut self) -> PlotterRes<'a, X, Y> {
        let mut pp = self.move_into();

        let (boundx, boundy) = num::find_bounds(
            pp.plots.iter_mut().flat_map(|x| x.plots.iter_first()),
            pp.xmarkers,
            pp.ymarkers,
        );

        PlotterRes {
            title: pp.title,
            xname: pp.xname,
            yname: pp.yname,
            plots: pp.plots,
            xtick_fmt: pp.xtick_fmt,
            ytick_fmt: pp.ytick_fmt,
            boundx,
            boundy,
            dash_x: pp.dash_x,
            dash_y: pp.dash_y,
        }
    }

    ///
    /// Preserve the aspect ratio by drawing a smaller graph in the same area.
    ///
    pub fn preserve_aspect(&mut self) -> &mut Self {
        self.preserve_aspect = true;
        self
    }

    ///
    /// The number of distinct css classes. If there are more plots than
    /// classes, then they will wrap around. The default value is 8.
    ///
    /// A value of None, means it will never wrap around.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// plotter.num_css_class(Some(30));
    /// ```
    ///
    pub fn num_css_class(&mut self, a: Option<usize>) -> &mut Self {
        self.num_css_classes = a;
        self
    }

    pub fn xmarker(&mut self, a: X) -> &mut Self {
        self.xmarkers.push(a);
        self
    }

    pub fn ymarker(&mut self, a: Y) -> &mut Self {
        self.ymarkers.push(a);
        self
    }

    pub fn no_dash_x(&mut self) -> &mut Self {
        self.dash_x = false;
        self
    }
    pub fn no_dash_y(&mut self) -> &mut Self {
        self.dash_y = false;
        self
    }

    ///
    /// Move a plotter out from behind a mutable reference leaving
    /// an empty plotter.
    ///
    pub fn move_into(&mut self) -> Plotter<'a, X, Y> {
        let mut empty = crate::plot("", "", "");
        core::mem::swap(&mut empty, self);
        empty
    }

    ///
    /// Use the plot iterators to write out the graph elements.
    /// Does not add a svg tag, or any styling elements.
    /// Use this if you want to embed a svg into your html.
    /// You will just have to add your own svg sag and then supply styling.
    ///
    /// Panics if the render fails.
    ///
    /// In order to meet a more flexible builder pattern, instead of consuming the Plotter,
    /// this function will mutable borrow the Plotter and leave it with empty data.
    ///
    /// ```
    /// let data = [[1.0,4.0], [2.0,5.0], [3.0,6.0]];
    /// let mut plotter = poloto::plot("title", "x", "y");
    /// plotter.line("", &data);
    /// let mut k=String::new();
    /// plotter.render(&mut k);
    /// ```
    pub fn render<T: std::fmt::Write>(&mut self, a: T) -> fmt::Result {
        let data = self.find_bounds();

        //knowldge of canvas dim
        let canvas = crate::Canvas::new();

        //compute step info
        let ticks = canvas.gen_ticks(&data);

        canvas.render(a, data, ticks)
    }
}

pub trait SimpleTheme {
    fn simple_theme<T: fmt::Write>(&mut self, a: T) -> std::fmt::Result;
    fn simple_theme_dark<T: fmt::Write>(&mut self, a: T) -> std::fmt::Result;
}

impl<X: PlotNum, Y: PlotNum> SimpleTheme for Plotter<'_, X, Y> {
    fn simple_theme<T: std::fmt::Write>(&mut self, mut a: T) -> std::fmt::Result {
        write!(
            &mut a,
            "{}<style>{}</style>{}{}",
            SVG_HEADER,
            STYLE_CONFIG_LIGHT_DEFAULT,
            disp(|a| self.render(a)),
            SVG_END
        )
    }

    fn simple_theme_dark<T: std::fmt::Write>(&mut self, mut a: T) -> std::fmt::Result {
        write!(
            &mut a,
            "{}<style>{}</style>{}{}",
            SVG_HEADER,
            STYLE_CONFIG_DARK_DEFAULT,
            disp(|a| self.render(a)),
            SVG_END
        )
    }
}

/// Shorthand for `moveable_format(move |w|write!(w,...))`
/// Similar to `format_args!()` except has a more flexible lifetime.
#[macro_export]
macro_rules! formatm {
    ($($arg:tt)*) => {
        $crate::DisplayableClosure::new(move |w| write!(w,$($arg)*))
    }
}

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
pub struct DisplayableClosure<F>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> DisplayableClosure<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosure(a)
    }
}
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for DisplayableClosure<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(formatter)
    }
}

///
/// Leverage rust's display format system using `RefCell` under the hood.
///
pub fn disp<F: FnOnce(&mut fmt::Formatter) -> fmt::Result>(a: F) -> DisplayableClosureOnce<F> {
    DisplayableClosureOnce::new(a)
}

pub fn disp_const<F: Fn(&mut fmt::Formatter) -> fmt::Result>(a: F) -> DisplayableClosure<F> {
    DisplayableClosure::new(a)
}

use std::cell::RefCell;

///
/// Wrap a mutable closure in a `RefCell` to allow it to be called inside of `fmt::Display::fmt`
///
pub struct DisplayableClosureOnce<F>(pub RefCell<Option<F>>);

impl<F: FnOnce(&mut fmt::Formatter) -> fmt::Result> DisplayableClosureOnce<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosureOnce(RefCell::new(Some(a)))
    }
}
impl<F: FnOnce(&mut fmt::Formatter) -> fmt::Result> fmt::Display for DisplayableClosureOnce<F> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if let Some(f) = (self.0.borrow_mut()).take() {
            (f)(formatter)
        } else {
            Ok(())
        }
    }
}
