use crate::*;
use tagger::attr_builder;
use tagger::prelude::*;

use std::fmt;

//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub fn render<'b>(plotter: Plotter<'b>) -> Result<tagger::Element<'b>, fmt::Error> {
    let Plotter {
        element,
        title,
        xname,
        yname,
        mut plots,
    } = plotter;

    //let header = format!("{}", tagger::moveable_format(|f| names.write_header(f)));
    //let body = format!("{}", tagger::moveable_format(|f| names.write_body(f)));
    //let footer = format!("{}", tagger::moveable_format(|f| names.write_footer(f)));

    //let mut svg = single!(header);
    //svg.append(single!(body));

    let mut svg = element;

    let width = crate::WIDTH as f64;
    let height = crate::HEIGHT as f64;
    let padding = 150.0;
    let paddingy = 100.0;

    let d = attr_builder()
        .attr("class", "poloto_background")
        .attr("fill", "white")
        .attr("x", 0)
        .attr("y", 0)
        .attr("width", width)
        .attr("height", height)
        .build();

    svg.append(single!("rect", d));

    //Find range.
    let [minx, maxx, miny, maxy] = if let Some(m) = util::find_bounds(
        plots
            .iter_mut()
            .flat_map(|x| x.plots.iter_first().map(|[x, y]| [x as f64, y as f64])),
    ) {
        m
    } else {
        //TODO test that this looks ok
        return Ok(svg); //No plots at all. don't need to draw anything
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
                let d = attr_builder()
                    .attr("class", "poloto_text")
                    .attr("alignment-baseline", "middle")
                    .attr("text-anchor", "start")
                    .attr("x", width * 0.55)
                    .attr("y", paddingy * 0.7)
                    .build();

                svg.append(
                    elem!("text", d).add(single!(tagger::moveable_format(move |w| {
                        write!(w, "Where j = ")?;
                        crate::util::interval_float(w, xstart_step, None)
                    }))),
                );

                ("j+", 0.0)
            } else {
                ("", xstart_step)
            };

            //Draw interva`l x text
            for a in 0..xstep_num {
                let p = (a as f64) * xstep;

                let xx = (distance_to_firstx + p) * scalex + padding;

                let d = attr_builder()
                    .attr("class", "poloto_axis_lines")
                    .attr("stroke", "black")
                    .attr("x1", xx)
                    .attr("x2", xx)
                    .attr("y1", height - paddingy)
                    .attr("y2", height - paddingy * 0.95)
                    .build();

                svg.append(single!("line", d));

                let d = attr_builder()
                    .attr("class", "poloto_text")
                    .attr("alignment-baseline", "start")
                    .attr("text-anchor", "middle")
                    .attr("x", xx)
                    .attr("y", height - paddingy + texty_padding)
                    .build();

                svg.append(
                    elem!("text", d).add(single!(tagger::moveable_format(move |w| {
                        write!(w, "{}", extra)?;
                        util::interval_float(w, p + xstart_step, Some(xstep))
                    }))),
                );
            }
        }

        {
            //TODO remove unwrap()???
            //step num is assured to be atleast 1.
            let (extra, ystart_step) = if crate::util::determine_if_should_use_strat(
                ystart_step,
                ystart_step + ((ystep_num - 1) as f64) * ystep,
                ystep,
            )
            .unwrap()
            {
                let e = attr_builder()
                    .attr("class", "poloto_text")
                    .attr("alignment-baseline", "middle")
                    .attr("text-anchor", "start")
                    .attr("x", padding)
                    .attr("y", paddingy * 0.7)
                    .build();

                svg.append(
                    elem!("text", e).add(single!(tagger::moveable_format(move |w| {
                        write!(w, "Where k = ")?;
                        crate::util::interval_float(w, ystart_step, None)
                    }))),
                );

                ("k+", 0.0)
            } else {
                ("", ystart_step)
            };

            //Draw interval y text
            for a in 0..ystep_num {
                let p = (a as f64) * ystep;

                let yy = height - (distance_to_firsty + p) * scaley - paddingy;
                let e = attr_builder()
                    .attr("class", "poloto_axis_lines")
                    .attr("stroke", "black")
                    .attr("x1", padding)
                    .attr("x2", padding * 0.96)
                    .attr("y1", yy)
                    .attr("y2", yy)
                    .build();

                svg.append(single!("line", e));

                let e = attr_builder()
                    .attr("class", "poloto_text")
                    .attr("alignment-baseline", "middle")
                    .attr("text-anchor", "end")
                    .attr("x", padding - textx_padding)
                    .attr("y", yy)
                    .build();

                svg.append(
                    single!("text", e).add(single!(tagger::moveable_format(move |w| {
                        write!(w, "{}", extra)?;
                        util::interval_float(w, p + ystart_step, Some(ystep))
                    }))),
                );
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

        let legendx1 = width - padding / 1.2 + padding / 30.0;
        let legendy1 = paddingy - padding / 8.0 + (i as f64) * spacing;

        //Draw plots
        let name_exists = {
            use fmt::Write;
            let mut wc = WriteCounter::new();
            write!(
                wc,
                "{}",
                tagger::moveable_format(|w| { plots.write_name(w) })
            )?;
            wc.get_counter() != 0
        };

        let it = plots.iter_second().map(|[x, y]| {
            [
                padding + (x as f64 - minx) * scalex,
                height - paddingy - (y as f64 - miny) * scaley,
            ]
        });

        match plot_type {
            PlotType::Line => {
                if name_exists {
                    let d = attr_builder()
                        .attr("class", formatm!("poloto{}stroke", colori))
                        .attr("stroke", "black")
                        .attr("x1", legendx1)
                        .attr("x2", legendx1 + padding / 3.0)
                        .attr("y1", legendy1)
                        .attr("y2", legendy1)
                        .build();

                    svg.append(single!("line", d));
                }

                let mut pp = tagger::points_builder();
                for [x, y] in it {
                    pp.add(x, y);
                }

                let d = attr_builder()
                    .attr("class", formatm!("poloto{}stroke", colori))
                    .attr("fill", "none")
                    .attr("stroke", "black")
                    .attr_whole(pp.build())
                    .build();

                svg.append(single!("polyline", d));
            }
            PlotType::Scatter => {
                if name_exists {
                    let d = attr_builder()
                        .attr("class", formatm!("poloto{}fill", colori))
                        .attr("cx", legendx1 + padding / 30.0)
                        .attr("cy", legendy1)
                        .attr("r", padding / 30.0)
                        .build();
                    svg.append(single!("circle", d));
                }

                use tagger::PathCommand::*;
                let mut d = tagger::path_builder();
                d.add(M(padding, height - paddingy));
                for [x, y] in it {
                    d.add(M(x, y));
                    d.add(H_(0));
                }
                d.add(Z(""));

                let e = attr_builder()
                    .attr("class", formatm!("scatter poloto{}stroke", colori))
                    .attr_whole(d.build())
                    .build();

                svg.append(single!("path", e));
            }
            PlotType::Histo => {
                if name_exists {
                    let d = attr_builder()
                        .attr("class", formatm!("poloto{}fill", colori))
                        .attr("x", legendx1)
                        .attr("y", legendy1 - padding / 30.0)
                        .attr("width", padding / 3.0)
                        .attr("height", padding / 20.0)
                        .attr("rx", padding / 30.0)
                        .attr("ry", padding / 30.0)
                        .build();

                    svg.append(single!("rect", d));
                }

                let mut g = elem!(
                    "g",
                    attr_builder()
                        .attr("class", formatm!("poloto{}fill", colori))
                        .build()
                );

                let mut last = None;
                for [x, y] in it {
                    if let Some((lx, ly)) = last {
                        let d = attr_builder()
                            .attr("x", lx)
                            .attr("y", ly)
                            .attr("width", (padding * 0.02).max((x - lx) - (padding * 0.02)))
                            .attr("height", height - paddingy - ly)
                            .build();

                        g.append(single!("rect", d));
                    }
                    last = Some((x, y))
                }

                svg.append(g);
            }
            PlotType::LineFill => {
                if name_exists {
                    let d = attr_builder()
                        .attr("class", formatm!("poloto{}fill", colori))
                        .attr("x", legendx1)
                        .attr("y", legendy1 - padding / 30.0)
                        .attr("width", padding / 3.0)
                        .attr("height", padding / 20.0)
                        .attr("rx", padding / 30.0)
                        .attr("ry", padding / 30.0)
                        .build();

                    svg.append(single!("rect", d));
                }

                let mut path = tagger::path_builder();
                use tagger::PathCommand::*;
                path.add(M(padding, height - paddingy));

                for [x, y] in it {
                    path.add(L(x, y));
                }

                path.add(L(width - padding, height - paddingy));
                path.add(Z(""));

                let d = attr_builder()
                    .attr("class", formatm!("poloto{}fill", colori))
                    .attr_whole(path.build())
                    .build();
                svg.append(elem!("path", d));
            }
        }

        //let name = format!("{}", tagger::moveable_format(|w| { plots.write_name(w) }));
        //let name_exists = !name.is_empty();
        let name = tagger::moveable_format(move |w| plots.write_name(w));

        let d = attr_builder()
            .attr("class", "poloto_text")
            .attr("alignment-baseline", "middle")
            .attr("text-anchor", "start")
            .attr("font-size", "large")
            .attr("x", width - padding / 1.2)
            .attr("y", paddingy + (i as f64) * spacing)
            .build();

        svg.append(elem!("text", d).add(single!(name)));
    }

    let d = attr_builder()
        .attr("class", "poloto_text")
        .attr("alignment-baseline", "start")
        .attr("text-anchor", "middle")
        .attr("font-size", "x-large")
        .attr("x", width / 2.0)
        .attr("y", padding / 4.0)
        .build();
    svg.append(elem!("text", d).add(single!(title)));

    let d = attr_builder()
        .attr("class", "poloto_text")
        .attr("alignment-baseline", "start")
        .attr("text-anchor", "middle")
        .attr("font-size", "x-large")
        .attr("x", width / 2.0)
        .attr("y", height - padding / 8.)
        .build();
    svg.append(elem!("text", d).add(single!(xname)));

    let d = attr_builder()
        .attr("class", "poloto_text")
        .attr("alignment-baseline", "start")
        .attr("text-anchor", "middle")
        .attr("font-size", "x-large")
        .attr(
            "transform",
            formatm!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
        )
        .attr("x", padding / 4.0)
        .attr("y", height / 2.0)
        .build();

    svg.append(elem!("text", d).add(single!(yname)));

    let mut pp = tagger::path_builder();
    use tagger::PathCommand::*;
    pp.add(M(padding, paddingy));
    pp.add(L(padding, height - paddingy));
    pp.add(L(width - padding, height - paddingy));

    let d = attr_builder()
        .attr("stroke", "black")
        .attr("fill", "none")
        .attr("class", "poloto_axis_lines")
        .attr_whole(pp.build())
        .build();

    svg.append(single!("path", d));

    Ok(svg)
}

struct WriteCounter {
    counter: usize,
}
impl WriteCounter {
    fn new() -> WriteCounter {
        WriteCounter { counter: 0 }
    }
    fn get_counter(&self) -> usize {
        self.counter
    }
}
impl fmt::Write for WriteCounter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.counter += s.len();
        Ok(())
    }
}
