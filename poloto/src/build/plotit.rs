use super::*;

///
/// Return min max bounds as well as the points of one plot.
///
pub trait PlotIt {
    type L: Point;
    type It: Iterator<Item = Self::L>;
    fn unpack(self, area: &mut Area<<Self::L as Point>::X, <Self::L as Point>::Y>) -> Self::It;
}

#[derive(Copy, Clone)]
pub struct ClonedPlotIt<I>(I);

impl<L: Point, I: Iterator + Clone> ClonedPlotIt<I>
where
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    pub fn new(it: I) -> Self {
        Self(it)
    }
}

impl<L: Point, I: Iterator + Clone> PlotIt for ClonedPlotIt<I>
where
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    type L = L;
    type It = build::unwrapper::UnwrapperIter<I>;

    fn unpack(self, area: &mut Area<L::X, L::Y>) -> Self::It {
        let it = self.0;
        for k in it.clone() {
            let l = k.unwrap();
            let (x, y) = l.get();
            area.grow(Some(x), Some(y));
        }
        build::unwrapper::UnwrapperIter(it)
    }
}

impl<L: Point, I: IntoIterator> PlotIt for I
where
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    type L = L;
    type It = std::vec::IntoIter<L>;

    fn unpack(self, area: &mut Area<L::X, L::Y>) -> Self::It {
        let it = self.into_iter();

        let vec: Vec<_> = it.map(|j| j.unwrap()).collect();

        for l in vec.iter() {
            let (x, y) = l.get();
            area.grow(Some(x), Some(y));
        }

        vec.into_iter()
    }
}
