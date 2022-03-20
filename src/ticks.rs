//!
//! Tools to create tick distributions.
//!
use super::*;

///
/// Building block to make ticks.
///
/// Created once the min and max bounds of all the plots has been computed.
/// Contains in it all the information typically needed to make a [`TickFormat`].
///
/// Used by [`ticks::from_default`]
///
#[derive(Debug, Clone)]
pub struct Bound<'a, X> {
    pub data: &'a ticks::DataBound<X>,
    pub canvas: &'a CanvasBound,
}

///
/// Tick relevant information of [`Data`]
///
#[derive(Debug, Clone)]
pub struct DataBound<X> {
    pub min: X,
    pub max: X,
}

///
/// Tick relevant information of [`Canvas`]
///
#[derive(Debug)]
pub struct CanvasBound {
    pub ideal_num_steps: u32,
    pub ideal_dash_size: f64,
    pub max: f64,
    pub axis: Axis,
}

///
/// Create a tick distribution from the default tick generator for the plotnum type.
///
pub fn from_default<X: HasDefaultTicks>(bound: Bound<X>) -> X::Fmt {
    X::generate(bound)
}

///
/// Create a [`TickFormat`] from a step iterator.
///
///
pub fn from_iter<X: PlotNum + Display, I: Iterator<Item = X>>(ticks: I) -> TickIterFmt<I> {
    TickIterFmt { ticks }
}

///
/// Used by [`ticks::from_iter`]
///
pub struct TickIterFmt<I: Iterator> {
    ticks: I,
}
impl<I: Iterator> TickFormat for TickIterFmt<I>
where
    I::Item: PlotNum + Display,
{
    type Num = I::Item;
    fn dash_size(&self) -> Option<f64> {
        None
    }
    fn next_tick(&mut self) -> Option<Self::Num> {
        self.ticks.next()
    }
    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        write!(writer, "{}", val)
    }
}

///
/// Trait to allow a plotnum to have a default tick distribution.
///
/// Used by [`Data::plot`]
///
pub trait HasDefaultTicks: PlotNum {
    type Fmt: TickFormat<Num = Self>;
    fn generate(bound: ticks::Bound<Self>) -> Self::Fmt;
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

pub trait TickFormatExt: TickFormat {
    fn with_tick_fmt<F>(self, func: F) -> TickFmt<Self, F>
    where
        Self: Sized,
        F: Fn(&mut dyn std::fmt::Write, &Self::Num) -> std::fmt::Result,
    {
        TickFmt { inner: self, func }
    }

    fn with_where_fmt<F>(self, func: F) -> WhereFmt<Self, F>
    where
        Self: Sized,
        F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result,
    {
        WhereFmt { inner: self, func }
    }
}

impl<T: TickFormat> TickFormatExt for T {}

///
/// Useful for numbering footnotes. If one axis uses the number one as a footnote,
/// The second access should use the number two as a footnote.
///
pub struct IndexRequester<'a> {
    counter: &'a mut usize,
}
impl<'a> IndexRequester<'a> {
    pub fn new(counter: &'a mut usize) -> Self {
        IndexRequester { counter }
    }
    pub fn request(&mut self) -> usize {
        let val = *self.counter;
        *self.counter += 1;
        val
    }
}
///
/// Formatter for a tick.
///
pub trait TickFormat {
    type Num: PlotNum;
    fn dash_size(&self) -> Option<f64>;
    fn next_tick(&mut self) -> Option<Self::Num>;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result;
    fn write_where(
        &mut self,
        _: &mut dyn std::fmt::Write,
        _req: IndexRequester,
    ) -> std::fmt::Result {
        Ok(())
    }
}

///
/// Used by [`TickFormatExt::with_where_fmt`]
///
pub struct WhereFmt<T, F> {
    inner: T,
    func: F,
}
impl<T: TickFormat, F: Fn(&mut dyn std::fmt::Write) -> std::fmt::Result> TickFormat
    for WhereFmt<T, F>
{
    type Num = T::Num;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        self.inner.write_tick(a, val)
    }
    fn write_where(
        &mut self,
        a: &mut dyn std::fmt::Write,
        _req: IndexRequester,
    ) -> std::fmt::Result {
        (self.func)(a)
    }
    fn dash_size(&self) -> Option<f64> {
        self.inner.dash_size()
    }
    fn next_tick(&mut self) -> Option<Self::Num> {
        self.inner.next_tick()
    }
}

///
/// Used by [`TickFormatExt::with_tick_fmt`]
///
pub struct TickFmt<T, F> {
    inner: T,
    func: F,
}
impl<T: TickFormat, F: Fn(&mut dyn std::fmt::Write, &T::Num) -> std::fmt::Result> TickFormat
    for TickFmt<T, F>
{
    type Num = T::Num;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        (self.func)(a, val)
    }
    fn write_where(
        &mut self,
        a: &mut dyn std::fmt::Write,
        ind: IndexRequester,
    ) -> std::fmt::Result {
        self.inner.write_where(a, ind)
    }
    fn dash_size(&self) -> Option<f64> {
        self.inner.dash_size()
    }
    fn next_tick(&mut self) -> Option<Self::Num> {
        self.inner.next_tick()
    }
}
