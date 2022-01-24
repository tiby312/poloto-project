use crate::*;



pub struct Data<X:PlotNum,Y:PlotNum>{
    pub boundx:[X;2],
    pub boundy:[Y;2],
    pub tickx:TickInfo<X>,
    pub ticky:TickInfo<Y>
}


#[derive(Copy, Clone)]
pub(super) struct Canvas {
    pub ideal_num_xsteps: u32,
    pub ideal_num_ysteps: u32,
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
    aspect_offset: f64,
    pub scalex2: f64,
    pub scaley2: f64,
    spacing: f64,
    legendx1: f64,
    num_css_classes: Option<usize>,
    preserve_aspect: bool,
}
impl Canvas {
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


    pub fn render<X: PlotNumContext, Y: PlotNumContext>(
        &self,
        writer: impl std::fmt::Write,
        plotter: &mut Plotter<X, Y>,
        data:Data<X::Num,Y::Num>
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

        let [minx, maxx] = data.boundx;
        let [miny, maxy] = data.boundy;

        let mut writer = tagger::new(writer);

        let xcontext=&mut plotter.xcontext;
        let ycontext=&mut plotter.ycontext;

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

            let aa = xcontext.scale(minx,[minx, maxx], scalex2);
            let bb = ycontext.scale(miny,[miny, maxy], scaley2);

            struct PlotIter<X:PlotNumContext, Y:PlotNumContext> {
                basex_ii: f64,
                basey_ii: f64,
                rangex_ii: [X::Num; 2],
                rangey_ii: [Y::Num; 2],
                maxx_ii: f64,
                maxy_ii: f64,
            }
            impl<X: PlotNumContext, Y: PlotNumContext> PlotIter<X, Y> {
                fn gen_iter<'a>(
                    &'a self,
                    p: &'a mut Plot<X::Num, Y::Num>,
                    xcontext:&'a mut X,
                    ycontext:&'a mut Y
                ) -> impl Iterator<Item = [f64; 2]> + 'a {
                    p.plots.iter_second().map(move |(x, y)| {
                        [
                            self.basex_ii + xcontext.scale(x,self.rangex_ii, self.maxx_ii),
                            self.basey_ii - ycontext.scale(y,self.rangey_ii, self.maxy_ii),
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
                        d.path(|a| render::line(a, plot_iter.gen_iter(&mut p,xcontext,ycontext)))
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
                                .gen_iter(&mut p,xcontext,ycontext)
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
                                .gen_iter(&mut p,xcontext,ycontext)
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
                                plot_iter.gen_iter(&mut p,xcontext,ycontext),
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
                                plot_iter.gen_iter(&mut p,xcontext,ycontext),
                                height - paddingy,
                                false,
                            )
                        })
                    })?;
                }
            }
        }

        self.draw_base(&mut writer, plotter, data)?;

        Ok(())
    }

    pub fn draw_base<X: PlotNumContext, Y: PlotNumContext, T: fmt::Write>(
        &self,
        writer: &mut tagger::ElemWriter<T>,
        plotter: &mut Plotter<X, Y>,
        data: Data<X::Num, Y::Num>,
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

        use std::fmt::Write;
        let boundx = data.boundx;
        let boundy = data.boundy;
        let [minx, maxx] = boundx;
        let [miny, maxy] = boundy;

        let mut xtick_info = data.tickx;
        let mut ytick_info = data.ticky;

        let xcontext=&mut plotter.xcontext;
        let ycontext=&mut plotter.ycontext;
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
                write!(w.writer_safe(),"{}",plotter.title)
                /* 
                plotter.title.fmt_self(
                    &mut w.writer_safe(),
                    DataSingle {
                        bound: boundx,
                        step: &mut xtick_info.unit_data,
                    },
                    DataSingle {
                        bound: boundy,
                        step: &mut ytick_info.unit_data,
                    },
                )*/
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
                write!(w.writer_safe(),"{}",plotter.xaxis.name)
                /* 
                plotter.xaxis.name.fmt_self(
                    &mut w.writer_safe(),
                    DataSingle {
                        bound: boundx,
                        step: &mut xtick_info.unit_data,
                    },
                )
                */
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
                write!(w.writer_safe(),"{}",plotter.yaxis.name)
                /* 
                plotter.yaxis.name.fmt_self(
                    &mut w.writer_safe(),
                    DataSingle {
                        bound: boundy,
                        step: &mut ytick_info.unit_data,
                    },
                )
                */
            })?;

        let xdash_size: Option<f64> = if plotter.xaxis.dash {
            xtick_info.dash_size
        } else {
            None
        };
        let ydash_size: Option<f64> = if plotter.yaxis.dash {
            ytick_info.dash_size
        } else {
            None
        };

        use tagger::PathCommand::*;

        let first_tickx = xtick_info.ticks[0];

        let first_ticky = ytick_info.ticks[0];

        {
            //step num is assured to be atleast 1.
            let extra = if let Some(mut base) = xtick_info.display_relative {
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

                        xcontext.where_fmt(base, &mut w, boundx)
                    })?;

                "j+"
            } else {
                ""
            };

            //Draw interva`l x text
            for &Tick { position,mut  value } in xtick_info.ticks.iter() {
                let xx = (xcontext.scale(position,[minx, maxx], scalex2)
                    - xcontext.scale(minx,[minx, maxx], scalex2))
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

                        xcontext.tick_fmt(value,&mut w, boundx, &mut xtick_info.unit_data)
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
            let extra = if let Some(mut base) = ytick_info.display_relative {
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

                        ycontext.where_fmt(base,&mut w,boundy)
                    })?;

                "k+"
            } else {
                ""
            };

            //Draw interval y text
            for &Tick { position, mut value } in ytick_info.ticks.iter() {
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

                        ycontext.tick_fmt(value, &mut w, boundy, &mut ytick_info.unit_data)
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

        let d1 = xcontext.scale(minx,[minx, maxx], scalex2);
        let d2 = xcontext.scale(first_tickx.position,[minx, maxx], scalex2);
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

        let d1 = ycontext.scale(miny,[miny, maxy], scaley2);
        let d2 = ycontext.scale(first_ticky.position,[miny, maxy], scaley2);
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

pub fn line_fill<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
    base_line: f64,
    add_start_end_base: bool,
) -> fmt::Result {
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
                        if add_start_end_base {
                            path.put(M(last[0], base_line))?;
                            path.put(L(last[0], last[1]))?;
                        } else {
                            path.put(M(last[0], last[1]))?;
                        }
                        first = false;
                    }
                    last_finite = Some([newx, newy]);
                    path.put(L(newx, newy))?;
                }
                (true, false) => {
                    path.put(M(newx, newy))?;
                }
                (false, true) => {
                    path.put(L(last[0], base_line))?;
                }
                _ => {}
            };
            last = [newx, newy];
        }
        if let Some([x, _]) = last_finite {
            if add_start_end_base {
                path.put(L(x, base_line))?;
            }
            path.put(Z(""))?;
        }
    }
    Ok(())
}

pub fn line<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
) -> fmt::Result {
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
                        path.put(M(last[0], last[1]))?;
                        first = false;
                    }
                    path.put(L(newx, newy))?;
                }
                (true, false) => {
                    path.put(M(newx, newy))?;
                }
                _ => {}
            };
            last = [newx, newy];
        }
    }
    Ok(())
}
