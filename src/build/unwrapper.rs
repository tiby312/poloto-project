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

pub trait IntoPlotnum: Copy {
    type Into: PlotNum;
    fn into(self) -> Self::Into;
}
impl<P: PlotNum> IntoPlotnum for P {
    type Into = P;
    fn into(self) -> Self::Into {
        self
    }
}

impl<A: IntoPlotnum, B: IntoPlotnum> Unwrapper for (A, B) {
    type Item = (A::Into, B::Into);
    fn unwrap(self) -> (A::Into, B::Into) {
        let (a, b) = self;
        (a.into(), b.into())
    }
}

impl<A: IntoPlotnum, B: IntoPlotnum> Unwrapper for &(A, B) {
    type Item = (A::Into, B::Into);
    fn unwrap(self) -> (A::Into, B::Into) {
        let (a, b) = *self;
        (a.into(), b.into())
    }
}
