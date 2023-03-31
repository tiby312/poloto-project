//!
//! Contains the [`Croppable`] trait which allows replacing values with disconnect values.
//!
use super::*;

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

impl<X: DiscNum + PlotNum, Y: DiscNum + PlotNum, I: FusedIterator> FusedIterator for Crop<X, Y, I> where
    I::Item: Unwrapper<Item = (X, Y)>
{
}
impl<X: DiscNum + PlotNum, Y: DiscNum + PlotNum, I: Iterator> Iterator for Crop<X, Y, I>
where
    I::Item: Unwrapper<Item = (X, Y)>,
{
    type Item = (X, Y);

    #[inline(always)]
    fn next(&mut self) -> Option<(X, Y)> {
        if let Some(g) = self.inner.next() {
            let (x, y) = g.unwrap();
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

    fn crop_around_y(self, val: [Y; 2]) -> Crop<X, Y, Crop<X, Y, Self>>;

    fn crop_around_x(self, val: [X; 2]) -> Crop<X, Y, Crop<X, Y, Self>>;

    fn crop_around_area(self, val: [X; 2], val2: [Y; 2]) -> RectCrop<X, Y, Self>;
}

type RectCrop<X, Y, I> = Crop<X, Y, Crop<X, Y, Crop<X, Y, Crop<X, Y, I>>>>;

impl<X: DiscNum + PlotNum, Y: DiscNum + PlotNum, I: Iterator> Croppable<X, Y> for I
where
    I::Item: Unwrapper<Item = (X, Y)>,
{
    fn crop_around_y(self, val: [Y; 2]) -> Crop<X, Y, Crop<X, Y, Self>> {
        let [a, b] = val;
        self.crop_below(a).crop_above(b)
    }

    fn crop_around_x(self, val: [X; 2]) -> Crop<X, Y, Crop<X, Y, Self>> {
        let [a, b] = val;
        self.crop_left(a).crop_right(b)
    }

    fn crop_around_area(self, val: [X; 2], val2: [Y; 2]) -> RectCrop<X, Y, I> {
        self.crop_around_x(val).crop_around_y(val2)
    }
}
