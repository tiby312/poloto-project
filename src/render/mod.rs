use crate::*;

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
