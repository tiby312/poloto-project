
//!
//! Contains the [`PlotNumContextExt`] trait that provides adaptor functions modifying a [`PlotNumContext`].
//!
use super::*;

///
/// Used by [`PlotNumContextExt::with_where_fmt()`]
///
pub struct WhereFmt<X, F> {
    ctx: X,
    func: F,
}

impl<X: PlotNumContext, F: FnMut(&mut dyn fmt::Write, X::Num, [X::Num; 2]) -> fmt::Result>
    PlotNumContext for WhereFmt<X, F>
{
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        self.ctx.compute_ticks(ideal_num_steps, range, dash)
    }

    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        self.ctx.tick_fmt(writer, val, bound, extra)
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        (self.func)(writer, val, bound)
    }

    fn markers(&self) -> Vec<Self::Num> {
        self.ctx.markers()
    }

    fn ideal_num_ticks(&self) -> Option<u32> {
        self.ctx.ideal_num_ticks()
    }
}

///
/// Used by [`PlotNumContextExt::with_tick_fmt()`]
///
pub struct TickFmt<X, F> {
    ctx: X,
    func: F,
}

impl<
        X: PlotNumContext,
        F: FnMut(&mut dyn fmt::Write, X::Num, [X::Num; 2], &mut X::StepInfo) -> fmt::Result,
    > PlotNumContext for TickFmt<X, F>
{
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        self.ctx.compute_ticks(ideal_num_steps, range, dash)
    }

    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        (self.func)(writer, val, bound, extra)
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        self.ctx.where_fmt(writer, val, bound)
    }

    fn markers(&self) -> Vec<Self::Num> {
        self.ctx.markers()
    }

    fn ideal_num_ticks(&self) -> Option<u32> {
        self.ctx.ideal_num_ticks()
    }
}

///
/// Used by [`PlotNumContextExt::with_no_dash()`]
///
pub struct NoDash<X> {
    ctx: X,
}

impl<X: PlotNumContext> PlotNumContext for NoDash<X> {
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        let mut d = self.ctx.compute_ticks(ideal_num_steps, range, dash);
        d.dash_size = None;
        d
    }

    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        self.ctx.tick_fmt(writer, val, bound, extra)
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        self.ctx.where_fmt(writer, val, bound)
    }

    fn markers(&self) -> Vec<Self::Num> {
        self.ctx.markers()
    }

    fn ideal_num_ticks(&self) -> Option<u32> {
        self.ctx.ideal_num_ticks()
    }
}

///
/// Used by [`PlotNumContextExt::with_marker()`]
///
pub struct WithMarker<X: PlotNumContext> {
    ctx: X,
    marker: X::Num,
}

impl<X: PlotNumContext> PlotNumContext for WithMarker<X> {
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        self.ctx.compute_ticks(ideal_num_steps, range, dash)
    }

    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        self.ctx.tick_fmt(writer, val, bound, extra)
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        self.ctx.where_fmt(writer, val, bound)
    }

    fn markers(&self) -> Vec<Self::Num> {
        let mut a = self.ctx.markers();
        a.push(self.marker);
        a
    }

    fn ideal_num_ticks(&self) -> Option<u32> {
        self.ctx.ideal_num_ticks()
    }
}

///
/// Used by [`PlotNumContextExt::with_ideal_num_ticks()`]
///
pub struct WithNumTick<X: PlotNumContext> {
    ctx: X,
    num_ticks: u32,
}

impl<X: PlotNumContext> PlotNumContext for WithNumTick<X> {
    type StepInfo = X::StepInfo;
    type Num = X::Num;

    fn scale(&mut self, val: Self::Num, range: [Self::Num; 2], max: f64) -> f64 {
        self.ctx.scale(val, range, max)
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [Self::Num; 2],
        dash: DashInfo,
    ) -> TickInfo<Self::Num, Self::StepInfo> {
        self.ctx.compute_ticks(ideal_num_steps, range, dash)
    }

    fn unit_range(&mut self, offset: Option<Self::Num>) -> [Self::Num; 2] {
        self.ctx.unit_range(offset)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
        extra: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        self.ctx.tick_fmt(writer, val, bound, extra)
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: Self::Num,
        bound: [Self::Num; 2],
    ) -> std::fmt::Result {
        self.ctx.where_fmt(writer, val, bound)
    }

    fn markers(&self) -> Vec<Self::Num> {
        self.ctx.markers()
    }

    fn ideal_num_ticks(&self) -> Option<u32> {
        Some(self.num_ticks)
    }
}

use std::fmt;

pub trait PlotNumContextExt: PlotNumContext + Sized {
    fn with_tick_fmt<
        F: FnMut(&mut dyn fmt::Write, Self::Num, [Self::Num; 2], &mut Self::StepInfo) -> fmt::Result,
    >(
        self,
        func: F,
    ) -> TickFmt<Self, F> {
        TickFmt { ctx: self, func }
    }

    fn with_where_fmt<F: FnMut(&mut dyn fmt::Write, Self::Num, [Self::Num; 2]) -> fmt::Result>(
        self,
        func: F,
    ) -> WhereFmt<Self, F> {
        WhereFmt { ctx: self, func }
    }

    fn with_no_dash(self) -> NoDash<Self> {
        NoDash { ctx: self }
    }

    fn with_marker(self, marker: Self::Num) -> WithMarker<Self> {
        WithMarker { ctx: self, marker }
    }

    fn with_ideal_num_ticks(self, num_ticks: u32) -> WithNumTick<Self> {
        WithNumTick {
            ctx: self,
            num_ticks,
        }
    }
}

impl<T: PlotNumContext> PlotNumContextExt for T {}
