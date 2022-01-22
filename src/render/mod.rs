use crate::*;

mod render_helper;
use render_helper::*;

use std::fmt;
struct DrawData {
    width: f64,
    height: f64,
    padding: f64,
    paddingy: f64,
}
struct ScaleData<X: PlotNum, Y: PlotNum> {
    minx: X,
    maxx: X,
    miny: Y,
    maxy: Y,
    scalex: f64,
    scaley: f64,
    preserve_aspect: bool,
    aspect_offset: f64,
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


/*
//Returns error if the user supplied format functions don't work.
//Panics if the element tag writing writes fail
pub fn render<X: PlotNum, Y: PlotNum, T: std::fmt::Write>(
    plotter: &mut Plotter<X, Y>,
    writer: T,
) -> fmt::Result {
    let mut writer = tagger::new(writer);

    let width = crate::WIDTH as f64;
    let height = crate::HEIGHT as f64;
    let padding = 150.0;
    let paddingy = 100.0;

    let xcontext: &mut dyn PlotNumContext<Num = X> = plotter.xcontext.as_mut().unwrap().as_mut();
    let ycontext: &mut dyn PlotNumContext<Num = Y> = plotter.ycontext.as_mut().unwrap().as_mut();

    let ([minx, maxx], [miny, maxy]) = num::find_bounds(
        xcontext,
        ycontext,
        plotter.plots.iter_mut().flat_map(|x| x.plots.iter_first()),
    );

    let preserve_aspect = plotter.preserve_aspect;

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
}

*/