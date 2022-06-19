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

pub trait AsPlotnum {
    type Into: PlotNum;
    fn as_plotnum(&self) -> &Self::Into;
}
impl<P: PlotNum> AsPlotnum for P {
    type Into = P;
    fn as_plotnum(&self) -> &Self::Into {
        self
    }
}

impl<A: AsPlotnum, B: AsPlotnum> Unwrapper for (A, B) {
    type Item = (A::Into, B::Into);
    fn unwrap(self) -> (A::Into, B::Into) {
        let (a, b) = self;
        (*a.as_plotnum(), *b.as_plotnum())
    }
}

impl<A: AsPlotnum, B: AsPlotnum> Unwrapper for &(A, B) {
    type Item = (A::Into, B::Into);
    fn unwrap(self) -> (A::Into, B::Into) {
        let (a, b) = self;
        (*a.as_plotnum(), *b.as_plotnum())
    }
}
