//!
//! Funcionality to format the title/xaxis/yaxis and ticks with more information as
//! input.
//!
//!
//!
use super::*;
use std::fmt;

///
/// Write labels with more information about bounds and step sizes.
///
/// Implemented for anything that implements [`Display`].
///
///
pub trait PlotterNameFmt<X: PlotNumContext, Y: PlotNumContext> {
    fn fmt_self(
        &mut self,
        w: &mut dyn fmt::Write,
        x: ([X::Num; 2], &X::StepInfo),
        y: ([Y::Num; 2], &Y::StepInfo),
    ) -> std::fmt::Result;
}

impl<K: Display, X: PlotNumContext, Y: PlotNumContext> PlotterNameFmt<X, Y> for K {
    fn fmt_self(
        &mut self,
        w: &mut dyn fmt::Write,
        _x: ([X::Num; 2], &X::StepInfo),
        _y: ([Y::Num; 2], &Y::StepInfo),
    ) -> std::fmt::Result {
        write!(w, "{}", self)
    }
}

///
/// If you desire to write out the bounds or step sizes as part of a label,
/// this requires that the bounds and tick sizes be computed. Therefore,
/// we pass a closure that takes those as arguments.
///
pub fn name_ext<X: PlotNumContext, Y: PlotNumContext>(
    func: impl FnMut(
        &mut dyn fmt::Write,
        ([X::Num; 2], &X::StepInfo),
        ([Y::Num; 2], &Y::StepInfo),
    ) -> fmt::Result,
) -> impl PlotterNameFmt<X, Y> {
    pub struct Foo<X>(X);
    impl<
            X: PlotNumContext,
            Y: PlotNumContext,
            F: FnMut(
                &mut dyn fmt::Write,
                ([X::Num; 2], &X::StepInfo),
                ([Y::Num; 2], &Y::StepInfo),
            ) -> fmt::Result,
        > PlotterNameFmt<X, Y> for Foo<F>
    {
        fn fmt_self(
            &mut self,
            w: &mut dyn fmt::Write,
            x: ([X::Num; 2], &X::StepInfo),
            y: ([Y::Num; 2], &Y::StepInfo),
        ) -> std::fmt::Result {
            (self.0)(w, x, y)
        }
    }
    Foo(func)
}
