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

pub trait HasDefaultContext {
    type DefaultContext: PlotNumContext + Default;
}

pub struct WhereFmt<X, F> {
    ctx: X,
    func: F,
}

impl<X: PlotNumContext, F: FnMut(X::Num, &mut dyn fmt::Write, [X::Num; 2]) -> fmt::Result>
    PlotNumContext for WhereFmt<X, F>
{
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guarenteed to be called before fmt_tick.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        self.ctx.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn std::fmt::Write,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        self.ctx.tick_fmt(val, writer, bound, extra)
    }

    fn where_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn std::fmt::Write,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        (self.func)(val, writer, bound)
    }
}
pub struct TickFmt<X, F> {
    ctx: X,
    func: F,
}

impl<
        X: PlotNumContext,
        F: FnMut(X::Num, &mut dyn fmt::Write, [X::Num; 2], &mut X::StepInfo) -> fmt::Result,
    > PlotNumContext for TickFmt<X, F>
{
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guarenteed to be called before fmt_tick.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        self.ctx.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn std::fmt::Write,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        (self.func)(val, writer, bound, extra)
    }

    fn where_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn std::fmt::Write,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        self.ctx.where_fmt(val, writer, bound)
    }
}

use std::fmt;

pub trait PlotNumContextExt: PlotNumContext + Sized {
    fn with_tick_fmt<
        F: FnMut(Self::Num, &mut dyn fmt::Write, [Self::Num; 2], &mut Self::StepInfo) -> fmt::Result,
    >(
        self,
        func: F,
    ) -> TickFmt<Self, F> {
        TickFmt { ctx: self, func }
    }

    fn with_where_fmt<F: FnMut(Self::Num, &mut dyn fmt::Write, [Self::Num; 2]) -> fmt::Result>(
        self,
        func: F,
    ) -> WhereFmt<Self, F> {
        WhereFmt { ctx: self, func }
    }
}

impl<T: PlotNumContext> PlotNumContextExt for T {}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNumContext {
    type StepInfo;
    type Num: PlotNum;

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    /// Guarenteed to be called before fmt_tick.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo>;

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2];

    fn tick_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn std::fmt::Write,
        _bound: [Self::Num; 2],
        _extra: &mut Self::StepInfo,
    ) -> std::fmt::Result;

    fn where_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn std::fmt::Write,
        _bound: [Self::Num; 2],
    ) -> std::fmt::Result;
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
///
/// TODO split into two type parameters
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
/*
impl<I:PlotNum> TickInfo<I>{
    pub fn map<J:PlotNum<StepInfo=I::StepInfo>>(self,func:impl Fn(I)->J+Copy)->TickInfo<J> {
        TickInfo { unit_data:self.unit_data, ticks: self.ticks.into_iter().map(move |x|x.map(func)).collect(), dash_size: self.dash_size, display_relative: self.display_relative.map(|a|func(a)) }
    }
}
*/
