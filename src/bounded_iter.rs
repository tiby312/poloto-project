//!
//! Create a [`PlotIter`] from a plot with min/max bounds known beforehand.
//!
//! Typically, in order to find min and max bounds, iterators passed to poloto are iterated through twice.
//! However, in some cases, the user might already know the bounds, making the first iteration pointless.
//! In this case, consider using [`from_bounds`]. In short, it will make the first iteration of [`PlotIter`] just return
//! two points. The smallest 2d point, and the biggest 2d point.
//!
use super::*;

pub struct KnownBounds<I1, I2> {
    iter1: Option<I1>,
    iter2: I2,
}
impl<I1: Iterator, I2: Iterator<Item = I1::Item>> PlotIter for KnownBounds<I1, I2> {
    type Item = I1::Item;
    type It1 = I1;
    type It2 = I2;
    fn first(&mut self) -> Self::It1 {
        self.iter1.take().unwrap()
    }
    fn second(self, _: Self::It1) -> Self::It2 {
        self.iter2
    }
}

use crate::plottable::FromOrig;
pub fn from_bounds<
    X: PlotNum,
    Y: PlotNum,
    T: Plottable<Item = (X, Y)> + FromOrig<X = X, Y = Y>,
    I: Iterator<Item = T>,
>(
    x: [X; 2],
    y: [Y; 2],
    iter: I,
) -> KnownBounds<std::vec::IntoIter<T>, I> {
    let min = T::from_orig(x[0], y[0]);
    let max = T::from_orig(x[1], y[1]);

    KnownBounds {
        iter1: Some(vec![min, max].into_iter()),
        iter2: iter,
    }
}
