//!
//! Create a [`PlotIter`] from a plot with min/max bounds known beforehand.
//!
//! Typically, in order to find min and max bounds, iterators passed to poloto are iterated through twice.
//! However, in some cases, the user might already know the bounds, making the first iteration pointless.
//! In this case, consider using [`from_rect`] or [`from_iter`]. [`from_rect`] will make the first iteration of [`PlotIter`] just return
//! two points. The smallest 2d point, and the biggest 2d point. [`from_iter`] gives you more control as to how you want to define the bounds,
//! maybe a subset of the second iterator for example.
//!
use super::*;

#[derive(Clone)]
pub struct KnownBounds<I1, I2> {
    iter1: Option<I1>,
    iter2: I2,
}
impl<I1: Iterator, I2: Iterator> PlotIter for KnownBounds<I1, I2> {
    type Item1 = I1::Item;
    type Item2 = I2::Item;
    type It1 = I1;
    type It2 = I2;
    fn first(&mut self) -> Self::It1 {
        self.iter1.take().unwrap()
    }
    fn second(self, _: Self::It1) -> Self::It2 {
        self.iter2
    }
}

pub fn from_iter<I1: Iterator, I2: Iterator>(iter1: I1, iter2: I2) -> KnownBounds<I1, I2> {
    KnownBounds {
        iter1: Some(iter1),
        iter2,
    }
}

pub fn from_rect<X, I: Iterator>(min: X, max: X, iter: I) -> KnownBounds<std::vec::IntoIter<X>, I> {
    from_iter(vec![min, max].into_iter(), iter)
}
