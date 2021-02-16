pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 500.0;

use super::*;
use tagger::*;



//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub fn render<T: Write>(pl: Plotter, writer:&mut T) -> fmt::Result {
    
    let width = WIDTH;
    let height = HEIGHT;
    let padding = 150.0;
    let paddingy = 100.0;

    let mut svg=new_element!(
        writer,
        "<svg class='poloto' height='{h}' width='{w}' viewBox='0 0 {w} {h}' xmlns='http://www.w3.org/2000/svg'>",
        "</svg>",
        w=render::WIDTH,
        h=render::HEIGHT)?;


    empty_element!(svg,"<rect class='poloto_background' fill='white' x='0' y='0' width='{}' height='{}'/>",width,height)?;
    

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


    empty_element!(svg,
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
        colors[7],
    )?;

    //TODO BIIIIG data structure. what to do?
    let plots: Vec<_> = pl
        .plots
        .into_iter()
        .map(|mut x| {
            let plots: Vec<_> = x
                .plots
                .get_iter_mut()
                .filter(|[x, y]| !(x.is_nan() || y.is_nan() || x.is_infinite() || y.is_infinite()))
                .collect();

            PlotDecomp {
                plot_type: x.plot_type,
                name: x.name,
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

        let (xstep_num, xstep, xstart_step) = util::find_good_step(ideal_num_xsteps, [minx, maxx]);
        let (ystep_num, ystep, ystart_step) = util::find_good_step(ideal_num_ysteps, [miny, maxy]);

        let distance_to_firstx = xstart_step - minx;

        let distance_to_firsty = ystart_step - miny;

        //Draw interva`l x text
        for a in 0..xstep_num {
            let p = (a as f32) * xstep;

            let xx = (distance_to_firstx + p) * scalex + padding;

            empty_element!(svg,"<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='black' class='poloto_axis_lines'/>",
                    xx,xx,height-paddingy,height-paddingy*0.95)?;

            empty_element!(svg,"<text x='{}' y='{}' alignment-baseline='start' text-anchor='middle' class='poloto_text'>{}</text>",
                xx,height-paddingy+texty_padding,util::interval_float(p + xstart_step, xstep))?;
                
        }

        //Draw interval y text
        for a in 0..ystep_num {
            let p = (a as f32) * ystep;

            let yy = height - (distance_to_firsty + p) * scaley - paddingy;

            empty_element!(svg,"<line x1='{}' x2='{}' y1='{}' y2='{}' stroke='black' class='poloto_axis_lines'/>",
                padding,padding*0.96,yy,yy)?;

            empty_element!(svg,"<text x='{}' y='{}' alignment-baseline='middle' text-anchor='end' class='poloto_text'>{}</text>",
                    padding-textx_padding,yy,util::interval_float(p + ystart_step, ystep))?;


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


        empty_element!(svg,
"<text x='{}' y='{}' alignment-baseline='middle' text-anchor='start' font-size='large' class='poloto_text'>{}</text>",
width-padding/1.2,paddingy+(i as f32)*spacing,name)?;


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

                empty_element!(svg,"<line x1='{}' y1='{}' x2='{}' y2='{}' class='poloto{}stroke'/>",
                    legendx1,legendy1,legendx1+padding/3.0,legendy1,colori)?;


                empty_element!(svg,"<polyline class='poloto{}stroke' fill='none' stroke='black' points='{}'/>",
                    colori,tagger::poly(it))?;
                
            }
            PlotType::Scatter => {
                empty_element!(svg,"<circle cx='{}' cy='{}' r='{}' class='poloto{}fill'/>",
                        legendx1+padding/30.0,legendy1,padding/30.0,colori)?;

                for [x, y] in it {
                    empty_element!(svg,"<circle cx='{}' cy='{}' r='{}' class='poloto{}fill'/>",
                        x,y,padding/30.0,colori)?;
                }
            }
            PlotType::Histo => {

                empty_element!(svg,"<rect class='poloto{}fill' x='{}' y='{}' width='{}' height='{}' rx='{}' ry='{}'/>",
                    colori,legendx1,legendy1-padding/30.0,padding/3.0,padding/20.0,padding/30.0,padding/30.0)?;

                let mut last = None;
                for [x, y] in it {
                    if let Some((lx, ly)) = last {
                        empty_element!(svg,"<rect class='poloto{}fill' x='{}' y='{}' width='{}' height='{}'/>",
                        colori,lx,ly,(padding * 0.02).max((x - lx) - (padding * 0.02)),height - paddingy - ly)?;
                    }
                    last = Some((x, y))
                }
            }
            PlotType::LineFill => {
                empty_element!(svg,"<rect class='poloto{}fill' x='{}' y='{}' width='{}' height='{}' rx='{}' ry='{}'/>",
                    colori,legendx1,legendy1-padding/30.0,padding/3.0,padding/20.0,padding/30.0,padding/30.0)?;

                
                empty_element!(svg,"<path class='poloto{}fill' d='{}'/>",colori,tagger::path(|mut data|{
                    data.move_to([padding, height - paddingy])?;

                    for p in it {
                        data.line_to(p)?;
                    }

                    data.line_to([width - padding, height - paddingy])?;
                    data.close()?;
                    Ok(())
                }))?;

            }
        }
    }

    empty_element!(svg,
        "<text x='{}' y='{}' alignment-baseline='start' text-anchor='middle' font-size='x-large' class='poloto_text'>{}</text>",
        width/2.0,padding/4.0,pl.title)?;


    empty_element!(svg,"<text x='{}' y='{}' alignment-baseline='start' text-anchor='middle' font-size='large' class='poloto_text'>{}</text>",
    width / 2.0,height - padding / 8.,pl.xname)?;
    

    empty_element!(svg,"
        <text x='{}' y='{}' alignment-baseline='start' text-anchor='middle' transform='rotate(-90,{},{})' font-size='large' class='poloto_text'>{}</text>",
            padding/4.0,height/2.0,padding/4.0,height/2.0,pl.yname)?;


    empty_element!(svg,"<path stroke='black' class='poloto_axis_lines' d='{}'/>",
            tagger::path(|mut p|{
                p.move_to([padding, paddingy])?
                .line_to([padding, height - paddingy])?
                .line_to([width - padding, height - paddingy])?;
                Ok(())
            }))?;
    
    Ok(())
}
