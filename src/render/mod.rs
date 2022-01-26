use crate::*;

pub struct Data<X, Y, J, K> {
    pub boundx: [X; 2],
    pub boundy: [Y; 2],
    pub tickx: TickInfo<X, J>,
    pub ticky: TickInfo<Y, K>,
}

mod render_base;
mod render_plots;

#[derive(Copy, Clone)]
pub struct Canvas {
    pub ideal_num_xsteps: u32,
    pub ideal_num_ysteps: u32,
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
    aspect_offset: f64,
    pub scalex: f64,
    pub scaley: f64,
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
        let scalex = if preserve_aspect {
            height - paddingy * 2.0
        } else {
            width - padding * 2.0
        };

        let scaley = height - paddingy * 2.0;

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
            scalex,
            scaley,
            spacing,
            legendx1,
            num_css_classes,
            preserve_aspect,
        }
    }

    pub fn render_plots<X: PlotNumContext, Y: PlotNumContext>(
        &self,
        writer: impl std::fmt::Write,
        plotter: &mut Plotter<X, Y>,
        data: &Data<X::Num, Y::Num, X::StepInfo, Y::StepInfo>,
    ) -> std::fmt::Result {
        render_plots::render_plots(self, writer, plotter, data)
    }

    pub fn render_base<X: PlotNumContext, Y: PlotNumContext>(
        &self,
        writer: impl std::fmt::Write,
        plotter: &mut Plotter<X, Y>,
        data: &mut Data<X::Num, Y::Num, X::StepInfo, Y::StepInfo>,
    ) -> std::fmt::Result {
        render_base::render_base(self, writer, plotter, data)
    }
}

pub fn line_fill<T: std::fmt::Write>(
    path: &mut tagger::PathBuilder<T>,
    mut it: impl Iterator<Item = [f64; 2]>,
    base_line: f64,
    add_start_end_base: bool,
) -> sfmt::Result {
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
) -> sfmt::Result {
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
