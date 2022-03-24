//!
//! Create bar charts
//!
use super::*;
use std::convert::TryFrom;
struct BarTickFmt<D> {
    ticks: Vec<D>,
    steps: std::vec::IntoIter<i128>,
}

impl<'a, D: Display> TickFormat for BarTickFmt<D> {
    type Num = i128;
    fn write_tick(&mut self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> fmt::Result {
        let j = &self.ticks[usize::try_from(*val).unwrap()];
        write!(writer, "{}", j)
    }
    fn dash_size(&self) -> Option<f64> {
        None
    }
    fn next_tick(&mut self) -> Option<Self::Num> {
        self.steps.next()
    }
}

use crate::build::marker::MarkerableExt;
use crate::build::PlotIteratorAndMarkers;
pub fn gen_bar<K: Display, D: Display, X: PlotNum>(
    name: K,
    vals: impl IntoIterator<Item = (X, D)>,
) -> (
    impl PlotIteratorAndMarkers<X = X, Y = i128>,
    impl TickFormat<Num = i128>,
) {
    let (vals, names): (Vec<_>, Vec<_>) = vals.into_iter().unzip();

    let vals_len = vals.len();

    let bars = crate::build::bars(
        name,
        vals.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i128::try_from(i).unwrap())),
    );

    let ticks = (0..vals_len)
        .map(|x| i128::try_from(x).unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    (
        bars.markers([], [-1, i128::try_from(vals_len).unwrap()]),
        BarTickFmt {
            steps: ticks,
            ticks: names,
        },
    )
}
