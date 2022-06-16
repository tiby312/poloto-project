//!
//! Create a [`PlotIter`] from an iterator that does not implement `Clone`.
//!
//! If you have an iterator where each call to next() does
//! an expensive calculation, using a buffered iterator might be desirable.
//!
//! [`PlotIter`] has a blanket impl on all iterators that implement clone.
//! If an iterator isn't clonable, we can instead iterate over it once,
//! but as we do, clone the items and store them in a vec, to be used
//! the next time we need to iterate over everything. The [`buffered`]
//! function takes an unclonable iterator and returns a [`PlotIter`]
//!
//!

use super::PlotIter;

///
/// Create a [`PlotIter`] from an iterator that does not implement `Clone`.
///
pub fn buffered<I: Iterator>(it: I) -> VecBackedIter<I>
where
    I::Item: Clone,
{
    VecBackedIter::new(it)
}

///
/// Used by [`VecBackedIter`]
///
pub struct WrapperIter<I: Iterator> {
    vec: Vec<I::Item>,
    iter: I,
}
impl<I: Iterator> Iterator for WrapperIter<I>
where
    I::Item: Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        if let Some(b) = self.iter.next() {
            self.vec.push(b.clone());
            Some(b)
        } else {
            None
        }
    }
}

///
/// Used by [`buffered`]
///
#[derive(Clone)]
pub struct VecBackedIter<I: Iterator> {
    iter: Option<I>,
}
impl<I: Iterator> VecBackedIter<I>
where
    I::Item: Clone,
{
    pub fn new(iter: I) -> Self {
        VecBackedIter { iter: Some(iter) }
    }
}
impl<I: Iterator> PlotIter for VecBackedIter<I>
where
    I::Item: Clone,
{
    type Item1 = I::Item;
    type Item2 = I::Item;
    type It1 = WrapperIter<I>;
    type It2 = std::vec::IntoIter<I::Item>;
    fn first(&mut self) -> Self::It1 {
        WrapperIter {
            vec: vec![],
            iter: self.iter.take().unwrap(),
        }
    }
    fn second(self, last: Self::It1) -> Self::It2 {
        last.vec.into_iter()
    }
}
