pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 500.0;

use super::*;
use svg::Node;
pub fn render(pl: Plotter) -> Document {
    let width = WIDTH;
    let height = HEIGHT;
    let padding = 150.0;
    let paddingy = 100.0;
    let mut doc = pl.doc;

    //Draw background
    doc.append(
        element::Rectangle::new()
            .set("class", "poloto_background")
            //Do this just so that on legacy svg viewers that don't support css they see *something*.
            .set("fill", "white")
            .set("x", "0")
            .set("y", "0")
            .set("width", width)
            .set("height", height),
    );

    //Default colors if CSS is not overriden with user colors.
    let text_color = "black";
    let background_color = "aliceblue";

    let colors = [
        "blue",
        "red",
        "green",
        "gold",
        "aqua",
        "brown",
        "lime",
        "chocolate",
    ];

    //Add CSS styling
    doc.append(element::Style::new(format!(
        r###".poloto {{
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
.poloto7fill{{fill:var(--poloto_color7,{9});}}"###,
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
    )));

    //TODO BIIIIG data structure. what to do?
    let plots: Vec<_> = pl
        .plots
        .into_iter()
        .map(|mut x| PlotDecomp {
            name: x.name,
            plot_type: x.plot_type,
            plots: x.plots.into_iter().collect::<Vec<_>>(),
        })
        .collect();

    //Find range.
    let [minx, maxx, miny, maxy] =
        if let Some(m) = util::find_bounds(plots.iter().flat_map(|x| x.plots.iter().map(|x| *x))) {
            m
        } else {
            //TODO test that this looks ok
            return doc; //No plots at all. dont need to draw anything
        };

    //Insert a range if the range is zero.
    let [miny, maxy] = if miny == maxy {
        [miny - 1.0, miny + 1.0]
    } else {
        [miny, maxy]
    };

    //Insert a range if the range is zero.
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

        //Draw interval x text
        for a in 0..xstep_num {
            let p = (a as f32) * xstep;

            let t = node::Text::new(util::print_interval_float(p + minx_fixed));

            doc.append(
                element::Text::new()
                    .add(t)
                    .set("x", p * scalex + padding)
                    .set("y", height - paddingy + texty_padding)
                    .set("alignment-baseline", "start")
                    .set("text-anchor", "middle")
                    .set("class", "poloto_text"),
            );
        }

        //Draw interval y text
        for a in 0..ystep_num {
            let p = (a as f32) * ystep;

            let t = node::Text::new(util::print_interval_float(p + miny_fixed));

            doc.append(
                element::Text::new()
                    .add(t)
                    .set("x", padding - textx_padding)
                    .set("y", height - p * scaley - paddingy)
                    .set("alignment-baseline", "middle")
                    .set("text-anchor", "end")
                    .set("class", "poloto_text"),
            );
        }
    }

    for (
        i,
        colori,
        PlotDecomp {
            plot_type,
            name,
            plots,
        },
    ) in plots
        .into_iter()
        .enumerate()
        .map(|(i, x)| (i, i % colors.len(), x))
    {
        let spacing = padding / 3.0;

        //Draw legend text
        doc.append(
            element::Text::new()
                .add(node::Text::new(name))
                .set("x", width - padding / 1.2)
                .set("y", paddingy + (i as f32) * spacing)
                .set("alignment-baseline", "middle")
                .set("text-anchor", "start")
                .set("font-size", "large")
                .set("class", "poloto_text"),
        );

        let legendx1 = width - padding / 1.2 + padding / 30.0;
        let legendy1 = paddingy - padding / 8.0 + (i as f32) * spacing;

        //Draw plots

        let it = plots.into_iter().map(|[x, y]| {
            [
                padding + (x - minx) * scalex,
                height - paddingy - (y - miny) * scaley,
            ]
        });

        match plot_type {
            PlotType::Line => {
                let st = format!("poloto{}stroke", colori);
                doc.append(
                    element::Line::new()
                        .set("x1", legendx1)
                        .set("y1", legendy1)
                        .set("x2", legendx1 + padding / 3.0)
                        .set("y2", legendy1)
                        .set("class", st.clone()),
                );
                use std::fmt::Write;
                let mut points = String::new();
                for [x, y] in it {
                    write!(&mut points, "{},{} ", x, y).unwrap();
                }
                doc.append(
                    Polyline::new()
                        .set("class", st)
                        .set("fill", "none")
                        //Do this so that on legacy svg viewers that dont have CSS, we see *something*.
                        .set("stroke", "black")
                        .set("points", points),
                );
            }
            PlotType::Scatter => {
                let st = format!("poloto{}fill", colori);
                doc.append(
                    element::Circle::new()
                        .set("cx", legendx1 + padding / 30.0)
                        .set("cy", legendy1)
                        .set("r", padding / 30.0)
                        .set("class", st.clone()),
                );
                for [x, y] in it {
                    doc.append(
                        element::Circle::new()
                            .set("cx", x)
                            .set("cy", y)
                            .set("r", padding / 50.0)
                            .set("class", st.clone()),
                    );
                }
            }
            PlotType::Histo => {
                let st = format!("poloto{}fill", colori);
                doc.append(
                    element::Rectangle::new()
                        .set("class", st.clone())
                        .set("x", legendx1)
                        .set("y", legendy1 - padding / 30.0)
                        .set("width", padding / 3.0)
                        .set("height", padding / 20.0)
                        .set("rx", padding / 30.0)
                        .set("ry", padding / 30.0),
                );
                let mut last = None;
                for [x, y] in it {
                    if let Some((lx, ly)) = last {
                        let k = element::Rectangle::new()
                            .set("x", lx)
                            .set("y", ly)
                            .set("width", (padding * 0.02).max((x - lx) - (padding * 0.02)))
                            .set("height", height - paddingy - ly) //TODO ugly?
                            .set("class", st.clone());

                        doc.append(k);
                    }
                    last = Some((x, y))
                }
            }
            PlotType::LineFill => {
                let st = format!("poloto{}fill", colori);
                doc.append(
                    element::Rectangle::new()
                        .set("class", st.clone())
                        .set("x", legendx1)
                        .set("y", legendy1 - padding / 30.0)
                        .set("width", padding / 3.0)
                        .set("height", padding / 20.0)
                        .set("rx", padding / 30.0)
                        .set("ry", padding / 30.0),
                );

                let mut data = Data::new().move_to((padding, height - paddingy));

                for [x, y] in it {
                    data = data.line_to((x, y));
                }

                data = data.line_to((width - padding, height - paddingy));
                data = data.close();

                doc.append(Path::new().set("class", st.clone()).set("d", data));
            }
        }
    }

    //Draw title
    doc.append(
        element::Text::new()
            .add(node::Text::new(pl.title))
            .set("x", width / 2.0)
            .set("y", padding / 4.0)
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set("font-size", "x-large")
            .set("class", "poloto_text"),
    );

    //Draw xname
    doc.append(
        element::Text::new()
            .add(node::Text::new(pl.xname))
            .set("x", width / 2.0)
            .set("y", height - padding / 8.)
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set("font-size", "large")
            .set("class", "poloto_text"),
    );

    //Draw yname
    doc.append(
        element::Text::new()
            .add(node::Text::new(pl.yname))
            .set("x", padding / 4.0)
            .set("y", height / 2.0)
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set(
                "transform",
                format!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
            )
            .set("font-size", "large")
            .set("class", "poloto_text"),
    );

    let data = Data::new()
        .move_to((padding, paddingy))
        .line_to((padding, height - paddingy))
        .line_to((width - padding, height - paddingy));

    //Draw axis lines
    doc.append(
        Path::new()
            .set("stroke", "black")
            .set("d", data)
            .set("class", "poloto_axis_lines"),
    );

    doc
}
