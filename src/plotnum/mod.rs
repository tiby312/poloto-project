//!
//! Contains the [`PlotNum`] trait and their supporting structs.
//!

/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`crate::build::crop::Croppable`]
///
pub trait DiscNum {
    /// Create a hole value.
    fn hole() -> Self;
}

pub trait AsPlotnum {
    type Target: PlotNum;
    fn as_plotnum(&self) -> &Self::Target;
}
impl<P: PlotNum> AsPlotnum for P {
    type Target = P;
    fn as_plotnum(&self) -> &Self::Target {
        self
    }
}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy + std::fmt::Debug {
    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool;

    fn scale(&self, range: [Self; 2], max: f64) -> f64;

    fn unit_range(offset: Option<Self>) -> [Self; 2];
}

pub trait HasDefaultTicks: Sized {
    type DefaultTicks: crate::ticks::GenTickDist<Self>;
    fn default_ticks() -> Self::DefaultTicks;
}

use std::fmt;

///
/// Used by [`crate::render::plot_with`]
///
pub trait BaseFmt {
    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
}

///
/// Signify if a number has a zero value.
///
pub trait HasZero {
    fn zero() -> Self;
}
