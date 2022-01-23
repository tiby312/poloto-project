//!
//! Funcionality to format the title/xaxis/yaxis and ticks with more information as
//! input.
//!
use super::*;

///
/// Allows to override the default tick formatting using information
/// such as min and max bounds and step information.
///
pub trait PlotterTickFmt<X: PlotNum> {
    fn fmt_self(&mut self, val: X, data: DataSingle<X>, ff: FmtFull) -> std::fmt::Result;
}

pub fn default_tick_fmt<'a, X: PlotNum + 'a>() -> impl PlotterTickFmt<X> + 'a {
    tick_fmt_ext(|mut v: X, mut d, ff| v.val_fmt(d.writer, ff, &mut d.step))
}

pub fn tick_fmt_ext<X: PlotNum>(
    func: impl FnMut(X, DataSingle<X>, FmtFull) -> std::fmt::Result,
) -> impl PlotterTickFmt<X> {
    impl<X: PlotNum, F> PlotterTickFmt<X> for F
    where
        F: FnMut(X, DataSingle<X>, FmtFull) -> std::fmt::Result,
    {
        fn fmt_self(&mut self, val: X, data: DataSingle<X>, ff: FmtFull) -> std::fmt::Result {
            (self)(val, data, ff)
        }
    }

    func
}

pub trait PlotterNameSingleFmt<X: PlotNum> {
    fn fmt_self(&mut self, data: DataSingle<X>) -> std::fmt::Result;
}

impl<T: std::fmt::Display, X: PlotNum> PlotterNameSingleFmt<X> for T {
    fn fmt_self(&mut self, data: DataSingle<X>) -> std::fmt::Result {
        write!(data.writer, "{}", self)
    }
}

pub fn name_single_ext<X: PlotNum, F: FnMut(DataSingle<X>) -> std::fmt::Result>(
    func: F,
) -> impl PlotterNameSingleFmt<X> {
    pub struct NoDisp<F>(pub F);

    impl<X: PlotNum, F> PlotterNameSingleFmt<X> for NoDisp<F>
    where
        F: FnMut(DataSingle<X>) -> std::fmt::Result,
    {
        fn fmt_self(&mut self, data: DataSingle<X>) -> std::fmt::Result {
            (self.0)(data)
        }
    }

    NoDisp(func)
}

///
/// Allows to format either the title,xaxis label, or yaxis label
/// using information such as the min and max bounds or step information.
///
pub trait PlotterNameFmt<X: PlotNum, Y: PlotNum> {
    fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result;
}

impl<T: std::fmt::Display, X: PlotNum, Y: PlotNum> PlotterNameFmt<X, Y> for T {
    fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result {
        write!(data.writer, "{}", self)
    }
}

pub fn name_ext<X: PlotNum, Y: PlotNum, F: FnMut(Data<X, Y>) -> std::fmt::Result>(
    func: F,
) -> impl PlotterNameFmt<X, Y> {
    pub struct NoDisp<F>(pub F);

    impl<X: PlotNum, Y: PlotNum, F> PlotterNameFmt<X, Y> for NoDisp<F>
    where
        F: FnMut(Data<X, Y>) -> std::fmt::Result,
    {
        fn fmt_self(&mut self, data: Data<X, Y>) -> std::fmt::Result {
            (self.0)(data)
        }
    }

    NoDisp(func)
}

pub struct DataSingle<'a, X: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub bound: [X; 2],
    pub step: &'a mut X::StepInfo,
}
pub struct Data<'a, X: PlotNum, Y: PlotNum> {
    pub writer: &'a mut dyn std::fmt::Write,
    pub boundx: [X; 2],
    pub boundy: [Y; 2],
    pub stepx: &'a mut X::StepInfo,
    pub stepy: &'a mut Y::StepInfo,
}
