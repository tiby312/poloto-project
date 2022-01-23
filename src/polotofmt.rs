//!
//! Funcionality to format the title/xaxis/yaxis and ticks with more information as
//! input.
//!
use super::*;
use std::fmt;

///
/// Allows to override the default tick formatting using information
/// such as min and max bounds and step information.
///
pub trait PlotterTickFmt<X: PlotNum> {
    fn fmt_self(
        &mut self,
        w: &mut dyn fmt::Write,
        val: X,
        data: DataSingle<X>,
        ff: FmtFull,
    ) -> std::fmt::Result;
}

pub fn default_tick_fmt<'a, X: PlotNum + 'a>() -> impl PlotterTickFmt<X> + 'a {
    tick_fmt_ext(|w, mut v: X, mut d, ff| v.val_fmt(w, ff, &mut d.step))
}

pub fn tick_fmt_ext<X: PlotNum>(
    func: impl FnMut(&mut dyn fmt::Write, X, DataSingle<X>, FmtFull) -> std::fmt::Result,
) -> impl PlotterTickFmt<X> {
    impl<X: PlotNum, F> PlotterTickFmt<X> for F
    where
        F: FnMut(&mut dyn fmt::Write, X, DataSingle<X>, FmtFull) -> std::fmt::Result,
    {
        fn fmt_self(
            &mut self,
            w: &mut dyn fmt::Write,
            val: X,
            data: DataSingle<X>,
            ff: FmtFull,
        ) -> std::fmt::Result {
            (self)(w, val, data, ff)
        }
    }

    func
}

pub trait PlotterNameSingleFmt<X: PlotNum> {
    fn fmt_self(&mut self, w: &mut dyn fmt::Write, data: DataSingle<X>) -> std::fmt::Result;
}

impl<T: std::fmt::Display, X: PlotNum> PlotterNameSingleFmt<X> for T {
    fn fmt_self(&mut self, w: &mut dyn fmt::Write, _data: DataSingle<X>) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}

pub fn name_single_ext<
    X: PlotNum,
    F: FnMut(&mut dyn fmt::Write, DataSingle<X>) -> std::fmt::Result,
>(
    func: F,
) -> impl PlotterNameSingleFmt<X> {
    pub struct NoDisp<F>(pub F);

    impl<X: PlotNum, F> PlotterNameSingleFmt<X> for NoDisp<F>
    where
        F: FnMut(&mut dyn fmt::Write, DataSingle<X>) -> std::fmt::Result,
    {
        fn fmt_self(&mut self, w: &mut dyn fmt::Write, data: DataSingle<X>) -> std::fmt::Result {
            (self.0)(w, data)
        }
    }

    NoDisp(func)
}

///
/// Allows to format either the title,xaxis label, or yaxis label
/// using information such as the min and max bounds or step information.
///
pub trait PlotterNameFmt<X: PlotNum, Y: PlotNum> {
    fn fmt_self(
        &mut self,
        w: &mut dyn fmt::Write,
        x: DataSingle<X>,
        y: DataSingle<Y>,
    ) -> std::fmt::Result;
}

impl<T: std::fmt::Display, X: PlotNum, Y: PlotNum> PlotterNameFmt<X, Y> for T {
    fn fmt_self(
        &mut self,
        w: &mut dyn fmt::Write,
        _x: DataSingle<X>,
        _y: DataSingle<Y>,
    ) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}

pub fn name_ext<
    X: PlotNum,
    Y: PlotNum,
    F: FnMut(&mut dyn fmt::Write, DataSingle<X>, DataSingle<Y>) -> std::fmt::Result,
>(
    func: F,
) -> impl PlotterNameFmt<X, Y> {
    pub struct NoDisp<F>(pub F);

    impl<X: PlotNum, Y: PlotNum, F> PlotterNameFmt<X, Y> for NoDisp<F>
    where
        F: FnMut(&mut dyn fmt::Write, DataSingle<X>, DataSingle<Y>) -> std::fmt::Result,
    {
        fn fmt_self(
            &mut self,
            w: &mut dyn fmt::Write,
            x: DataSingle<X>,
            y: DataSingle<Y>,
        ) -> std::fmt::Result {
            (self.0)(w, x, y)
        }
    }

    NoDisp(func)
}

pub struct DataSingle<'a, X: PlotNum> {
    pub bound: [X; 2],
    pub step: &'a mut X::StepInfo,
}
