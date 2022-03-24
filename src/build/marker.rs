use super::*;

pub struct Area<X, Y> {
    x: Option<[X; 2]>,
    y: Option<[Y; 2]>,
}

impl<X: PlotNum, Y: PlotNum> Area<X, Y> {
    pub(crate) fn new() -> Area<X, Y> {
        Area { x: None, y: None }
    }

    #[inline(always)]
    pub fn grow(&mut self, x: Option<X>, y: Option<Y>) {
        if let Some(x) = x {
            if !x.is_hole() {
                match &mut self.x {
                    None => self.x = Some([x, x]),
                    Some([min, max]) => {
                        if x < *min {
                            self.x = Some([x, *max]);
                        } else if x > *max {
                            self.x = Some([*min, x]);
                        }
                    }
                }
            }
        }

        if let Some(y) = y {
            if !y.is_hole() {
                match &mut self.y {
                    None => self.y = Some([y, y]),
                    Some([min, max]) => {
                        if y < *min {
                            self.y = Some([y, *max]);
                        } else if y > *max {
                            self.y = Some([*min, y]);
                        }
                    }
                }
            }
        }
    }

    pub fn build(self) -> (DataBound<X>, DataBound<Y>) {
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

pub trait Markerable {
    type X: PlotNum;
    type Y: PlotNum;

    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>);

    fn markers<XI: IntoIterator<Item = Self::X>, YI: IntoIterator<Item = Self::Y>>(
        self,
        x: XI,
        y: YI,
    ) -> Marker<Self, XI::IntoIter, YI::IntoIter>
    where
        Self: Sized,
    {
        Marker {
            plots: self,
            x: x.into_iter(),
            y: y.into_iter(),
        }
    }
}

pub fn markers<P: Markerable<X = XI::Item, Y = YI::Item>, XI: IntoIterator, YI: IntoIterator>(
    plots: P,
    x: XI,
    y: YI,
) -> Marker<P, XI::IntoIter, YI::IntoIter> {
    plots.markers(x, y)
}

pub struct Marker<P, XI, YI> {
    plots: P,
    x: XI,
    y: YI,
}

impl<P: PlotIterator, XI, YI> PlotIterator for Marker<P, XI, YI> {
    type Item = P::Item;
    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.plots.next_typ()
    }
    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        self.plots.next_plot_point()
    }
    #[inline(always)]
    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result> {
        self.plots.next_name(w)
    }
}
impl<P: Markerable<X = XI::Item, Y = YI::Item>, XI: Iterator, YI: Iterator> Markerable
    for Marker<P, XI, YI>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
    type X = XI::Item;
    type Y = YI::Item;
    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>) {
        self.plots.increase_area(area);
        for a in &mut self.x {
            area.grow(Some(a), None);
        }
        for a in &mut self.y {
            area.grow(None, Some(a));
        }
    }
}
