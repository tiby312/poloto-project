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
