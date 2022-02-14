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
    ///
    /// Used by [`Bound::default_ticks`](crate::Bound::default_ticks)
    ///
    type DefaultTickGenerator;

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
#[derive(Copy, Clone)]
pub struct DashInfo {
    //The ideal dash size in the drawing area
    pub ideal_dash_size: f64,

    //The total drawing area
    pub max: f64,
}

///
/// One interval tick.
/// Used by [`TickInfo`]
///
#[derive(Debug, Clone, Copy)]
pub struct Tick<I> {
    pub position: I,
    /// If [`TickInfo::display_relative`] is `None`, then this has the same value as [`Tick::position`]
    pub value: I,
}
impl<I> Tick<I> {
    pub fn map<J>(self, func: impl Fn(I) -> J) -> Tick<J> {
        Tick {
            position: func(self.position),
            value: func(self.value),
        }
    }
}

///
/// Information on the properties of all the interval ticks for one dimension.
/// Used by [`TickDist`]
///
#[derive(Debug, Clone)]
pub struct TickInfo<I> {
    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    pub ticks: Vec<Tick<I>>,

    /// The number of dashes between two ticks must be a multiple of this number.
    pub dash_size: Option<f64>,

    /// If we want to display the tick values relatively, this will
    /// have the base start to start with.
    pub display_relative: Option<I>,
}

///
/// Returned by [`TickGenerator::generate`]
///
pub struct TickDist<J: TickFormat> {
    pub ticks: TickInfo<J::Num>,
    pub fmt: J,
}

impl<J: TickFormat> TickDist<J> {
    pub fn with_tick_fmt<F>(self, func: F) -> TickDist<TickFmt<J, F>>
    where
        Self: Sized,
        F: Fn(&mut dyn std::fmt::Write, &J::Num) -> std::fmt::Result,
    {
        TickDist {
            ticks: self.ticks,
            fmt: TickFmt {
                inner: self.fmt,
                func,
            },
        }
    }

    pub fn with_no_dash(mut self) -> Self {
        self.ticks.dash_size = None;
        self
    }
}

///
/// From the given min and max bounds of a distribution of plots,
/// generate out a distribution of ticks to display.
///
pub trait TickGenerator {
    type Num: PlotNum;
    type Fmt: TickFormat<Num = Self::Num>;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guaranteed to be called before fmt_tick.
    ///
    fn generate(self, bound: crate::Bound<Self::Num>) -> TickDist<Self::Fmt>;
}

///
/// Formatter for a tick.
///
pub trait TickFormat {
    type Num;
    fn write_tick(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result;
    fn write_where(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        self.write_tick(a, val)
    }

    fn with_tick_fmt<F>(self, func: F) -> TickFmt<Self, F>
    where
        Self: Sized,
        F: Fn(&mut dyn std::fmt::Write, &Self::Num) -> std::fmt::Result,
    {
        TickFmt { inner: self, func }
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
    fn write_where(&mut self, a: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        self.inner.write_where(a, val)
    }
}
