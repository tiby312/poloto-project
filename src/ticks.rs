//!
//! Tools to create tick distributions.
//!
use super::*;

///
/// Building block to make ticks.
///
/// Created once the min and max bounds of all the plots has been computed.
/// Contains in it all the information typically needed to make a [`TickInfo`].
///
/// Used by [`ticks::from_default`]
///
#[derive(Debug, Clone)]
pub struct Bound<X> {
    pub data: ticks::DataBound<X>,
    pub canvas: CanvasBound,
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
#[derive(Debug, Clone)]
pub struct CanvasBound {
    pub ideal_num_steps: u32,
    pub ideal_dash_size: f64,
    pub max: f64,
    pub axis: Axis,
}

///
/// Information on the properties of all the interval ticks for one dimension.
///
#[derive(Debug, Clone)]
pub struct TickInfo<I: Iterator> {
    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    pub ticks: I,

    pub dash_size: Option<f64>,
}

impl<I: Iterator> TickGen for TickInfo<I> {
    type Item = I::Item;
    fn dash_size(&self) -> Option<f64> {
        self.dash_size
    }
    fn next_tick(&mut self) -> Option<Self::Item> {
        self.ticks.next()
    }
}

///
/// Create a tick distribution from the default tick generator for the plotnum type.
///
pub fn from_default<X: HasDefaultTicks>(bound: &Bound<X>) -> (X::Gen, X::Fmt) {
    X::generate(bound)
}

///
/// Create a [`TickGen`] and a [`TickFormat`] from a step iterator.
///
///
pub fn from_iter<X: PlotNum + Display, I: Iterator<Item = X>>(ticks: I) -> (I, TickIterFmt<X>) {
    (ticks, TickIterFmt { _p: PhantomData })
}

///
/// Used by [`ticks::from_iter`]
///
pub struct TickIterFmt<T> {
    _p: PhantomData<T>,
}
impl<J: PlotNum + Display> TickFormat for TickIterFmt<J> {
    type Num = J;
    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        write!(writer, "{}", val)
    }
}

///
/// Trait that draws the physical ticks instead of writing the text for each one ([`TickFormat`])
///
pub trait TickGen {
    type Item;
    fn dash_size(&self) -> Option<f64>;
    fn next_tick(&mut self) -> Option<Self::Item>;
}

impl<I: Iterator> TickGen for I {
    type Item = I::Item;
    fn dash_size(&self) -> Option<f64> {
        None
    }
    fn next_tick(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

///
/// Trait to allow a plotnum to have a default tick distribution.
///
/// Used by [`Stager::plot`]
///
pub trait HasDefaultTicks: PlotNum {
    type Fmt: TickFormat<Num = Self>;
    type Gen: TickGen<Item = Self>;
    fn generate(bound: &ticks::Bound<Self>) -> (Self::Gen, Self::Fmt);
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
/// Formatter for a tick.
///
pub trait TickFormat {
    type Num: PlotNum;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result;
    fn write_where(&mut self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
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
    fn write_where(&mut self, a: &mut dyn std::fmt::Write) -> std::fmt::Result {
        (self.func)(a)
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
    fn write_where(&mut self, a: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.inner.write_where(a)
    }
}
