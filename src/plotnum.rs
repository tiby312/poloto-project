//!
//! Contains the [`PlotNum`] trait and their supporting structs.
//!
///
/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`crate::plottable::crop::Croppable`]
///
pub trait DiscNum: PlotNum {
    /// Create a hole value.
    fn hole() -> Self;
}

pub struct TickFmt<'a, T: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub val: T,
    pub info: T::StepInfo,
    pub bounds: [T; 2],
}



///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy + std::fmt::Display {
    type StepInfo;

    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool {
        false
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&self, range: [Self; 2], max: f64) -> f64;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guarenteed to be called before fmt_tick.
    ///
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2], dash: DashInfo) -> TickInfo<Self>;

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(offset: Option<Self>) -> [Self; 2];

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        _bound:[Self;2],
        _extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        write!(writer, "{}", self)
    }

    fn where_fmt(
        &mut self,
        writer:&mut dyn std::fmt::Write,
        _bound:[Self;2]
    )->std::fmt::Result{
        write!(writer,"{}",self)
    }
}

pub struct DashInfo {
    //The ideal dash size in the drawing area
    pub ideal_dash_size: f64,

    //The total drawing area
    pub max: f64,
}


///
/// One interval tick
///
#[derive(Debug, Clone, Copy)]
pub struct Tick<I> {
    pub position: I,
    /// If [`TickInfo::display_relative`] is `None`, then this has the same value as [`Tick::position`]
    pub value: I,
}

///
/// Information on the properties of all the interval ticks for one dimension.
///
#[derive(Debug, Clone)]
pub struct TickInfo<I: PlotNum> {
    pub unit_data: I::StepInfo,
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
