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


impl<A: AsPlotnum, B: AsPlotnum> Unwrapper for (A, B) {
    type Item = (A::Target, B::Target);
    fn unwrap(self) -> (A::Target, B::Target) {
        let (a, b) = self;
        (*a.as_plotnum(), *b.as_plotnum())
    }
}

impl<A: AsPlotnum, B: AsPlotnum> Unwrapper for &(A, B) {
    type Item = (A::Target, B::Target);
    fn unwrap(self) -> (A::Target, B::Target) {
        let (a, b) = self;
        (*a.as_plotnum(), *b.as_plotnum())
    }
}
