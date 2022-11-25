//!
//! Shortand replacing `x.map(|x|[x,func(x)])` with `x.zip_output(|x|func(x))`
//!

use std::iter::FusedIterator;

#[derive(Copy, Clone)]
pub struct OutputZipper<I, F> {
    inner: I,
    func: F,
}

impl<Y, I: ExactSizeIterator, F: Fn(I::Item) -> Y> ExactSizeIterator for OutputZipper<I, F> where
    I::Item: Clone
{
}
impl<Y, I: FusedIterator, F: Fn(I::Item) -> Y> FusedIterator for OutputZipper<I, F> where
    I::Item: Clone
{
}
impl<Y, I: DoubleEndedIterator, F: Fn(I::Item) -> Y> DoubleEndedIterator for OutputZipper<I, F>
where
    I::Item: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.inner.next_back() {
            let s2 = s.clone();
            Some((s2, (self.func)(s)))
        } else {
            None
        }
    }
}

impl<Y, I: Iterator, F: Fn(I::Item) -> Y> Iterator for OutputZipper<I, F>
where
    I::Item: Clone,
{
    type Item = (I::Item, Y);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.inner.next() {
            let s2 = s.clone();
            Some((s2, (self.func)(s)))
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

pub trait OutputZip: Iterator
where
    Self::Item: Clone,
{
    ///
    /// Provides shorthand for plotting the output of some functions.
    /// maps x and y.
    ///
    ///
    /// Regular:
    ///
    /// ```rust
    /// let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);
    /// let _ = poloto::build::plot("a").scatter(x.map(|x|[x,x.cos()]);
    /// ```
    ///
    /// Shortened:
    ///
    /// ```rust
    /// use poloto::prelude::OutputZip;
    /// let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);
    /// let _ = poloto::build::plot("a").scatter(x.zip_output(|x|x.cos());
    /// ```
    fn zip_output<Y, F: Fn(Self::Item) -> Y>(self, func: F) -> OutputZipper<Self, F>
    where
        Self: Sized,
    {
        OutputZipper { inner: self, func }
    }
}
impl<I: Iterator> OutputZip for I where Self::Item: Clone {}
