//!
//! Adapters to manipulate a [`PlotNumContext`]
//!
//!
use super::*;
pub struct WithNumTicks<T: PlotNumContext> {
    t: T,
    num: u32,
}
impl<P: PlotNumContext> PlotNumContext for WithNumTicks<P> {
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        self.t.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.t.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.t.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        self.t.fmt_tick(formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        Some(self.num)
    }
}

pub struct WithFmt<T, F> {
    pub t: T,
    pub func: F,
}
impl<
        P: PlotNumContext,
        F: FnMut(&mut dyn std::fmt::Write, P::Num, P::UnitData, FmtFull) -> std::fmt::Result,
    > PlotNumContext for WithFmt<P, F>
{
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        self.t.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.t.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.t.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        mut formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        (self.func)(&mut formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.t.ideal_num_ticks()
    }
}

pub struct NoDash<T>(pub T);

impl<P: PlotNumContext> PlotNumContext for NoDash<P> {
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        let mut t = self.0.compute_ticks(ideal_num_steps, range, dash);
        t.dash_size = None;
        t
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.0.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.0.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        self.0.fmt_tick(formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.0.ideal_num_ticks()
    }
}

pub struct Marker<T: PlotNumContext>(pub T, T::Num);

impl<P: PlotNumContext> PlotNumContext for Marker<P> {
    type UnitData = P::UnitData;
    type Num = P::Num;
    type TickIter = P::TickIter;

    ///
    /// Given an ideal number of intervals across the min and max values,
    /// Calculate information related to where the interval ticks should go.
    ///
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::UnitData, Self::TickIter> {
        self.0.compute_ticks(ideal_num_steps, range, dash)
    }

    /// If there is only one point in a graph, or no point at all,
    /// the range to display in the graph.
    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.0.unit_range(offset)
    }

    /// Provided a min and max range, scale the current value against max.
    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.0.scale(val, range, max)
    }

    /// Used to display a tick
    /// Before overriding this, consider using [`crate::Plotter::xinterval_fmt`] and [`crate::Plotter::yinterval_fmt`].
    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: Self::Num,
        step: Self::UnitData,
        draw_full: FmtFull,
    ) -> std::fmt::Result {
        self.0.fmt_tick(formatter, val, step, draw_full)
    }

    fn ideal_num_ticks(&mut self) -> Option<u32> {
        self.0.ideal_num_ticks()
    }

    fn get_markers(&mut self) -> Vec<Self::Num> {
        //TODO replace when existential types come?
        let mut a = self.0.get_markers();
        a.push(self.1);
        a
    }
}

pub trait PlotNumContextExt: PlotNumContext + Sized {
    fn marker(self, a: Self::Num) -> Marker<Self> {
        Marker(self, a)
    }
    fn no_dash(self) -> NoDash<Self> {
        NoDash(self)
    }

    fn with_fmt<F>(self, func: F) -> WithFmt<Self, F>
    where
        F: FnMut(&mut dyn std::fmt::Write, Self::Num, Self::UnitData, FmtFull) -> std::fmt::Result,
    {
        WithFmt { t: self, func }
    }

    fn with_ideal_num_ticks(self, num: u32) -> WithNumTicks<Self> {
        assert!(num >= 2);
        WithNumTicks { t: self, num }
    }
}
impl<T: PlotNumContext> PlotNumContextExt for T {}
