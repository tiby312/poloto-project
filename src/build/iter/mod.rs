pub mod bounded_iter;
pub mod buffered_iter;

///
/// Iterator that is accepted by plot functions like `line`,`scatter`, etc.
/// The second function will only get called after
/// the first iterator has been fully consumed.
///
pub trait PlotIter {
    type Item1;
    type Item2;
    type It1: Iterator<Item = Self::Item1>;
    type It2: Iterator<Item = Self::Item2>;

    /// Return an iterator that will be used to find min max bounds.
    fn first(&mut self) -> Self::It1;

    /// Return an iterator that returns the same data as before in order to scale the plots.
    fn second(self, last: Self::It1) -> Self::It2;
}

#[derive(Clone)]
pub struct ClonedIter<T>(pub T);

impl<I: Iterator + Clone> PlotIter for ClonedIter<I> {
    type Item1 = I::Item;
    type Item2 = I::Item;
    type It1 = I;
    type It2 = I;

    fn first(&mut self) -> Self::It1 {
        self.0.clone()
    }
    fn second(self, _last: Self::It1) -> Self::It2 {
        self.0
    }
}

pub trait IterBuilder: Iterator + Sized {
    fn buffered_plot(self) -> buffered_iter::VecBackedIter<Self>
    where
        Self::Item: Clone;
    fn cloned_plot(self) -> ClonedIter<Self>
    where
        Self: Clone;
    fn rect_bound_plot<X>(
        self,
        min: X,
        max: X,
    ) -> bounded_iter::KnownBounds<std::vec::IntoIter<X>, Self>;
    fn custom_bound_plot<II: Iterator<Item = Self::Item>>(
        self,
        bound: II,
    ) -> bounded_iter::KnownBounds<II, Self>;
}

impl<I: Iterator> IterBuilder for I {
    fn rect_bound_plot<X>(
        self,
        min: X,
        max: X,
    ) -> bounded_iter::KnownBounds<std::vec::IntoIter<X>, Self> {
        bounded_iter::from_rect(min, max, self)
    }
    fn custom_bound_plot<II: Iterator<Item = Self::Item>>(
        self,
        bound: II,
    ) -> bounded_iter::KnownBounds<II, Self> {
        bounded_iter::from_iter(bound, self)
    }
    fn buffered_plot(self) -> buffered_iter::VecBackedIter<Self>
    where
        Self::Item: Clone,
    {
        buffered_iter::buffered(self)
    }
    fn cloned_plot(self) -> ClonedIter<Self>
    where
        Self: Clone,
    {
        ClonedIter(self)
    }
}
