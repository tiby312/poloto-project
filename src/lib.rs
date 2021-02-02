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
        Box::new(self.0.as_ref().unwrap().clone())
    }

    #[inline(always)]
    fn into_iter(&mut self) -> Box<dyn Iterator<Item = [f32; 2]> + 'a> {
        Box::new(self.0.take().unwrap())
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

    fn setup_axis(&self, mut doc: Document, width: f32, height: f32, padding: f32) -> Document {
        use svg::node::element::Rectangle;

        doc = doc.add(
            Rectangle::new()
                .set("class", "pbackground")
                .set("x", "0")
                .set("y", "0")
                .set("width", format!("{}", width))
                .set("height", format!("{}", height)),
        );

        let data = node::Text::new(format!("{}", self.title));
        let k = element::Text::new()
            .add(data)
            .set("x", format!("{}", width / 2.0))
            .set("y", format!("{}", padding / 4.0));
        let k = k
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle");
        let k = k.set("font-size", "x-large").set("class", "ptext");
        doc = doc.add(k);

        let data = node::Text::new(format!(
            "<tspan class=\"ptext_bold\">X</tspan>:  {}",
            self.xname
        ));
        let k = element::Text::new()
            .add(data)
            .set("x", format!("{}", width / 2.0))
            .set("y", format!("{}", padding / 2.0));
        let k = k
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle");
        let k = k.set("font-size", "large").set("class", "ptext");
        doc = doc.add(k);

        let data = node::Text::new(format!(
            "<tspan class=\"ptext_bold\">Y</tspan>:  {}",
            self.yname
        ));
        let k = element::Text::new()
            .add(data)
            .set("x", format!("{}", width / 2.0))
            .set("y", format!("{}", padding / 1.5));
        let k = k
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle");
        let k = k.set("font-size", "large").set("class", "ptext");
        doc = doc.add(k);

        let data = node::Text::new("X");
        let k = element::Text::new()
            .add(data)
            .set("x", format!("{}", width / 2.0))
            .set("y", format!("{}", height - padding / 5.));
        let k = k
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle");
        let k = k.set("font-size", "large").set("class", "ptext_bold");
        doc = doc.add(k);

        let data = node::Text::new("Y");
        let k = element::Text::new()
            .add(data)
            .set("x", format!("{}", padding / 5.0))
            .set("y", format!("{}", height / 2.));
        let k = k
            .set("alignment-baseline", "start")
            .set("text-anchor", "middle");
        let k = k.set("font-size", "large").set("class", "ptext_bold");
        doc = doc.add(k);

        let data = Data::new()
            .move_to((padding, padding))
            .line_to((padding, height - padding))
            .line_to((width - padding, height - padding));

        let vert_line = Path::new()
            .set("style", "fill:none !important;")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data)
            .set("class", "pline");

        doc.add(vert_line)
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
        let height = 600.0;
        let padding = 150.0;

        let mut doc = Document::new()
            .set("width", width)
            .set("height", height)
            .set("viewBox", (0, 0, width, height))
            .set("class", "splotclass")
            .set("id", "splot");

        let text_color = "#000000";
        let background_color = "#FFFFFF";
        /*
        const COLOR_TABLE: [usize; 6] =
            [0x2b2255, ed1c1c, 0x0000FF, 0x445522, 0x558833, 0xFF0045];

        let colors: Vec<_> = COLOR_TABLE
            .iter()
            .map(|color| format!("#{:06x?}", color))
            .collect();
        */
        let colors=vec!("blue","red","green","purple","yellow","aqua");

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

        doc = self.setup_axis(doc, width, height, padding);

        let [minx, maxx, miny, maxy] =
            if let Some(m) = find_bounds(self.plots.iter().flat_map(|a| a.plots.ref_iter())) {
                m
            } else {
                //TODO test that this looks ok
                return doc; //No plots at all. dont need to draw anything
            };

        let scalex = (width - padding * 2.0) / (maxx - minx);
        let scaley = (height - padding * 2.0) / (maxy - miny);

        {
            //Draw step lines
            //https://stackoverflow.com/questions/60497397/how-do-you-format-a-float-to-the-first-significant-decimal-and-with-specified-pr

            let num_steps = 10;
            let texty_padding = padding * 0.2;
            let textx_padding = padding * 0.4;

            let (xstep_num, xstep_power, xstep) = find_good_step(num_steps, maxx - minx);
            let (ystep_num, ystep_power, ystep) = find_good_step(num_steps, maxy - miny);

            let minx_fixed = (minx / xstep).ceil() * xstep;
            let miny_fixed = (miny / ystep).ceil() * ystep;
            //dbg!(xstep,xstep_num,ystep,ystep_num,xstep_power,ystep_power);

            for a in 0..xstep_num {
                let p = (a as f32) * xstep;

                let precision = (1.0 + xstep_power).max(0.0) as usize;
                let data = node::Text::new(format!("{0:.1$}", p + minx_fixed, precision));
                let k = element::Text::new()
                    .add(data)
                    .set("x", format!("{}", p * scalex + padding))
                    .set("y", format!("{}", height - padding + textx_padding));
                let k = k
                    .set("alignment-baseline", "start")
                    .set("text-anchor", "middle")
                    .set("class", "ptext");
                doc = doc.add(k);
            }

            for a in 0..ystep_num {
                let p = (a as f32) * ystep;

                //dbg!(p,miny,miny_fixed,p+miny_fixed);
                let precision = (1.0 + ystep_power).max(0.0) as usize;
                let data = node::Text::new(format!("{0:.1$}", p + miny_fixed, precision));
                let k = element::Text::new()
                    .add(data)
                    .set("x", format!("{}", padding - texty_padding))
                    .set("y", format!("{}", height - p * scaley - padding));
                let k = k
                    .set("alignment-baseline", "middle")
                    .set("text-anchor", "end")
                    .set("class", "ptext");
                doc = doc.add(k);
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
            //let color = COLOR_TABLE[i % (COLOR_TABLE.len())];
            //println!("{:x}",color);
            //Draw legend

            let spacing = padding / 3.0;
            let data = node::Text::new(name);
            let k = element::Text::new()
                .add(data)
                .set("x", format!("{}", width - padding / 1.2))
                .set("y", format!("{}", padding + (i as f32) * spacing));
            let k = k
                .set("alignment-baseline", "middle")
                .set("text-anchor", "start");
            let k = k.set("font-size", "large").set("class", "ptext");
            doc = doc.add(k);

            //dbg!(format!("#{:08x?}",color));
            let k = element::Circle::new()
                //.set("fill", format!("#{:06x?}", color))
                .set("cx", format!("{}", width - padding / 1.2 + padding / 30.0))
                .set(
                    "cy",
                    format!("{}", padding - padding / 8.0 + (i as f32) * spacing),
                )
                .set("r", format!("{}", padding / 30.0))
                .set("class", format!("plot{}fill", i));
            doc = doc.add(k);

            let it = plots.into_iter();

            let it = it.map(|[x, y]| {
                [
                    padding + (x - minx) * scalex,
                    height - padding - (y - miny) * scaley,
                ]
            });

            match plot_type {
                PlotType::Line => {
                    let mut data = Polyline::new()
                        .set("class", format!("plot{}color", i))
                        .set("fill", "none")
                        //.set("stroke", format!("#{:06x?}", color))
                        .set("stroke-width", 2);

                    use std::fmt::Write;
                    let mut points = String::new();
                    for [x, y] in it {
                        write!(&mut points, "{},{}\n", x, y).unwrap();
                    }

                    data = data.set("points", points);
                    doc = doc.add(data);
                }
                PlotType::Scatter => {
                    for [x, y] in it {
                        let k = element::Circle::new()
                            //.set("fill", format!("#{:06x?}", color))
                            .set("cx", format!("{}", x))
                            .set("cy", format!("{}", y))
                            .set("r", format!("{}", padding / 50.0))
                            .set("class", format!("plot{}fill", i));

                        doc = doc.add(k);
                    }
                }
                PlotType::Histo => {
                    let mut last = None;
                    for [x, y] in it {
                        if let Some((lx, ly)) = last {
                            let k = element::Rectangle::new()
                                //.set("fill", format!("#{:06x?}", color))
                                .set("x", format!("{}", lx))
                                .set("y", format!("{}", ly))
                                .set("width", format!("{}", (x - lx) - padding * 0.02))
                                .set("height", format!("{}", (height - padding - ly))) //TODO ugly?
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

                        let linefill = Path::new()
                            .set("d", data)
                            .set("class", format!("plot{}fill", i));

                        doc = doc.add(linefill);
                    }
                }
            }
        }

        doc
    }
}

fn find_good_step(num_steps: usize, range: f32) -> (usize, f32, f32) {
    //https://stackoverflow.com/questions/237220/tickmark-algorithm-for-a-graph-axis

    let rough_step = range / (num_steps - 1) as f32;

    let step_power = 10.0f32.powf(-rough_step.abs().log10().floor()) as f32;
    let normalized_step = rough_step * step_power;
    //dbg!(normalized_step);

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
