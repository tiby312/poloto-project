
use super::*;

pub fn render(pl:Plotter) -> Document {
    let width = 800.0;
    let height = 500.0;
    let padding = 150.0;
    let paddingy = 100.0;

    let mut doc = Document::new()
        .set("width", width)
        .set("height", height)
        .set("viewBox", (0, 0, width, height))
        .set("class", "splotclass")
        .set("id", "splot");

    doc = doc.add(
        element::Rectangle::new()
            .set("class", "pbackground")
            //Do this just so that on legacy svg viewers that don't support css they see *something*.
            .set("fill", "white")
            .set("x", "0")
            .set("y", "0")
            .set("width", format!("{}", width))
            .set("height", format!("{}", height)),
    );

    let text_color = "black";
    let background_color = "yellow";

    let colors = vec!["blue", "red", "green", "purple", "aqua", "brown"];

    let s = element::Style::new(format!(
        r###".splotclass {{
font-family: "Arial";
--fg_color:   {0};
--bg_color:   {1};
--plot_color0:{2};
--plot_color1:{3};
--plot_color2:{4};
--plot_color3:{5};
--plot_color4:{6};
--plot_color5:{7};
}}
.ptext{{fill: var(--fg_color);  }}
.ptext_bold{{fill: var(--fg_color);font-weight: bold; }}
.pline{{stroke: var(--fg_color);}}
.pbackground{{fill: var(--bg_color); }}
.plot0color{{stroke:  var(--plot_color0); }}
.plot1color{{stroke:  var(--plot_color1); }}
.plot2color{{stroke:  var(--plot_color2); }}
.plot3color{{stroke:  var(--plot_color3); }}
.plot4color{{stroke:  var(--plot_color4); }}
.plot5color{{stroke:  var(--plot_color5); }}
.plot0fill{{fill:var(--plot_color0);}}
.plot1fill{{fill:var(--plot_color1);}}
.plot2fill{{fill:var(--plot_color2);}}
.plot3fill{{fill:var(--plot_color3);}}
.plot4fill{{fill:var(--plot_color4);}}
.plot5fill{{fill:var(--plot_color5);}}"###,
        text_color,
        background_color,
        colors[0],
        colors[1],
        colors[2],
        colors[3],
        colors[4],
        colors[5]
    ));

    doc = doc.add(s);

    let [minx, maxx, miny, maxy] =
        if let Some(m) = util::find_bounds(pl.plots.iter().flat_map(|a| a.plots.ref_iter())) {
            m
        } else {
            //TODO test that this looks ok
            return doc; //No plots at all. dont need to draw anything
        };

    let [miny, maxy] = if miny == maxy {
        [miny - 1.0, miny + 1.0]
    } else {
        [miny, maxy]
    };

    let [minx, maxx] = if minx == maxx {
        [minx - 1.0, minx + 1.0]
    } else {
        [minx, maxx]
    };

    let scalex = (width - padding * 2.0) / (maxx - minx);
    let scaley = (height - paddingy * 2.0) / (maxy - miny);

    {
        //Draw step lines
        //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

        let num_steps = 10;
        let texty_padding = paddingy * 0.4;
        let textx_padding = padding * 0.2;

        let (xstep_num, xstep) = util::find_good_step(num_steps, maxx - minx);
        let (ystep_num, ystep) = util::find_good_step(num_steps, maxy - miny);

        let minx_fixed = (minx / xstep).ceil() * xstep;
        let miny_fixed = (miny / ystep).ceil() * ystep;

        for a in 0..xstep_num {
            let p = (a as f32) * xstep;

            let t = node::Text::new(util::print_interval_float(p + minx_fixed));

            doc = doc.add(
                element::Text::new()
                    .add(t)
                    .set("x", format!("{}", p * scalex + padding))
                    .set("y", format!("{}", height - paddingy + texty_padding))
                    .set("alignment-baseline", "start")
                    .set("text-anchor", "middle")
                    .set("class", "ptext"),
            );
        }

        for a in 0..ystep_num {
            let p = (a as f32) * ystep;

            let t = node::Text::new(util::print_interval_float(p + miny_fixed));

            doc = doc.add(
                element::Text::new()
                    .add(t)
                    .set("x", format!("{}", padding - textx_padding))
                    .set("y", format!("{}", height - p * scaley - paddingy))
                    .set("alignment-baseline", "middle")
                    .set("text-anchor", "end")
                    .set("class", "ptext"),
            );
        }
    }

    for (
        i,
        Plot {
            plot_type,
            name,
            mut plots,
        },
    ) in pl.plots.into_iter().enumerate()
    {
        let spacing = padding / 3.0;

        doc = doc.add(
            element::Text::new()
                .add(node::Text::new(name))
                .set("x", format!("{}", width - padding / 1.2))
                .set("y", format!("{}", paddingy + (i as f32) * spacing))
                .set("alignment-baseline", "middle")
                .set("text-anchor", "start")
                .set("font-size", "large")
                .set("class", "ptext"),
        );

        doc = doc.add(
            element::Circle::new()
                .set("cx", format!("{}", width - padding / 1.2 + padding / 30.0))
                .set(
                    "cy",
                    format!("{}", paddingy - padding / 8.0 + (i as f32) * spacing),
                )
                .set("r", format!("{}", padding / 30.0))
                .set("class", format!("plot{}fill", i)),
        );

        let it = plots.into_iter().map(|[x, y]| {
            [
                padding + (x - minx) * scalex,
                height - paddingy - (y - miny) * scaley,
            ]
        });

        match plot_type {
            PlotType::Line => {
                use std::fmt::Write;
                let mut points = String::new();
                for [x, y] in it {
                    write!(&mut points, "{},{} ", x, y).unwrap();
                }
                doc = doc.add(
                    Polyline::new()
                        .set("class", format!("plot{}color", i))
                        .set("fill", "none")
                        .set("stroke", "black")
                        .set("stroke-width", 2)
                        .set("points", points),
                );
            }
            PlotType::Scatter => {
                for [x, y] in it {
                    doc = doc.add(
                        element::Circle::new()
                            .set("cx", format!("{}", x))
                            .set("cy", format!("{}", y))
                            .set("r", format!("{}", padding / 50.0))
                            .set("class", format!("plot{}fill", i)),
                    );
                }
            }
            PlotType::Histo => {
                let mut last = None;
                for [x, y] in it {
                    if let Some((lx, ly)) = last {
                        let k = element::Rectangle::new()
                            .set("x", format!("{}", lx))
                            .set("y", format!("{}", ly))
                            .set(
                                "width",
                                format!(
                                    "{}",
                                    (padding * 0.02).max((x - lx) - (padding * 0.02))
                                ),
                            )
                            .set("height", format!("{}", (height - paddingy - ly))) //TODO ugly?
                            .set("class", format!("plot{}fill", i));

                        doc = doc.add(k);
                    }
                    last = Some((x, y))
                }
            }
            PlotType::LineFill => {
                let mut data = Data::new().move_to((padding, height - paddingy));

                for [x, y] in it {
                    data = data.line_to((x, y));
                }

                data = data.line_to((width - padding, height - paddingy));
                data = data.close();

                doc = doc.add(
                    Path::new()
                        .set("class", format!("plot{}fill", i))
                        .set("d", data),
                );
            }
            PlotType::DottedLine => {
                use std::fmt::Write;
                let mut points = String::new();
                for [x, y] in it {
                    write!(&mut points, "{},{} ", x, y).unwrap();
                }
                doc = doc.add(
                    Polyline::new()
                        .set("class", format!("plot{}color", i))
                        .set("fill", "none")
                        .set("stroke-dasharray","4") //TODO combine with ine?
                        .set("stroke", "black")
                        .set("stroke-width", 2)
                        .set("points", points),
                );
            }
        }
    }

    doc = doc.add(
        element::Text::new()
            .add(node::Text::new(pl.title))
            .set("x", format!("{}", width / 2.0))
            .set("y", format!("{}", padding / 4.0))
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set("font-size", "x-large")
            .set("class", "ptext"),
    );

    doc = doc.add(
        element::Text::new()
            .add(node::Text::new(pl.xname))
            .set("x", format!("{}", width / 2.0))
            .set("y", format!("{}", height - padding / 8.))
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set("font-size", "large")
            .set("class", "ptext"),
    );

    doc = doc.add(
        element::Text::new()
            .add(node::Text::new(pl.yname))
            .set("x", format!("{}", padding / 4.0))
            .set("y", format!("{}", height / 2.0))
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set(
                "transform",
                format!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
            )
            .set("font-size", "large")
            .set("class", "ptext"),
    );

    let data = Data::new()
        .move_to((padding, paddingy))
        .line_to((padding, height - paddingy))
        .line_to((width - padding, height - paddingy));

    doc.add(
        Path::new()
            .set("style", "fill:none !important;")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data)
            .set("class", "pline"),
    )
}