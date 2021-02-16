pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 500.0;

use super::*;

//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub fn render<T:Write>(pl: Plotter,svg:&mut Element<T>) ->fmt::Result{
    use tagger::prelude::*;
        
    let width = WIDTH;
    let height = HEIGHT;
    let padding = 150.0;
    let paddingy = 100.0;
    
    /*
    let mut svg=if pl.make_svg{
        use tagger::prelude::*;
        let root=pl.element;
        let svg=root.tag_build_flat("svg")
        .set("class","poloto")
        .set("height",render::HEIGHT)
        .set("width",render::WIDTH)
        .set("viewBox",format!("0 0 {} {}",render::WIDTH,render::HEIGHT))
        .set("xmlns","http://www.w3.org/2000/svg")
        .end();
        svg
    }else{
        pl.element
    };
    */


    //Draw background
    svg.tag_build("rect").set("class", "poloto_background")
    //Do this just so that on svg viewers that don't support css they see *something*.
    .set("fill", "white")
    .set("x", 0)
    .set("y", 0)
    .set("width", width)
    .set("height", height)
    .end();


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

    
    let mut s=svg.tag_build("style").end();
    write!(s.get_writer(),
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
    ).unwrap();
    drop(s);

    //TODO BIIIIG data structure. what to do?
    let plots: Vec<_> = pl
        .plots
        .into_iter()
        .map(|mut x| {
            let plots:Vec<_>=x.plots
            .get_iter_mut()
            .filter(|[x, y]| !(x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite()))
            .collect();
            
        
            PlotDecomp {
                plot_type:x.plot_type,
                name:x.name,
                plots,
            }
        })
        .collect();

    //Find range.
    let [minx, maxx, miny, maxy] =
        if let Some(m) = util::find_bounds(plots.iter().flat_map(|x| x.plots.iter().map(|x| *x))) {
            m
        } else {
            //TODO test that this looks ok
            return Ok(()); //No plots at all. dont need to draw anything
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

        let ideal_num_xsteps = 9;
        let ideal_num_ysteps = 10;
        
        let texty_padding = paddingy * 0.4;
        let textx_padding = padding * 0.2;

        let (xstep_num, xstep,xstart_step) = util::find_good_step(ideal_num_xsteps, [minx,maxx] );
        let (ystep_num, ystep,ystart_step) = util::find_good_step(ideal_num_ysteps, [miny,maxy] );


        let distance_to_firstx=xstart_step-minx;

        let distance_to_firsty=ystart_step-miny;

        //Draw interva`l x text
        for a in 0..xstep_num {
            
            let p = (a as f32) * xstep;


            let xx=(distance_to_firstx+p) * scalex + padding;
            svg.tag_build("line")
                    .set("x1",xx)
                    .set("x2",xx)
                    .set("y1",height-paddingy)
                    .set("y2",height-paddingy*0.95)
                    .set("stroke","black")
                    .set("class", "poloto_axis_lines")
                    .empty();

            let mut t=svg.tag_build("text")
                    .set("x", xx)
                    .set("y", height - paddingy + texty_padding)
                    .set("alignment-baseline", "start")
                    .set("text-anchor", "middle")
                    .set("class", "poloto_text").end();


            t.write_str(&util::print_interval_float(p + xstart_step,xstep));
            
        }

        //Draw interval y text
        for a in 0..ystep_num {
            let p = (a as f32) * ystep;

            let yy=height - (distance_to_firsty+p) * scaley - paddingy;
            svg.tag_build("line")
                    .set("x1",padding)
                    .set("x2",padding*0.96)
                    .set("y1",yy)
                    .set("y2",yy)
                    .set("stroke","black")
                    .set("class", "poloto_axis_lines")
                    .empty();
            let mut t=svg.tag_build("text")
                    .set("x", padding - textx_padding)
                    .set("y", yy)
                    .set("alignment-baseline", "middle")
                    .set("text-anchor", "end")
                    .set("class", "poloto_text")
                    .end();

            t.write_str(&util::print_interval_float(p + ystart_step,ystep));
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

        let mut t=svg.tag_build("text")
                .set("x", width - padding / 1.2)
                .set("y", paddingy + (i as f32) * spacing)
                .set("alignment-baseline", "middle")
                .set("text-anchor", "start")
                .set("font-size", "large")
                .set("class", "poloto_text")
                .end();
        
        t.write_str(name);
        drop(t);

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
                svg.tag_build("line")
                    .set("x1", legendx1)
                    .set("y1", legendy1)
                    .set("x2", legendx1 + padding / 3.0)
                    .set("y2", legendy1)
                    .setw("class",|w|write!(w,"poloto{}stroke",colori))    
                    .empty();
                
                let mut poly=svg.tag_build("polyline")
                    .setw("class",|w|write!(w,"poloto{}stroke",colori))
                    .set("fill", "none")
                    //Do this so that on legacy svg viewers that dont have CSS, we see *something*.
                    .set("stroke", "black");

                {
                    let mut d=poly.polyline_data();
                    for p in it{
                        d.point(p);
                    }
                }
                
                poly.empty();
                
            }
            PlotType::Scatter => {
                svg.tag_build("circle")
                    .set("cx", legendx1 + padding / 30.0)
                    .set("cy", legendy1)
                    .set("r", padding / 30.0)
                    .setw("class",|w|write!(w,"poloto{}fill",colori))    
                    .empty();
                
                for [x, y] in it {
                    svg.tag_build("circle")
                        .set("cx", x)
                        .set("cy", y)
                        .set("r", padding / 50.0)
                        .setw("class",|w|write!(w,"poloto{}fill",colori))
                        .empty();
                    
                }
            }
            PlotType::Histo => {
                svg.tag_build("rect")
                        .setw("class",|w|write!(w,"poloto{}fill",colori))
                        .set("x", legendx1)
                        .set("y", legendy1 - padding / 30.0)
                        .set("width", padding / 3.0)
                        .set("height", padding / 20.0)
                        .set("rx", padding / 30.0)
                        .set("ry", padding / 30.0)
                        .empty();
                
                let mut last = None;
                for [x, y] in it {
                    if let Some((lx, ly)) = last {
                        svg.tag_build("rect")
                            .set("x", lx)
                            .set("y", ly)
                            .set("width", (padding * 0.02).max((x - lx) - (padding * 0.02)))
                            .set("height", height - paddingy - ly) //TODO ugly?
                            .setw("class",|w|write!(w,"poloto{}fill",colori))
                            .empty();

                    }
                    last = Some((x, y))
                }
            }
            PlotType::LineFill => {
                svg.tag_build("rect")
                        .setw("class",|w|write!(w,"poloto{}fill",colori))
                        .set("x", legendx1)
                        .set("y", legendy1 - padding / 30.0)
                        .set("width", padding / 3.0)
                        .set("height", padding / 20.0)
                        .set("rx", padding / 30.0)
                        .set("ry", padding / 30.0)
                        .empty();
                

                let mut d=svg.tag_build("path").setw("class",|w|write!(w,"poloto{}fill",colori));
                {
                    let mut data=d.path_data();
                    data.move_to([padding, height - paddingy]);

                    for p in it {
                        data.line_to(p);
                    }

                    data.line_to([width - padding, height - paddingy]);
                    data.close();
                }
                d.empty();

            }
        }
    }


    let mut t=svg.tag_build("text")
            .set("x", width / 2.0)
            .set("y", padding / 4.0)
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set("font-size", "x-large")
            .set("class", "poloto_text")
            .end();
    t.write_str(pl.title);
    drop(t);
    

    let mut t=svg.tag_build("text")
            .set("x", width / 2.0)
            .set("y", height - padding / 8.)
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set("font-size", "large")
            .set("class", "poloto_text")
            .end();
    t.write_str(pl.xname);
    drop(t);


    let mut t=svg.tag_build("text")
            .set("x", padding / 4.0)
            .set("y", height / 2.0)
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle")
            .set(
                "transform",
                format!("rotate(-90,{},{})", padding / 4.0, height / 2.0),
            )
            .set("font-size", "large")
            .set("class", "poloto_text")
            .end();
    t.write_str(pl.yname);
    drop(t);
    

    let mut t=svg.tag_build("path")
        .set("stroke", "black")
        .set("class", "poloto_axis_lines");
    {
        let mut p=t.path_data();
        p.move_to([padding, paddingy])
        .line_to([padding, height - paddingy])
        .line_to([width - padding, height - paddingy]);
    }   
    t.empty();
    Ok(())
}
