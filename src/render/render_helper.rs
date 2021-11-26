use super::*;

pub fn line_fill<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
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
                        path.put(M(last[0], base_line));
                        path.put(L(last[0], last[1]));

                        first = false;
                    }
                    last_finite = Some([newx, newy]);
                    path.put(L(newx, newy));
                }
                (true, false) => {
                    path.put(M(newx, newy));
                }
                (false, true) => {
                    path.put(L(last[0], base_line));
                }
                _ => {}
            };
            last = [newx, newy];
        }
        if let Some([x, _]) = last_finite {
            path.put(L(x, base_line));
            path.put(Z(""));
        }
    }
}

pub fn line<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
) {
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
                        path.put(M(last[0], last[1]));
                        first = false;
                    }
                    path.put(L(newx, newy));
                }
                (true, false) => {
                    path.put(M(newx, newy));
                }
                _ => {}
            };
            last = [newx, newy];
        }
    }
}

///
/// Draw the axis lines, and tick intervals
///
pub(super) fn draw_base<T: fmt::Write>(
    plotter: &mut Plotter,
    writer: &mut tagger::ElemWriter<T>,
    dd: DrawData,
    sd: ScaleData,
) {
    let DrawData {
        width,
        height,
        padding,
        paddingy,
    } = dd;
    let ScaleData {
        minx,
        maxx,
        miny,
        maxy,
        scalex,
        scaley,
        preserve_aspect,
        aspect_offset,
    } = sd;

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_text")
                .attr("alignment-baseline", "start")
                .attr("text-anchor", "middle")
                .attr("font-size", "x-large")
                .attr("x", width / 2.0)
                .attr("y", padding / 4.0);
        })
        .build(|w| {
            write!(w.writer(), "{}", plotter.title).unwrap();
        });

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_text")
                .attr("alignment-baseline", "start")
                .attr("text-anchor", "middle")
                .attr("font-size", "x-large")
                .attr("x", width / 2.0)
                .attr("y", height - padding / 8.);
        })
        .build(|w| {
            write!(w.writer(), "{}", plotter.xname).unwrap();
        });

    writer
        .elem("text", |d| {
            d.attr("class", "poloto_text")
                .attr("alignment-baseline", "start")
                .attr("text-anchor", "middle")
                .attr("font-size", "x-large")
                .attr(
                    "transform",
                    format_args!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
                )
                .attr("x", padding / 4.0)
                .attr("y", height / 2.0);
        })
        .build(|w| {
            write!(w.writer(), "{}", plotter.yname).unwrap();
        });

    {
        //Draw step lines
        //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

        let ideal_num_xsteps = if preserve_aspect { 5 } else { 9 };

        let ideal_num_ysteps = 7;

        let texty_padding = paddingy * 0.3;
        let textx_padding = padding * 0.1;

        
        
        let (xstep,good_normalized_stepx)=util::find_good_step_f64(ideal_num_xsteps,[minx,maxx]);
        let (xstart_step,xstep_num)=util::get_range_info_f64(xstep,[minx,maxx]).unwrap();


        let (ystep,good_normalized_stepy)=util::find_good_step_f64(ideal_num_ysteps,[miny,maxy]);
        let (ystart_step,ystep_num)=util::get_range_info_f64(ystep,[miny,maxy]).unwrap();
        
        
        use tagger::PathCommand::*;

        fn best_dash_size(
            one_step: f64,
            mut good_normalized_step: u8,
            target_dash_size: f64,
        ) -> f64 {
            assert!(
                good_normalized_step == 2
                    || good_normalized_step == 5
                    || good_normalized_step == 10
            );

            if good_normalized_step == 10 {
                good_normalized_step = 5;
            }

            for x in 1..50 {
                let dash_size = one_step / ((good_normalized_step * x) as f64);

                if dash_size < target_dash_size {
                    return dash_size;
                }
            }
            unreachable!(
                "Could not find a good dash step size! {:?}",
                (one_step, good_normalized_step, target_dash_size)
            );
        }

        //The target dash size will be halfed later.
        //This ensures that its always an even number of dash and empty spaces which is needed
        //to avoid alternating dashes every interval for odd values (5,15,25,35,etc).
        let ydash_size = best_dash_size(ystep * scaley, good_normalized_stepy, 20.0);
        let xdash_size = best_dash_size(xstep * scalex, good_normalized_stepx, 20.0);

        let distance_to_firstx = xstart_step - minx;
        let distance_to_firsty = ystart_step - miny;

        {
            //step num is assured to be atleast 1.
            let (extra, xstart_step) = if util::determine_if_should_use_strat(
                xstart_step,
                xstart_step + ((xstep_num - 1) as f64) * xstep,
                xstep,
            ) {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "middle")
                            .attr("text-anchor", "start")
                            .attr("x", width * 0.55)
                            .attr("y", paddingy * 0.7);
                    })
                    .build(|d| {
                        d.put_raw(format_args!(
                            "Where j = {}",
                            DisplayableClosure::new(|w| plotter.xinterval_formatter.write(
                                w,
                                xstart_step,
                                None
                            ))
                        ));
                    });

                ("j+", 0.0)
            } else {
                ("", xstart_step)
            };

            //Draw interva`l x text
            for a in 0..xstep_num {
                let p = (a as f64) * xstep;

                let xx = (distance_to_firstx + p) * scalex + padding;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")
                        .attr("stroke", "black")
                        .attr("x1", aspect_offset + xx)
                        .attr("x2", aspect_offset + xx)
                        .attr("y1", height - paddingy)
                        .attr("y2", height - paddingy * 0.95);
                });

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "start")
                            .attr("text-anchor", "middle")
                            .attr("x", aspect_offset + xx)
                            .attr("y", height - paddingy + texty_padding);
                    })
                    .build(|w| {
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.xinterval_formatter.write(
                                w,
                                p + xstart_step,
                                Some(xstep)
                            ))
                        ));
                    });
            }
        }

        {
            //step num is assured to be atleast 1.
            let (extra, ystart_step) = if util::determine_if_should_use_strat(
                ystart_step,
                ystart_step + ((ystep_num - 1) as f64) * ystep,
                ystep,
            ) {
                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "middle")
                            .attr("text-anchor", "start")
                            .attr("x", padding)
                            .attr("y", paddingy * 0.7);
                    })
                    .build(|w| {
                        w.put_raw(format_args!(
                            "Where k = {}",
                            DisplayableClosure::new(|w| plotter.yinterval_formatter.write(
                                w,
                                ystart_step,
                                None
                            ))
                        ));
                    });

                ("k+", 0.0)
            } else {
                ("", ystart_step)
            };

            //Draw interval y text
            for a in 0..ystep_num {
                let p = (a as f64) * ystep;

                let yy = height - (distance_to_firsty + p) * scaley - paddingy;

                writer.single("line", |d| {
                    d.attr("class", "poloto_axis_lines")
                        .attr("stroke", "black")
                        .attr("x1", aspect_offset + padding)
                        .attr("x2", aspect_offset + padding * 0.96)
                        .attr("y1", yy)
                        .attr("y2", yy);
                });

                writer
                    .elem("text", |d| {
                        d.attr("class", "poloto_text")
                            .attr("alignment-baseline", "middle")
                            .attr("text-anchor", "end")
                            .attr("x", aspect_offset + padding - textx_padding)
                            .attr("y", yy);
                    })
                    .build(|w| {
                        w.put_raw(format_args!(
                            "{}{}",
                            extra,
                            DisplayableClosure::new(|w| plotter.yinterval_formatter.write(
                                w,
                                p + ystart_step,
                                Some(ystep)
                            ))
                        ));
                    });
            }
        }

        writer.single("path", |d| {
            d.attr("stroke", "black")
                .attr("fill", "none")
                .attr("class", "poloto_axis_lines")
                .attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        xdash_size / 2.0,
                        -distance_to_firstx * scalex
                    ),
                )
                .path(|p| {
                    p.put(M(padding + aspect_offset, height - paddingy));
                    if preserve_aspect {
                        p.put(L(
                            height - paddingy / 2.0 + aspect_offset,
                            height - paddingy,
                        ));
                    } else {
                        p.put(L(width - padding + aspect_offset, height - paddingy));
                    }
                });
        });

        writer.single("path", |d| {
            d.attr("stroke", "black")
                .attr("fill", "none")
                .attr("class", "poloto_axis_lines")
                .attr(
                    "style",
                    format_args!(
                        "stroke-dasharray:{};stroke-dashoffset:{};",
                        ydash_size / 2.0,
                        -distance_to_firsty * scaley
                    ),
                )
                .path(|p| {
                    p.put(M(aspect_offset + padding, height - paddingy));
                    p.put(L(aspect_offset + padding, paddingy));
                });
        });
    }
}
