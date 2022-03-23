use super::*;

pub trait Markerable: PlotIterator<Item = (Self::X, Self::Y)> {
    type X;
    type Y;
    ///
    /// Specify x and y values that must fit into the viewport.
    ///
    fn markers<XI: IntoIterator<Item = Self::X>, YI: IntoIterator<Item = Self::Y>>(
        self,
        xmarkers: XI,
        ymarkers: YI,
    ) -> MarkersStruct<Self, XI::IntoIter, YI::IntoIter>
    where
        Self: Sized,
    {
        MarkersStruct {
            plots: self,
            xmarkers: xmarkers.into_iter(),
            ymarkers: ymarkers.into_iter(),
        }
    }
}
impl<X, Y, I: PlotIterator<Item = (X, Y)>> Markerable for I {
    type X = X;
    type Y = Y;
}

///
/// Specify x and y values that must fit into the viewport.
///
/// Also consider [`Markerable::markers()`]
///
pub fn markers<
    X,
    Y,
    P: PlotIterator<Item = (X, Y)>,
    XI: IntoIterator<Item = X>,
    YI: IntoIterator<Item = Y>,
>(
    plots: P,
    x: XI,
    y: YI,
) -> MarkersStruct<P, XI::IntoIter, YI::IntoIter> {
    MarkersStruct {
        plots,
        xmarkers: x.into_iter(),
        ymarkers: y.into_iter(),
    }
}

pub trait PlotIteratorAndMarkers {
    type X;
    type Y;
    type Iter: PlotIterator<Item = (Self::X, Self::Y)>;
    type XI: Iterator<Item = Self::X>;
    type YI: Iterator<Item = Self::Y>;
    fn unpack(self) -> (Self::Iter, Self::XI, Self::YI);
}

impl<X: PlotNum, Y: PlotNum, I: PlotIterator<Item = (X, Y)>> PlotIteratorAndMarkers for I {
    type X = X;
    type Y = Y;
    type Iter = Self;
    type XI = std::iter::Empty<X>;
    type YI = std::iter::Empty<Y>;
    fn unpack(self) -> (Self::Iter, Self::XI, Self::YI) {
        (self, std::iter::empty(), std::iter::empty())
    }
}

pub struct MarkersStruct<I: PlotIterator<Item = (XI::Item, YI::Item)>, XI: Iterator, YI: Iterator> {
    plots: I,
    xmarkers: XI,
    ymarkers: YI,
}

impl<I: PlotIterator<Item = (XI::Item, YI::Item)>, XI: Iterator, YI: Iterator>
    PlotIteratorAndMarkers for MarkersStruct<I, XI, YI>
{
    type X = XI::Item;
    type Y = YI::Item;
    type Iter = I;
    type XI = XI;
    type YI = YI;
    fn unpack(self) -> (Self::Iter, Self::XI, Self::YI) {
        (self.plots, self.xmarkers, self.ymarkers)
    }
}
