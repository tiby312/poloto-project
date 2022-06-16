use crate::plotnum::PlotNum;

use super::{marker::Area, unwrapper::Unwrapper};

//pub mod bounded_iter;
//pub mod buffered_iter;

//handle must be called BEFORE the iterator is iterated on.
pub trait PlotIter<X, Y> {
    fn next(&mut self) -> Option<(X, Y)>;
    fn handle(&mut self, area: &mut Area<X, Y>);
}

#[derive(Clone)]
pub struct UnwrapperIter<I>(pub I);
impl<I: Iterator> Iterator for UnwrapperIter<I>
where
    I::Item: Unwrapper,
{
    type Item = <I::Item as Unwrapper>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.unwrap())
    }
}

pub trait IterBuilder<X: PlotNum, Y: PlotNum>: Iterator + Sized
where
    Self::Item: Unwrapper<Item = (X, Y)>,
{
    fn cloned_plot(self) -> ClonedIter<UnwrapperIter<Self>>;
    fn buffered_plot(self) -> BufferedIter<UnwrapperIter<Self>, (X, Y)>;

    fn rect_bound_plot(
        self,
        x: [X; 2],
        y: [Y; 2],
    ) -> KnownBounds<std::vec::IntoIter<(X, Y)>, UnwrapperIter<Self>> {
        let a = vec![(x[0], y[0]), (x[1], y[1])];
        KnownBounds(a.into_iter(), UnwrapperIter(self))
    }

    fn custom_bound_plot<I: Iterator>(
        self,
        bound: I,
    ) -> KnownBounds<UnwrapperIter<I>, UnwrapperIter<Self>>
    where
        I::Item: Unwrapper<Item = (X, Y)>,
    {
        KnownBounds(UnwrapperIter(bound), UnwrapperIter(self))
    }
}

impl<X: PlotNum, Y: PlotNum, I: Iterator> IterBuilder<X, Y> for I
where
    I::Item: Unwrapper<Item = (X, Y)>,
{
    fn cloned_plot(self) -> ClonedIter<UnwrapperIter<Self>> {
        ClonedIter(UnwrapperIter(self))
    }
    fn buffered_plot(self) -> BufferedIter<UnwrapperIter<Self>, (X, Y)> {
        BufferedIter(Floo::First(UnwrapperIter(self)))
    }
}

#[derive(Clone)]
pub struct ClonedIter<T>(T);

impl<X: PlotNum, Y: PlotNum, T: Iterator<Item = (X, Y)> + Clone> PlotIter<X, Y> for ClonedIter<T> {
    fn next(&mut self) -> Option<(X, Y)> {
        self.0.next()
    }
    fn handle(&mut self, area: &mut Area<X, Y>) {
        for (x, y) in self.0.clone() {
            area.grow(Some(x), Some(y));
        }
    }
}

#[derive(Clone)]
pub struct KnownBounds<T, K>(T, K);
impl<X: PlotNum, Y: PlotNum, T: Iterator<Item = (X, Y)>, K: Iterator<Item = (X, Y)>> PlotIter<X, Y>
    for KnownBounds<T, K>
{
    fn next(&mut self) -> Option<(X, Y)> {
        self.1.next()
    }
    fn handle(&mut self, area: &mut Area<X, Y>) {
        for (x, y) in &mut self.0 {
            area.grow(Some(x), Some(y));
        }
    }
}

#[derive(Clone)]
enum Floo<I: Iterator<Item = T>, T> {
    First(I),
    Next(std::vec::IntoIter<T>),
}

#[derive(Clone)]
pub struct BufferedIter<I: Iterator<Item = T>, T>(Floo<I, T>);
impl<X: PlotNum, Y: PlotNum, T: Iterator<Item = (X, Y)>> PlotIter<X, Y>
    for BufferedIter<T, (X, Y)>
{
    fn next(&mut self) -> Option<(X, Y)> {
        if let Floo::Next(n) = &mut self.0 {
            n.next()
        } else {
            panic!("incorrect trait usage");
        }
    }
    fn handle(&mut self, area: &mut Area<X, Y>) {
        if let Floo::First(a) = &mut self.0 {
            let mut vec = Vec::with_capacity(a.size_hint().0);
            for (x, y) in a {
                area.grow(Some(x), Some(y));
                vec.push((x, y));
            }
            self.0 = Floo::Next(vec.into_iter())
        } else {
            panic!("handle() called incorrectly")
        }
    }
}
