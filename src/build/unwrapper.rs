//!
//! Contains the [`Unwrapper`] trait and adapters that work on it.
//!

use super::*;

///
/// Used to allow the user to pass both T and &T
///
pub trait Unwrapper {
    type Item;
    /// Produce one plot
    fn unwrap(self) -> Self::Item;
}

impl<T: PlotNum> Unwrapper for [T; 2] {
    type Item = (T, T);
    fn unwrap(self) -> (T, T) {
        let [x, y] = self;
        (x, y)
    }
}

impl<T: PlotNum> Unwrapper for &[T; 2] {
    type Item = (T, T);
    fn unwrap(self) -> (T, T) {
        let [x, y] = *self;
        (x, y)
    }
}

pub struct RefVal<'a, A, B>(pub &'a A, pub B);

impl<A: PlotNum, B: PlotNum> Unwrapper for RefVal<'_, A, B> {
    type Item = (A, B);
    fn unwrap(self) -> (A, B) {
        let RefVal(a, b) = self;
        (*a, b)
    }
}

pub struct ValRef<'a, A, B>(pub A, pub &'a B);

impl<A: PlotNum, B: PlotNum> Unwrapper for ValRef<'_, A, B> {
    type Item = (A, B);
    fn unwrap(self) -> (A, B) {
        let ValRef(a, b) = self;
        (a, *b)
    }
}

impl<A: PlotNum, B: PlotNum> Unwrapper for (A, B) {
    type Item = (A, B);
    fn unwrap(self) -> (A, B) {
        self
    }
}

impl<A: PlotNum, B: PlotNum> Unwrapper for &(A, B) {
    type Item = (A, B);
    fn unwrap(self) -> (A, B) {
        *self
    }
}
