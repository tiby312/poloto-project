//!
//! Contains the [`PlotNum`] trait and their supporting structs.
//!

//pub mod ext;

/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`crate::plottable::crop::Croppable`]
///
pub trait DiscNum: PlotNum {
    /// Create a hole value.
    fn hole() -> Self;
}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy {
    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool {
        false
    }

    fn scale(&self, range: [Self; 2], max: f64) -> f64;

    fn unit_range(offset: Option<Self>) -> [Self; 2];
}

///
/// Used by [`crate::Bound`]
///
#[derive(Debug, Copy, Clone)]
pub struct DashInfo {
    //The ideal dash size in the drawing area
    pub ideal_dash_size: f64,

    //The total drawing area
    pub max: f64,
}

///
/// Information on the properties of all the interval ticks for one dimension.
///
#[derive(Debug, Clone)]
pub struct TickInfo<I> {
    /// Original bound
    pub bound: crate::Bound<I>,

    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    pub ticks: Vec<I>,

    /// The number of dashes between two ticks must be a multiple of this number.
    pub dash_size: Option<f64>,
}

///
/// Trait to allow a plotnum to have a default tick distribution.
///
/// Used by [`crate::DataResultWrapper::plot`]
///
pub trait HasDefaultTicks: PlotNum {
    type Fmt: TickFormat<Num = Self>;
    fn generate(bound: crate::Bound<Self>) -> (TickInfo<Self>, Self::Fmt);
}

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
}

///
/// Formatter for a tick.
///
pub trait TickFormat {
    type Num: PlotNum;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result;
    fn write_where(&mut self, _: &mut dyn std::fmt::Write) -> std::fmt::Result {
        Ok(())
    }

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

///
/// Used by [`TickFormat::with_where_fmt`]
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
/// Used by [`TickFormat::with_tick_fmt`]
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

use std::fmt;
///
/// Used by [`crate::DataResult::plot_with`]
///
pub trait PlotFmt {
    type X: PlotNum;
    type Y: PlotNum;
    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xwher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_ywher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xtick(&mut self, writer: &mut dyn fmt::Write, val: &Self::X) -> fmt::Result;
    fn write_ytick(&mut self, writer: &mut dyn fmt::Write, val: &Self::Y) -> fmt::Result;
}
