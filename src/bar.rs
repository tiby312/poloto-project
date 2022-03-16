//!
//! Create bar charts
//!
use super::*;
use std::convert::TryFrom;
pub struct BarTickFmt<D> {
    ticks: Vec<D>,
}

impl<'a, D: Display> TickFormat for BarTickFmt<D> {
    type Num = i128;
    fn write_tick(&mut self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> fmt::Result {
        let j = &self.ticks[usize::try_from(*val).unwrap()];
        write!(writer, "{}", j)
    }
}

pub fn gen_bar<K: Display, D: Display, X: PlotNum>(
    name: K,
    vals: impl IntoIterator<Item = (X, D)>,
) -> (
    impl Flop<X = X, Y = i128>,
    TickInfo<Vec<i128>>,
    BarTickFmt<D>,
) {
    let (vals, names): (Vec<_>, Vec<_>) = vals.into_iter().unzip();

    let vals_len = vals.len();

    let bars = crate::build::bars(
        name,
        vals.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i128::try_from(i).unwrap())),
    );

    //.ymarker(-1)
    //.ymarker(i128::try_from(vals_len).unwrap());

    let ticks = (0..vals_len).map(|x| i128::try_from(x).unwrap()).collect();

    (
        bars,
        TickInfo {
            ticks,
            dash_size: None,
        },
        BarTickFmt { ticks: names },
    )
}
