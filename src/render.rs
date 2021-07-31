use crate::*;
use tagger::prelude::*;

use std::fmt;

//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub fn render<'a>(plotter: &mut Plotter<'a>) -> Result<tagger::Element<'a>, fmt::Error> {
    let mut plotter = {
        let mut empty = crate::Plotter::new("", "", "");
        core::mem::swap(&mut empty, plotter);
        empty
    };

    let mut svg = tagger::Element::one_new("");

    let width = crate::WIDTH as f64;
    let height = crate::HEIGHT as f64;
    let padding = 150.0;
    let paddingy = 100.0;

    svg.append(single!(
        "rect",
        ("class","poloto_background"),
        ("fill","white"),
        ("x",0),
        ("y",0),
        ("width",width),
        ("height",height)
    ));
    
    //Find range.
    let [minx, maxx, miny, maxy] = if let Some(m) = util::find_bounds(
        plotter.plots.iter_mut().flat_map(|x| {
            x.plots
                .iter_first()
                .filter(|[x, y]| x.is_finite() && y.is_finite())
        }),
        plotter.xmarkers,
        plotter.ymarkers,
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

    for (
        i,
        colori,
        Plot {
            plot_type,
            mut plots,
        },
    ) in plotter
        .plots
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
                    svg.append(single!(
                        "line",
                        ("class",formatm!("poloto{}stroke", colori)),
                        ("stroke","black"),
                        ("x1",legendx1),
                        ("x2",legendx1+padding/3.0),
                        ("y1",legendy1),
                        ("y2",legendy1)
                    ));
                }

                let mut path = tagger::path_builder();
                line(&mut path, it);

                //println!("YOOOOOOOOOOOOO \"{}\"\n\n",path.build());
                //let k=path.build();
                svg.append(
                    elem!(
                        "path",
                        ("class",formatm!("poloto{}stroke", colori)),
                        ("fill","none"),
                        ("stroke","black"),
                        ("d",path.build())
                    )
                );

            }
            PlotType::Scatter => {
                if name_exists {
                    svg.append(single!(
                        "circle",
                        ("class",formatm!("poloto{}fill", colori)),
                        ("cx",legendx1 + padding / 30.0),
                        ("cy",legendy1),
                        ("r",padding / 30.0)
                    ));
                }

                use tagger::PathCommand::*;
                let mut d = tagger::path_builder();
                for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                    d.add(M(x, y));
                    d.add(H_(0));
                }

                svg.append(single!(
                    "path",
                    ("class",formatm!("scatter poloto{}stroke", colori)),
                    ("d",d.build())
                ));

            }
            PlotType::Histo => {
                if name_exists {
                    svg.append(single!(
                        "rect",
                        ("class",formatm!("poloto{}fill", colori)),
                        ("x",legendx1),
                        ("y",legendy1 - padding / 30.0),
                        ("width",padding / 3.0),
                        ("height",padding / 20.0),
                        ("rx",padding / 30.0),
                        ("ry",padding / 30.0)
                    ));
                }

                let mut g = elem!(
                    "g",
                    ("class",formatm!("poloto{}fill", colori))
                );

                let mut last = None;
                //TODO dont necesarily filter?
                for [x, y] in it.filter(|&[x, y]| x.is_finite() && y.is_finite()) {
                    if let Some((lx, ly)) = last {
                        g.append(single!(
                            "rect",
                            ("x",lx),
                            ("y",ly),
                            ("width",(padding * 0.02).max((x - lx) - (padding * 0.02))),
                            ("height",height - paddingy - ly)
                        ));
                    }
                    last = Some((x, y))
                }

                svg.append(g);
            }
            PlotType::LineFill => {
                if name_exists {
                    svg.append(single!(
                        "rect",
                        ("class",formatm!("poloto{}fill", colori)),
                        ("x",legendx1),
                        ("y",legendy1 - padding / 30.0),
                        ("width",padding / 3.0),
                        ("height",padding / 20.0),
                        ("rx",padding / 30.0),
                        ("ry",padding / 30.0)
                    ));
                }

                let mut path = tagger::path_builder();

                line_fill(&mut path, it, height - paddingy);

                svg.append(elem!(
                    "path",
                    ("class",formatm!("poloto{}fill", colori)),
                    ("d",path.build())
                ));
            }
        }

        let name = tagger::moveable_format(move |w| plots.write_name(w));

        svg.append(elem!(
            "text",
            ("class","poloto_text"),
            ("alignment-baseline","middle"),
            ("text-anchor","start"),
            ("font-size","large"),
            ("x", width - padding / 1.2),
            ("y",paddingy + (i as f64) * spacing)
        ).appendm(name));
    }

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
                let t=elem!(
                    "text",
                    ("class","poloto_text"),
                    ("alignment-baseline", "middle"),
                    ("text-anchor", "start"),
                    ("x",width*0.55),
                    ("y",paddingy*0.7)
                );

                svg.append(t.appendm(tagger::moveable_format(move |w| {
                    write!(w, "Where j = ")?;
                    crate::util::interval_float(w, xstart_step, None)
                })));

                ("j+", 0.0)
            } else {
                ("", xstart_step)
            };

            //Draw interva`l x text
            for a in 0..xstep_num {
                let p = (a as f64) * xstep;

                let xx = (distance_to_firstx + p) * scalex + padding;

                svg.append(single!(
                    "line",
                    ("class","poloto_axis_lines"),
                    ("stroke","black"),
                    ("x1",xx),
                    ("x2",xx),
                    ("y1",height - paddingy),
                    ("y2",height - paddingy * 0.95)
                ));
                
                
                let text=elem!(
                    "text",
                    ("class","poloto_text"),
                    ("alignment-baseline", "start"),
                    ("text-anchor", "middle"),
                    ("x",xx),
                    ("y",height - paddingy + texty_padding)
                );

                svg.append(text.appendm(tagger::moveable_format(move |w| {
                    write!(w, "{}", extra)?;
                    util::interval_float(w, p + xstart_step, Some(xstep))
                })));
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

                let text=elem!(
                    "text",
                    ("class","poloto_text"),
                    ("alignment-baseline", "middle"),
                    ("text-anchor", "start"),
                    ("x",padding),
                    ("y",paddingy*0.7)
                );
                
                svg.append(text.appendm(tagger::moveable_format(move |w| {
                    write!(w, "Where k = ")?;
                    crate::util::interval_float(w, ystart_step, None)
                })));

                ("k+", 0.0)
            } else {
                ("", ystart_step)
            };

            //Draw interval y text
            for a in 0..ystep_num {
                let p = (a as f64) * ystep;

                let yy = height - (distance_to_firsty + p) * scaley - paddingy;

                svg.append(single!(
                    "line",
                    ("class","poloto_axis_lines"),
                    ("stroke","black"),
                    ("x1",padding),
                    ("x2",padding*0.96),
                    ("y1",yy),
                    ("y2",yy)
                ));

                let text=elem!(
                    "text",
                    ("class", "poloto_text"),
                    ("alignment-baseline", "middle"),
                    ("text-anchor", "end"),
                    ("x",padding-textx_padding),
                    ("y",yy)
                );
                
                svg.append(text.appendm(tagger::moveable_format(move |w| {
                    write!(w, "{}", extra)?;
                    util::interval_float(w, p + ystart_step, Some(ystep))
                })));
            }
        }
    }


    let text=elem!(
        "text",
        ("class","poloto_text"),
        ("alignment-baseline", "start"),
        ("text-anchor", "middle"),
        ("font-size", "x-large"),
        ("x", width / 2.0),
        ("y", padding / 4.0)
    );
    svg.append(text.appendm(plotter.title));


    let text=elem!(
        "text",
        ("class", "poloto_text"),
        ("alignment-baseline", "start"),
        ("text-anchor", "middle"),
        ("font-size", "x-large"),
        ("x", width / 2.0),
        ("y", height - padding / 8.)
    );
    
    svg.append(text.appendm(plotter.xname));

    let text=elem!(
        "text",
        ("class", "poloto_text"),
        ("alignment-baseline", "start"),
        ("text-anchor", "middle"),
        ("font-size", "x-large"),
        ("transform", formatm!("rotate(-90,{},{})", padding / 4.0, height / 2.0)),
        ("x", padding / 4.0),
        ("y", height / 2.0)
    );

    svg.append(text.appendm(plotter.yname));

    use tagger::PathCommand::*;
    
    svg.append(single!(
        "path",
        ("stroke", "black"),
        ("fill", "none"),
        ("class", "poloto_axis_lines"),
        ("d",path!(
            M(padding,paddingy),
            L(padding,height-paddingy),
            L(width-padding,height-paddingy)
        ))
    ));

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

fn line_fill(
    path: &mut tagger::PathBuilder,
    mut it: impl Iterator<Item = [f64; 2]>,
    base_line: f64,
) {
    if let Some([startx, starty]) = it.next() {
        use tagger::PathCommand::*;

        let mut last = [startx, starty];
        let mut last_finite = None;
        let mut first = true;
        for [newx, newy] in it {
            match (
                newx.is_finite() && newy.is_finite(),
                last[0].is_finite() && last[1].is_finite(),
            ) {
                (true, true) => {
                    if first {
                        path.add(M(last[0], base_line));
                        path.add(L(last[0], last[1]));

                        first = false;
                    }
                    last_finite = Some([newx, newy]);
                    path.add(L(newx, newy));
                }
                (true, false) => {
                    path.add(M(newx, newy));
                }
                (false, true) => {
                    path.add(L(last[0], base_line));
                }
                _ => {}
            };
            last = [newx, newy];
        }
        if let Some([x, _]) = last_finite {
            path.add(L(x, base_line));
            path.add(Z(""));
        }
    }
}
fn line(path: &mut tagger::PathBuilder, mut it: impl Iterator<Item = [f64; 2]>) {
    if let Some([startx, starty]) = it.next() {
        use tagger::PathCommand::*;

        let mut last = [startx, starty];
        let mut first = true;
        for [newx, newy] in it {
            match (
                newx.is_finite() && newy.is_finite(),
                last[0].is_finite() && last[1].is_finite(),
            ) {
                (true, true) => {
                    if first {
                        path.add(M(last[0], last[1]));
                        first = false;
                    }
                    path.add(L(newx, newy));
                }
                (true, false) => {
                    path.add(M(newx, newy));
                }
                _ => {}
            };
            last = [newx, newy];
        }
    }
}
