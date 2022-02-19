//!
//! Contains the [`Plottable`] trait and adapters that work on it.
//!

use super::*;

/// Iterators that are passed to the [`Plotter`] plot functions must produce
/// items that implement this trait.
pub trait Plottable {
    type Item;
    /// Produce one plot
    fn make_plot(self) -> Self::Item;
}

pub trait FromOrig {
    type X;
    type Y;
    fn from_orig(a: Self::X, b: Self::Y) -> Self;
}

impl<T> FromOrig for [T; 2] {
    type X = T;
    type Y = T;
    fn from_orig(a: Self::X, b: Self::Y) -> Self {
        [a, b]
    }
}

impl<T: PlotNum> Plottable for [T; 2] {
    type Item = (T, T);
    fn make_plot(self) -> (T, T) {
        let [x, y] = self;
        (x, y)
    }
}

impl<T: PlotNum> Plottable for &[T; 2] {
    type Item = (T, T);
    fn make_plot(self) -> (T, T) {
        let [x, y] = *self;
        (x, y)
    }
}

impl<A, B> FromOrig for (A, B) {
    type X = A;
    type Y = B;
    fn from_orig(a: Self::X, b: Self::Y) -> Self {
        (a, b)
    }
}

impl<A: PlotNum, B: PlotNum> Plottable for (A, B) {
    type Item = (A, B);
    fn make_plot(self) -> (A, B) {
        self
    }
}

impl<A: PlotNum, B: PlotNum> Plottable for &(A, B) {
    type Item = (A, B);
    fn make_plot(self) -> (A, B) {
        *self
    }
}

pub mod crop {
    //!
    //! Contains the [`Croppable`] trait which allows replacing values with disconnect values.
    //!
    use crate::Plottable;

    use crate::DiscNum;

    #[derive(Copy, Clone)]
    enum Dir {
        Above,
        Below,
        Left,
        Right,
    }

    ///
    /// Represents one cropping.
    ///
    #[derive(Copy, Clone)]
    pub struct Crop<X, Y, I> {
        dir: Dir,
        val: (X, Y),
        inner: I,
    }
    impl<X: DiscNum, Y: DiscNum, I: Iterator> Iterator for Crop<X, Y, I>
    where
        I::Item: Plottable<Item = (X, Y)>,
    {
        type Item = (X, Y);
        fn next(&mut self) -> Option<(X, Y)> {
            if let Some(g) = self.inner.next() {
                let (x, y) = g.make_plot();
                Some(match self.dir {
                    Dir::Above => {
                        if y > self.val.1 {
                            (x, Y::hole())
                        } else {
                            (x, y)
                        }
                    }
                    Dir::Below => {
                        if y < self.val.1 {
                            (x, Y::hole())
                        } else {
                            (x, y)
                        }
                    }
                    Dir::Left => {
                        if x < self.val.0 {
                            (X::hole(), y)
                        } else {
                            (x, y)
                        }
                    }
                    Dir::Right => {
                        if x > self.val.0 {
                            (X::hole(), y)
                        } else {
                            (x, y)
                        }
                    }
                })
            } else {
                None
            }
        }
    }

    ///
    ///
    /// Using `Iterator::filter` to filter out plots can have
    /// undesirable effects when used with `Plotter::line`,
    /// since the line will assume continuity between each plot
    /// after the filtering has taken place.
    ///
    /// As an alternative, you can replace undesired plots with
    /// NaN values to indicate discontinuity.
    ///
    /// As a convenience, you can use this Trait that will
    /// automatically replace plots past certain bounds with NaN.
    ///
    ///
    pub trait Croppable<X: DiscNum, Y: DiscNum>: Sized {
        fn crop_above(self, val: Y) -> Crop<X, Y, Self> {
            Crop {
                dir: Dir::Above,
                val: (X::hole(), val),
                inner: self,
            }
        }
        fn crop_below(self, val: Y) -> Crop<X, Y, Self> {
            Crop {
                dir: Dir::Below,
                val: (X::hole(), val),
                inner: self,
            }
        }
        fn crop_left(self, val: X) -> Crop<X, Y, Self> {
            Crop {
                dir: Dir::Left,
                val: (val, Y::hole()),
                inner: self,
            }
        }
        fn crop_right(self, val: X) -> Crop<X, Y, Self> {
            Crop {
                dir: Dir::Right,
                val: (val, Y::hole()),
                inner: self,
            }
        }
    }

    impl<X: DiscNum, Y: DiscNum, T: Iterator> Croppable<X, Y> for T where
        T::Item: Plottable<Item = (X, Y)>
    {
    }
}
