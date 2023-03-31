use super::*;

#[derive(Copy, Clone)]
pub struct Area<X, Y> {
    x: Option<[X; 2]>,
    y: Option<[Y; 2]>,
}

impl<X: PlotNum, Y: PlotNum> Area<X, Y> {
    pub(crate) fn new() -> Area<X, Y> {
        Area { x: None, y: None }
    }

    pub fn grow_area(&mut self, other: &Area<X, Y>) {
        match (&mut self.x, &other.x) {
            (Some([a_min, a_max]), Some([b_min, b_max])) => {
                if *b_max > *a_max {
                    *a_max = *b_max;
                }
                if *b_min < *a_min {
                    *a_min = *b_min;
                }
            }
            (Some(_), None) => {
                //do nothing
            }
            (None, Some(a)) => {
                self.x = Some(*a);
            }
            (None, None) => {
                //do nothing
            }
        }

        match (&mut self.y, &other.y) {
            (Some([a_min, a_max]), Some([b_min, b_max])) => {
                if *b_max > *a_max {
                    *a_max = *b_max;
                }
                if *b_min < *a_min {
                    *a_min = *b_min;
                }
            }
            (Some(_), None) => {
                //do nothing
            }
            (None, Some(a)) => {
                self.y = Some(*a);
            }
            (None, None) => {
                //do nothing
            }
        }
    }

    #[inline(always)]
    pub fn grow(&mut self, x: Option<&X>, y: Option<&Y>) {
        if let Some(x) = x {
            if !x.is_hole() {
                match &mut self.x {
                    None => self.x = Some([*x, *x]),
                    Some([min, max]) => {
                        if *x < *min {
                            self.x = Some([*x, *max]);
                        } else if *x > *max {
                            self.x = Some([*min, *x]);
                        }
                    }
                }
            }
        }

        if let Some(y) = y {
            if !y.is_hole() {
                match &mut self.y {
                    None => self.y = Some([*y, *y]),
                    Some([min, max]) => {
                        if *y < *min {
                            self.y = Some([*y, *max]);
                        } else if *y > *max {
                            self.y = Some([*min, *y]);
                        }
                    }
                }
            }
        }
    }

    pub(crate) fn build(self) -> (DataBound<X>, DataBound<Y>) {
        let x = match self.x {
            None => X::unit_range(None),
            Some([min, max]) => {
                if min == max {
                    X::unit_range(Some(min))
                } else {
                    [min, max]
                }
            }
        };

        let y = match self.y {
            None => Y::unit_range(None),
            Some([min, max]) => {
                if min == max {
                    Y::unit_range(Some(min))
                } else {
                    [min, max]
                }
            }
        };

        assert!(x[0] != x[1]);
        assert!(y[0] != y[1]);
        (
            DataBound {
                min: x[0],
                max: x[1],
            },
            DataBound {
                min: y[0],
                max: y[1],
            },
        )
    }
}
