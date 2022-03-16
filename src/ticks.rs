//!
//! Tools to create tick distributions.
//!
use super::*;

///
/// Building block to make ticks.
///
/// Created once the min and max bounds of all the plots has been computed.
/// Contains in it all the information typically needed to make a [`TickInfo`].
///
/// Used by [`ticks::from_default`]
///
#[derive(Debug, Clone)]
pub struct Bound<X> {
    pub data: ticks::DataBound<X>,
    pub canvas: CanvasBound,
}

///
/// Tick relevant information of [`Data`]
///
#[derive(Debug, Clone)]
pub struct DataBound<X> {
    pub min: X,
    pub max: X,
}

///
/// Tick relevant information of [`Canvas`]
///
#[derive(Debug, Clone)]
pub struct CanvasBound {
    pub ideal_num_steps: u32,
    pub ideal_dash_size: f64,
    pub max: f64,
    pub axis: Axis,
}

///
/// Create a tick distribution from the default tick generator for the plotnum type.
///
pub fn from_default<X: HasDefaultTicks>(bound: &Bound<X>) -> (TickInfo<X::IntoIter>, X::Fmt) {
    X::generate(bound)
}

///
/// Create a [`plotnum::TickInfo`] from a step iterator.
///
///
pub fn from_iter<X: PlotNum + Display, I: Iterator<Item = X>>(
    ticks: I,
) -> (TickInfo<I>, TickIterFmt<X>) {
    (
        TickInfo {
            ticks,
            dash_size: None,
        },
        TickIterFmt { _p: PhantomData },
    )
}

///
/// Used by [`ticks::from_iter`]
///
pub struct TickIterFmt<T> {
    _p: PhantomData<T>,
}
impl<J: PlotNum + Display> TickFormat for TickIterFmt<J> {
    type Num = J;
    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        write!(writer, "{}", val)
    }
}
