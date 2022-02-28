use crate::*;

mod render_base;
mod render_plots;

#[derive(Copy, Clone)]
pub struct Canvas {
    pub ideal_num_xsteps: u32,
    pub ideal_num_ysteps: u32,
    pub ideal_dash_size: f64,
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
    xaspect_offset: f64,
    yaspect_offset: f64,
    pub scalex: f64,
    pub scaley: f64,
    spacing: f64,
    legendx1: f64,
    num_css_classes: Option<usize>,
}
impl Canvas {
    pub fn get_dim(&self) -> [f64; 2] {
        [self.width, self.height]
    }
    pub fn with_options<X: PlotNum, Y: PlotNum>(
        boundx: [X; 2],
        boundy: [Y; 2],
        dim: Option<[f64; 2]>,
        preserve_aspect: bool,
        num_css_classes: Option<usize>,
    ) -> Self {
        let (width, height) = if let Some([x, y]) = dim {
            (x, y)
        } else {
            (crate::WIDTH as f64, crate::HEIGHT as f64)
        };

        let ideal_dash_size = 20.0;
        let padding = 150.0;
        let paddingy = 100.0;

        //The range over which the data will be scaled to fit
        let (scalex, scaley) = if preserve_aspect {
            if width > height {
                (height - paddingy * 2.0, height - paddingy * 2.0)
            } else {
                (width - padding * 2.0, width - padding * 2.0)
            }
        } else {
            (width - padding * 2.0, height - paddingy * 2.0)
        };

        let [minx, maxx] = boundx;
        let [miny, maxy] = boundy;

        let distancex_min_to_max =
            maxx.scale([minx, maxx], scalex) - minx.scale([minx, maxx], scalex);
        let distancey_min_to_max =
            maxy.scale([miny, maxy], scaley) - miny.scale([miny, maxy], scaley);

        let (xaspect_offset, yaspect_offset) = if preserve_aspect {
            if width > height {
                (-padding + width / 2.0 - distancey_min_to_max / 2.0, 0.0)
            } else {
                (
                    0.0,
                    -height + paddingy + height / 2.0 + distancey_min_to_max / 2.0,
                )
            }
        } else {
            (0.0, 0.0)
        };

        let ideal_xtick_spacing = 80.0;

        let ideal_ytick_spacing = 60.0;

        let ideal_num_xsteps = (distancex_min_to_max / ideal_xtick_spacing).floor() as u32;
        let ideal_num_ysteps = (distancey_min_to_max / ideal_ytick_spacing).floor() as u32;
        let ideal_num_xsteps = ideal_num_xsteps.max(2);
        let ideal_num_ysteps = ideal_num_ysteps.max(2);

        let spacing = padding / 3.0;
        let legendx1 = width - padding / 1.2 + padding / 30.0;

        Canvas {
            ideal_num_xsteps,
            ideal_num_ysteps,
            ideal_dash_size,
            width,
            height,
            padding,
            paddingy,
            xaspect_offset,
            yaspect_offset,
            scalex,
            scaley,
            spacing,
            legendx1,
            num_css_classes,
        }
    }

    pub fn render_plots<
        XI: IntoIterator,
        YI: IntoIterator,
        PF: PlotFmt<X = XI::Item, Y = YI::Item>,
    >(
        writer: impl std::fmt::Write,
        plotter: &mut Plotter<XI, YI, PF>,
    ) -> std::fmt::Result
    where
        XI::Item: PlotNum,
        YI::Item: PlotNum,
    {
        render_plots::render_plots(writer, plotter)
    }

    pub fn render_base<
        XI: IntoIterator,
        YI: IntoIterator,
        PF: PlotFmt<X = XI::Item, Y = YI::Item>,
    >(
        writer: impl std::fmt::Write,
        plotter: &mut Plotter<XI, YI, PF>,
    ) -> std::fmt::Result
    where
        XI::Item: PlotNum,
        YI::Item: PlotNum,
    {
        render_base::render_base(writer, plotter)
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
