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

pub trait PlotNumContextFromBound: PlotNumContext {
    fn new(a: &crate::Bound<Self::Num>) -> Self;
}
///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNumContext {
    type StepInfo;
    type Num: PlotNum;
    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guaranteed to be called before fmt_tick.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo>;

    /// Format each tick.
    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        _extra: &Self::StepInfo,
    ) -> std::fmt::Result;

    /// Format the where clause
    fn where_fmt(&mut self, writer: &mut dyn std::fmt::Write, val: Self::Num) -> std::fmt::Result;

    /// Called before [`PlotNumContext::compute_ticks`] is called.
    /// If none, [`crate::Plotter`] will pick a number of ticks.
    fn ideal_num_ticks(&self) -> Option<u32> {
        None
    }
}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy {
    type DefaultContext: PlotNumContextFromBound<Num = Self>;

    fn default_ctx(bound: &crate::Bound<Self>) -> Self::DefaultContext {
        Self::DefaultContext::new(bound)
    }

    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool {
        false
    }

    fn default_scale(&self, range: [Self; 2], max: f64) -> f64;

    fn default_unit_range(offset: Option<Self>) -> [Self; 2];
}

///
/// Used by [`PlotNumContext::compute_ticks`]
///
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
/// Used by [`PlotNumContext::compute_ticks`]
///
#[derive(Debug, Clone)]
pub struct TickInfo<I, K> {
    pub unit_data: K,
    /// List of the position of each tick to be displayed.
    /// This must have a length of as least 2.
    pub ticks: Vec<Tick<I>>,

    /// The number of dashes between two ticks must be a multiple of this number.
    //pub dash_multiple: u32,
    pub dash_size: Option<f64>,

    /// If we want to display the tick values relatively, this will
    /// have the base start to start with.
    pub display_relative: Option<I>,
}
