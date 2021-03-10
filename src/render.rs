use super::*;
use tagger::prelude::*;

pub const NUM_COLORS: usize = 8;

use core::fmt::Display;

pub fn default_styling_variables(
    text_color: impl Display,
    background_color: impl Display,
    colors: [impl Display;NUM_COLORS],
) -> impl Display {
    moveable_format(move |w|{
        write!(w,
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
                background_color,
                colors[0],
                colors[1],
                colors[2],
                colors[3],
                colors[4],
                colors[5],
                colors[6],
                colors[7])
        
    })
}


///Add the default css styling.
pub fn default_styling(
    text_color: impl Display,
    background_color: impl Display,
    colors: [impl Display;NUM_COLORS],
) -> impl Display {
    //Default colors if CSS is not overriden with user colors.

    moveable_format(move |w|{

        write!(
            w,
            r###".poloto {{
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
            .poloto7fill{{fill:{9};}}"###,
            text_color,
            background_color,
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

//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub(super) fn render<'a, 'x, T: Write>(
    mut writer: &'x mut T,
    data: Vec<Box<dyn Display + 'a>>,
    mut plots: Vec<Plot<'a>>,
    names:Box<dyn Names+'a>
) -> Result<&'x mut T, fmt::Error> {
    for a in data.into_iter() {
        write!(writer,"{}",a)?;
    }
    use super::default_tags::*;
    let width = WIDTH;
    let height = HEIGHT;
    let padding = 150.0;
    let paddingy = 100.0;

    let svg = &mut tagger::Element::new(&mut writer);

    svg.single("rect", |w| {
        w.attr("class", "poloto_background")?
            .attr("fill", "white")?
            .attr("x", 0)?
            .attr("y", 0)?
            .attr("width", width)?
            .attr("height", height)
    })?;

    //Find range.
    let [minx, maxx, miny, maxy] =
        if let Some(m) = util::find_bounds(plots.iter_mut().flat_map(|x| x.plots.iter_first())) {
            m
        } else {
            //TODO test that this looks ok
            return Ok(writer); //No plots at all. don't need to draw anything
        };

    const EPSILON: f64 = f64::MIN_POSITIVE * 10.0;

    //Insert a range if the range is zero.
    let [miny, maxy] = if (maxy - miny).abs() < EPSILON {
        [miny - 1.0, miny + 1.0]
    } else {
        [miny, maxy]
    };

    //Insert a range if the range is zero.
    let [minx, maxx] = if (maxx - minx).abs() < EPSILON {
        [minx - 1.0, minx + 1.0]
    } else {
        [minx, maxx]
    };

    let scalex = (width - padding * 2.0) / (maxx - minx);
    let scaley = (height - paddingy * 2.0) / (maxy - miny);

    {
        //Draw step lines
        //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

        let ideal_num_xsteps = 9;
        let ideal_num_ysteps = 10;

        let texty_padding = paddingy * 0.3;
        let textx_padding = padding * 0.1;

        let (xstep_num, xstep, xstart_step) = util::find_good_step(ideal_num_xsteps, [minx, maxx]);
        let (ystep_num, ystep, ystart_step) = util::find_good_step(ideal_num_ysteps, [miny, maxy]);

        let distance_to_firstx = xstart_step - minx;

        let distance_to_firsty = ystart_step - miny;

        {
            //step num is assured to be atleast 1.
            let (extra, xstart_step) = if crate::util::determine_if_should_use_strat(
                xstart_step,
                xstart_step + ((xstep_num - 1) as f64) * xstep,
                xstep,
            )? {
                svg.elem("text", |writer| {
                    let text = writer.write(|w| {
                        w.attr("class", "poloto_text")?
                            .attr("alignment-baseline", "middle")?
                            .attr("text-anchor", "start")?
                            .attr("x", width * 0.55)?
                            .attr("y", paddingy * 0.7)
                    })?;
                    write!(text, "Where j = ")?;

                    crate::util::interval_float(text, xstart_step, None)?; //Some(xstep)
                    Ok(text)
                })?;

                ("j+", 0.0)
            } else {
                ("", xstart_step)
            };

            //Draw interva`l x text
            for a in 0..xstep_num {
                let p = (a as f64) * xstep;

                let xx = (distance_to_firstx + p) * scalex + padding;

                svg.single("line", |w| {
                    w.attr("class", "poloto_axis_lines")?
                        .attr("stroke", "black")?
                        .attr("x1", xx)?
                        .attr("x2", xx)?
                        .attr("y1", height - paddingy)?
                        .attr("y2", height - paddingy * 0.95) //TODO operations of order?
                })?;

                svg.elem("text", |writer| {
                    let text = writer.write(|w| {
                        w.attr("class", "poloto_text")?
                            .attr("alignment-baseline", "start")?
                            .attr("text-anchor", "middle")?
                            .attr("x", xx)?
                            .attr("y", height - paddingy + texty_padding)
                    })?;
                    write!(text, "{}", extra)?;

                    util::interval_float(text, p + xstart_step, Some(xstep))?;
                    Ok(text)
                })?;
            }
        }

        {
            //step num is assured to be atleast 1.
            let (extra, ystart_step) = if crate::util::determine_if_should_use_strat(
                ystart_step,
                ystart_step + ((ystep_num - 1) as f64) * ystep,
                ystep,
            )? {
                svg.elem("text", |writer| {
                    let text = writer.write(|w| {
                        w.attr("class", "poloto_text")?
                            .attr("alignment-baseline", "middle")?
                            .attr("text-anchor", "start")?
                            .attr("x", padding)?
                            .attr("y", paddingy * 0.7)
                    })?;
                    write!(text, "Where k = ")?;

                    crate::util::interval_float(text, ystart_step, None)?; //Some(ystep)

                    Ok(text)
                })?;

                ("k+", 0.0)
            } else {
                ("", ystart_step)
            };

            //Draw interval y text
            for a in 0..ystep_num {
                let p = (a as f64) * ystep;

                let yy = height - (distance_to_firsty + p) * scaley - paddingy;

                svg.single("line", |w| {
                    w.attr("class", "poloto_axis_lines")?
                        .attr("stroke", "black")?
                        .attr("x1", padding)?
                        .attr("x2", padding * 0.96)?
                        .attr("y1", yy)?
                        .attr("y2", yy)
                })?;

                svg.elem("text", |writer| {
                    let text = writer.write(|w| {
                        w.attr("class", "poloto_text")?
                            .attr("alignment-baseline", "middle")?
                            .attr("text-anchor", "end")?
                            .attr("x", padding - textx_padding)?
                            .attr("y", yy)
                    })?;
                    write!(text, "{}", extra)?;

                    util::interval_float(text, p + ystart_step, Some(ystep))?;
                    Ok(text)
                })?;
            }
        }
    }

    for (
        i,
        colori,
        Plot {
            plot_type,
            mut plots,
        },
    ) in plots
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i, i % NUM_COLORS, x))
    {
        let spacing = padding / 3.0;

        //TODO how to check for this???
        //if !name.is_empty() {
        svg.elem("text", |writer| {
            let text = writer.write(|w| {
                w.attr("class", "poloto_text")?
                    .attr("alignment-baseline", "middle")?
                    .attr("text-anchor", "start")?
                    .attr("font-size", "large")?
                    .attr("x", width - padding / 1.2)?
                    .attr("y", paddingy + (i as f64) * spacing)
            })?;
            
            write!(text,"{}",moveable_format(|w|plots.write_name(w)))?;

            Ok(text)
        })?;
        //}

        let legendx1 = width - padding / 1.2 + padding / 30.0;
        let legendy1 = paddingy - padding / 8.0 + (i as f64) * spacing;

        //Draw plots

        let it = plots.iter_second().map(|[x, y]| {
            [
                padding + (x - minx) * scalex,
                height - paddingy - (y - miny) * scaley,
            ]
        });

        match plot_type {
            PlotType::Line => {
                //TODO better way to modularize this if statement for all plots?
                //if !name.is_empty() {
                svg.single("line", |w| {
                    w.with_attr("class", wr!("poloto{}stroke", colori))?
                        .attr("stroke", "black")?
                        .attr("x1", legendx1)?
                        .attr("x2", legendx1 + padding / 3.0)?
                        .attr("y1", legendy1)?
                        .attr("y2", legendy1)
                })?;
                //}

                svg.single("polyline", |w| {
                    w.with_attr("class", wr!("poloto{}stroke", colori))?
                        .attr("fill", "none")?
                        .attr("stroke", "black")?
                        .points_data(|w| {
                            for [x, y] in it {
                                w.add_point(x, y)?;
                            }
                            Ok(w)
                        })
                })?;
            }
            PlotType::Scatter => {
                //if !name.is_empty() {
                svg.single("circle", |w| {
                    w.with_attr("class", wr!("poloto{}fill", colori))?
                        .attr("cx", legendx1 + padding / 30.0)?
                        .attr("cy", legendy1)?
                        .attr("r", padding / 30.0)
                })?;
                //}

                svg.elem("g", |w| {
                    let g = w.write(|w| w.with_attr("class", wr!("poloto{}fill", colori)))?;

                    for [x, y] in it {
                        g.single("circle", |w| {
                            //TODO use a g element!!!!
                            w.attr("cx", x)?.attr("cy", y)?.attr("r", padding / 30.0)
                        })?;
                    }
                    Ok(g)
                })?;
            }
            PlotType::Histo => {
                //if !name.is_empty() {
                svg.single("rect", |w| {
                    w.with_attr("class", wr!("poloto{}fill", colori))?
                        .attr("x", legendx1)?
                        .attr("y", legendy1 - padding / 30.0)?
                        .attr("width", padding / 3.0)?
                        .attr("height", padding / 20.0)?
                        .attr("rx", padding / 30.0)?
                        .attr("ry", padding / 30.0)
                })?;
                //}

                svg.elem("g", |w| {
                    let g = w.write(|w| w.with_attr("class", wr!("poloto{}fill", colori)))?;

                    let mut last = None;
                    for [x, y] in it {
                        if let Some((lx, ly)) = last {
                            g.single("rect", |w| {
                                w.attr("x", lx)?
                                    .attr("y", ly)?
                                    .attr(
                                        "width",
                                        (padding * 0.02).max((x - lx) - (padding * 0.02)),
                                    )?
                                    .attr("height", height - paddingy - ly)
                            })?;
                        }
                        last = Some((x, y))
                    }

                    Ok(g)
                })?;
            }
            PlotType::LineFill => {
                //if !name.is_empty() {
                svg.single("rect", |w| {
                    w.with_attr("class", wr!("poloto{}fill", colori))?
                        .attr("x", legendx1)?
                        .attr("y", legendy1 - padding / 30.0)?
                        .attr("width", padding / 3.0)?
                        .attr("height", padding / 20.0)?
                        .attr("rx", padding / 30.0)?
                        .attr("ry", padding / 30.0)
                })?;
                //}
                svg.single("path", |w| {
                    w.with_attr("class", wr!("poloto{}fill", colori))?
                        .path_data(|data| {
                            use tagger::svg::PathCommand::*;
                            data.draw(M(padding, height - paddingy))?;

                            for [x, y] in it {
                                data.draw(L(x, y))?;
                            }

                            data.draw(L(width - padding, height - paddingy))?;
                            data.draw_z()
                        })
                })?;
            }
        }
    }

    svg.elem("text", |writer| {
        let text = writer.write(|w| {
            w.attr("class", "poloto_text")?
                .attr("alignment-baseline", "start")?
                .attr("text-anchor", "middle")?
                .attr("font-size", "x-large")?
                .attr("x", width / 2.0)?
                .attr("y", padding / 4.0)
        })?;
        
        write!(text,"{}",moveable_format(|f|names.write_title(f)))?;
        Ok(text)
    })?;

    svg.elem("text", |writer| {
        let text = writer.write(|w| {
            w.attr("class", "poloto_text")?
                .attr("alignment-baseline", "start")?
                .attr("text-anchor", "middle")?
                .attr("font-size", "x-large")?
                .attr("x", width / 2.0)?
                .attr("y", height - padding / 8.)
        })?;
        write!(text,"{}",moveable_format(|f|names.write_xname(f)))?;
        
        Ok(text)
    })?;

    svg.elem("text", |writer| {
        let text = writer.write(|w| {
            w.attr("class", "poloto_text")?
                .attr("alignment-baseline", "start")?
                .attr("text-anchor", "middle")?
                .attr("font-size", "x-large")?
                .with_attr(
                    "transform",
                    wr!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
                )?
                .attr("x", padding / 4.0)?
                .attr("y", height / 2.0)
        })?;
        write!(text,"{}",moveable_format(|f|names.write_yname(f)))?;
        
        Ok(text)
    })?;

    svg.single("path", |w| {
        w.attr("stroke", "black")?
            .attr("fill", "none")?
            .attr("class", "poloto_axis_lines")?
            .path_data(|p| {
                use tagger::svg::PathCommand::*;
                p.draw(M(padding, paddingy))?
                    .draw(L(padding, height - paddingy))?
                    .draw(L(width - padding, height - paddingy))
            })
    })?;

    Ok(writer)
}
