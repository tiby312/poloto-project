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
pub struct TickInfo<I: IntoIterator> {
    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    pub ticks: I,

    pub dash_size: Option<f64>,
}

///
/// Trait to allow a plotnum to have a default tick distribution.
///
/// Used by [`crate::DataResult::plot`]
///
pub trait HasDefaultTicks: PlotNum {
    type Fmt: TickFormat<Num = Self>;
    type IntoIter: IntoIterator<Item = Self>;
    fn generate(bound: &crate::Bound<Self>) -> (TickInfo<Self::IntoIter>, Self::Fmt);
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

///
/// Iterator that is accepted by poloto.
/// The second function will only get called after
/// the first iterator has been fully consumed.
///
pub trait PlotIter {
    type Item1;
    type Item2;
    type It1: Iterator<Item = Self::Item1>;
    type It2: Iterator<Item = Self::Item2>;

    /// Return an iterator that will be used to find min max bounds.
    fn first(&mut self) -> Self::It1;

    /// Return an iterator that returns the same data as before in order to scale the plots.
    fn second(self, last: Self::It1) -> Self::It2;
}

impl<I: IntoIterator + Clone> PlotIter for I {
    type Item1 = I::Item;
    type Item2 = I::Item;
    type It1 = I::IntoIter;
    type It2 = I::IntoIter;

    fn first(&mut self) -> Self::It1 {
        self.clone().into_iter()
    }
    fn second(self, _last: Self::It1) -> Self::It2 {
        self.into_iter()
    }
}

pub(super) trait PlotIterExt: PlotIter {
    fn map_plot<B1, B2, F1: FnMut(Self::Item1) -> B1, F2: FnMut(Self::Item2) -> B2>(
        self,
        func1: F1,
        func2: F2,
    ) -> Map<Self, F1, F2>
    where
        Self: Sized,
    {
        Map {
            iter: self,
            func1: Some(func1),
            func2,
        }
    }
}
impl<I: PlotIter> PlotIterExt for I {}

pub(super) struct Map<I, F1, F2> {
    pub iter: I,
    func1: Option<F1>,
    func2: F2,
}

impl<B1, B2, I: PlotIter, F1: FnMut(I::Item1) -> B1, F2: FnMut(I::Item2) -> B2> PlotIter
    for Map<I, F1, F2>
{
    type Item1 = B1;
    type Item2 = B2;
    type It1 = map::Map<I::It1, F1>;
    type It2 = map::Map<I::It2, F2>;

    fn first(&mut self) -> Self::It1 {
        map::Map::new(self.iter.first(), self.func1.take().unwrap())
    }
    fn second(self, last: Self::It1) -> Self::It2 {
        map::Map::new(self.iter.second(last.iter), self.func2)
    }
}

mod map {
    /// Like std::iter::map but you can access the original iterator.

    pub struct Map<I, F> {
        pub iter: I,
        f: F,
    }

    impl<I, F> Map<I, F> {
        pub fn new(iter: I, f: F) -> Map<I, F> {
            Map { iter, f }
        }
    }

    impl<B, I: Iterator, F> Iterator for Map<I, F>
    where
        F: FnMut(I::Item) -> B,
    {
        type Item = B;

        #[inline]
        fn next(&mut self) -> Option<B> {
            self.iter.next().map(&mut self.f)
        }

        #[inline]
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.iter.size_hint()
        }
    }
}
