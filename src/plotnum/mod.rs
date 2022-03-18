//!
//! Contains the [`PlotNum`] trait and their supporting structs.
//!

/// A disconnectable number. A number that can me marked as a hole to signify that there is a disconnect in plots.
/// See [`crate::build::crop::Croppable`]
///
pub trait DiscNum: PlotNum {
    /// Create a hole value.
    fn hole() -> Self;
}

///
/// A plottable number. In order to be able to plot a number, we need information on how
/// to display it as well as the interval ticks.
///
pub trait PlotNum: PartialOrd + Copy + std::fmt::Debug {
    /// Is this a hole value to inject discontinuty?
    fn is_hole(&self) -> bool {
        false
    }

    fn scale(&self, range: [Self; 2], max: f64) -> f64;

    fn unit_range(offset: Option<Self>) -> [Self; 2];
}

use std::fmt;

///
/// Used by [`crate::render::Data::plot_with`]
///
pub trait BaseFmt {
    type X: PlotNum;
    type Y: PlotNum;

    fn write_title(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_yname(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xwher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_ywher(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result;
    fn write_xtick(&mut self, writer: &mut dyn fmt::Write, val: &Self::X) -> fmt::Result;
    fn write_ytick(&mut self, writer: &mut dyn fmt::Write, val: &Self::Y) -> fmt::Result;
}
