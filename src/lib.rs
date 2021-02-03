//!
//! splot - A very simple plotter
//!
//! splot is meant to be used to graph fairly 'obvious/general' trends
//! in the data. The user is not meant to be able to extract data
//! or notice subtle differences in data. The desired use case is
//! to allow inserting a nice graph into a webpage generated via
//! [mdBook](https://rust-lang.github.io/mdBook/).  
//!
//! For this reason, a lot
//! of options that are normally provided in a plotting library
//! aren't provided. For example, you can't change the thickness
//! of the lines. Instead the user is encouraged, to pick a good
//! set of data points to "zoom in" on a trend they want to show.
//!
//! ### How do I change the color of the plots?
//!
//! You can doing it by overriding the css. If you embed the generated svg into a html file,
//! then you can add this example css styling to override the default in the svg file:
//! ```css
//! #splot{
//!    font-family: "Open Sans", sans-serif;
//!    --fg_color:"black";
//!    --bg_color:"white;
//!    --plot_color0:"red";
//!    --plot_color1:"green";
//!    --plot_color2:"yellow";
//!    --plot_color3:"orange";
//!    --plot_color4:"purple";
//!    --plot_color5:"pink";
//!    --plot_color6:"aqua";
//! }
//! ```  
//! The default coloring uses a css class selector, so using a id select overrides it.
//! Using this method, you can have a specific themes for each mdBook theme.
//!
//! ### Usage
//!
//! plots containing NaN or Infinity are ignored. 
//!
//! ### Example
//!
//! See the graphs in this report: [broccoli_book](https://tiby312.github.io/broccoli_report/)
//!
use core::marker::PhantomData;
use svg::node;
use svg::node::element;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Polyline;
use svg::Document;

struct Wrapper<'a, I: Iterator<Item = [f32; 2]> + Clone + 'a>(Option<I>, PhantomData<&'a I>);

impl<'a, I: Iterator<Item = [f32; 2]> + Clone + 'a> PlotTrait<'a> for Wrapper<'a, I> {
    #[inline(always)]
    fn ref_iter(&self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a> {
        Box::new(self.0.as_ref().unwrap().clone().filter(|[x,y]|!(x.is_nan()||y.is_nan()||x.is_infinite()||y.is_infinite())))
    }

    #[inline(always)]
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a> {
        Box::new(self.0.take().unwrap().filter(|[x,y]|!(x.is_nan()||y.is_nan()||x.is_infinite()||y.is_infinite())))
    }
}

trait PlotTrait<'a> {
    fn ref_iter(&self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a>;
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a>;
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a> {
    name: String,
    plots: Box<dyn PlotTrait<'a> + 'a>,
    plot_type: PlotType,
}

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
///Each Iterator will be iterated through twice by doing one call to clone().
///once to find min max bounds, second to construct plot    
pub struct Plotter<'a> {
    title: String,
    xname: String,
    yname: String,
    plots: Vec<Plot<'a>>,
}

///Shorthand constructor.
pub fn plot<'a>(title: impl ToString, xname: impl ToString, yname: impl ToString) -> Plotter<'a> {
    Plotter::new(title, xname, yname)
}

impl<'a> Plotter<'a> {
    ///Create a plotter
    pub fn new(title: impl ToString, xname: impl ToString, yname: impl ToString) -> Plotter<'a> {
        Plotter {
            title: title.to_string(),
            plots: Vec::new(),
            xname: xname.to_string(),
            yname: yname.to_string(),
        }
    }
    pub fn lines<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    pub fn scatter<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    ///Each bar's left side will line up with a point
    pub fn histogram<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }

    pub fn line_fill<I: Iterator<Item = [f32; 2]> + Clone + 'a>(
        &mut self,
        name: impl ToString,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            name: name.to_string(),
            plots: Box::new(Wrapper(Some(plots), PhantomData)),
        })
    }


    pub fn into_document(self) -> Document {
        self.finalize_doc()
    }

    pub fn render_to_file(self, filename: &str) -> Result<(), std::io::Error> {
        let doc = self.finalize_doc();
        svg::save(filename, &doc)
    }

    pub fn render<T: std::io::Write>(self, target: T) -> Result<(), std::io::Error> {
        let doc = self.finalize_doc();
        svg::write(target, &doc)
    }

    fn finalize_doc(self) -> Document {
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

        let colors = vec!["blue", "red", "green", "purple", "yellow", "aqua"];

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
            if let Some(m) = find_bounds(self.plots.iter().flat_map(|a| a.plots.ref_iter())) {
                m
            } else {
                //TODO test that this looks ok
                return doc; //No plots at all. dont need to draw anything
            };

        let scalex = (width - padding * 2.0) / (maxx - minx);
        let scaley = (height - paddingy * 2.0) / (maxy - miny);

        {
            //Draw step lines
            //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

            let num_steps = 10;
            let texty_padding = paddingy * 0.4;
            let textx_padding = padding * 0.2;

            let (xstep_num, xstep_power, xstep) = find_good_step(num_steps, maxx - minx);
            let (ystep_num, ystep_power, ystep) = find_good_step(num_steps, maxy - miny);

            let minx_fixed = (minx / xstep).ceil() * xstep;
            let miny_fixed = (miny / ystep).ceil() * ystep;

            for a in 0..xstep_num {
                let p = (a as f32) * xstep;

                let precision = (1.0 + xstep_power).max(0.0) as usize;
                
                let t=if (p+minx_fixed).abs().log10()<5.0 {
                    node::Text::new(format!(
                        "{0:.1$}",
                        p + minx_fixed,
                        precision
                    ))
                }else{
                    node::Text::new(format!(
                        "{0:.1$e}",
                        p + minx_fixed,
                        precision
                    ))
                };

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


                let precision = (1.0 + ystep_power).max(0.0) as usize;
                //dbg!((p+miny_fixed).abs().log10());
                
                let t=if (p+miny_fixed).abs().log10()<5.0  {
                    node::Text::new(format!(
                        "{0:.1$}",
                        p + miny_fixed,
                        precision
                    ))
                }else{
                    node::Text::new(format!(
                        "{0:.1$e}",
                        p + miny_fixed,
                        precision
                    ))
                };
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
        ) in self.plots.into_iter().enumerate()
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

            let it = plots.into_iter();

            let it = it.map(|[x, y]| {
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
                        writeln!(&mut points, "{},{}", x, y).unwrap();
                    }
                    doc = doc.add(
                        Polyline::new()
                            .set("class", format!("plot{}color", i))
                            .set("fill", "none")
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
                                .set("width", format!("{}", (x - lx) - padding * 0.02))
                                .set("height", format!("{}", (height - paddingy - ly))) //TODO ugly?
                                .set("class", format!("plot{}fill", i));

                            doc = doc.add(k);
                        }
                        last = Some((x, y))
                    }
                }
                PlotType::LineFill => {
                    let mut it = it;
                    if let Some([startx, starty]) = it.next() {
                        let mut data = Data::new().move_to((startx, starty));
                        for [x, y] in it {
                            data = data.line_to((x, y));
                        }

                        data = data.close();

                        doc = doc.add(
                            Path::new()
                                .set("d", data)
                                .set("class", format!("plot{}fill", i)),
                        );
                    }
                }
            }
        }

        doc = doc.add(
            element::Text::new()
                .add(node::Text::new(self.title))
                .set("x", format!("{}", width / 2.0))
                .set("y", format!("{}", padding / 4.0))
                .set("alignment-baseline", "start")
                .set("text-anchor", "middle")
                .set("font-size", "x-large")
                .set("class", "ptext"),
        );

        doc = doc.add(
            element::Text::new()
                .add(node::Text::new(self.xname))
                .set("x", format!("{}", width / 2.0))
                .set("y", format!("{}", height - padding / 5.))
                .set("alignment-baseline", "start")
                .set("text-anchor", "middle")
                .set("font-size", "large")
                .set("class", "ptext"),
        );

        doc = doc.add(
            element::Text::new()
                .add(node::Text::new(self.yname))
                .set("x", format!("{}", padding / 3.0))
                .set("y", format!("{}", height / 2.0))
                .set("alignment-baseline", "start")
                .set("text-anchor", "middle")
                .set(
                    "transform",
                    format!("rotate(-90,{},{})", padding / 3.0, height / 2.0),
                )
                .set("font-size", "large")
                .set("class", "ptext"),
        );

        let data = Data::new()
            .move_to((padding, paddingy))
            .line_to((padding, height - paddingy))
            .line_to((width - padding, height - paddingy));

        let vert_line = Path::new()
            .set("style", "fill:none !important;")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data)
            .set("class", "pline");

        doc.add(vert_line)
    }
}

fn find_good_step(num_steps: usize, range: f32) -> (usize, f32, f32) {
    //https://stackoverflow.com/questions/237220/tickmark-algorithm-for-a-graph-axis

    let rough_step = range / (num_steps - 1) as f32;

    let step_power = 10.0f32.powf(-rough_step.abs().log10().floor()) as f32;
    let normalized_step = rough_step * step_power;
    
    let good_steps = [1.0, 2.0, 5.0, 10.0];
    let good_normalized_step = good_steps.iter().find(|a| **a > normalized_step).unwrap();
    //dbg!(good_normalized_step);

    let step = good_normalized_step / step_power;

    let new_step = if range % step != 0.0 {
        (range / step) as usize + 1
    } else {
        (range / step) as usize
    };

    (new_step, step_power.log10(), step)
}

fn find_bounds(it: impl IntoIterator<Item = [f32; 2]>) -> Option<[f32; 4]> {
    let mut ii = it.into_iter();
    if let Some([x, y]) = ii.next() {
        let mut val = [x, x, y, y];
        ii.fold(&mut val, |val, [x, y]| {
            if x < val[0] {
                val[0] = x;
            } else if x > val[1] {
                val[1] = x;
            }
            if y < val[2] {
                val[2] = y;
            } else if y > val[3] {
                val[3] = y;
            }
            val
        });
        Some(val)
    } else {
        None
    }
}
