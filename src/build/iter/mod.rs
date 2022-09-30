use std::iter::FusedIterator;

use crate::plotnum::PlotNum;

use super::unwrapper::Unwrapper;

#[derive(Clone)]
pub struct UnwrapperIter<I>(pub I);
impl<I: ExactSizeIterator> ExactSizeIterator for UnwrapperIter<I> where I::Item: Unwrapper {}
impl<I: FusedIterator> FusedIterator for UnwrapperIter<I> where I::Item: Unwrapper {}
impl<I: Iterator> Iterator for UnwrapperIter<I>
where
    I::Item: Unwrapper,
{
    type Item = <I::Item as Unwrapper>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.unwrap())
    }
}

use super::SinglePlotBuilder;
impl<X: PlotNum, Y: PlotNum, I: Iterator> IterBuilder<X, Y> for I where
    I::Item: Unwrapper<Item = (X, Y)>
{
}

pub trait IterBuilder<X: PlotNum, Y: PlotNum>: Iterator + Sized
where
    Self::Item: Unwrapper<Item = (X, Y)>,
{
    fn cloned_plot(self) -> SinglePlotBuilder<X, Y, UnwrapperIter<Self>>
    where
        Self: Clone,
    {
        SinglePlotBuilder::new_cloned(UnwrapperIter(self))
    }

    fn buffered_plot(self) -> SinglePlotBuilder<X, Y, std::vec::IntoIter<(X, Y)>>
    where
        Self::Item: Clone,
    {
        SinglePlotBuilder::new_buffered(UnwrapperIter(self))
    }
}
