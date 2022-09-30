//!
//! Create bar charts
//!
use super::*;
use std::convert::TryFrom;
struct BarTickFmt<D> {
    ticks: Vec<D>,
}

impl<'a, D: Display> crate::ticks::TickFmt<i128> for BarTickFmt<D> {
    fn write_tick(&mut self, writer: &mut dyn std::fmt::Write, val: &i128) -> fmt::Result {
        let j = &self.ticks[usize::try_from(*val).unwrap()];
        write!(writer, "{}", j)
    }
}

pub fn gen_bar<K: Display, D: Display, X: PlotNum>(
    name: K,
    vals: impl IntoIterator<Item = (X, D)>,
) -> (impl PlotIterator<X, i128>, impl TickFormat<Num = i128>) {
    let (vals, names): (Vec<_>, Vec<_>) = vals.into_iter().unzip();

    let vals_len = vals.len();

    let bars = build::SinglePlotBuilder::new_buffered(
        vals.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i128::try_from(i).unwrap())),
    )
    .bars(name);

    let ticks = (0..vals_len)
        .map(|x| i128::try_from(x).unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let m = build::markers([], [-1, i128::try_from(vals_len).unwrap()]);
    (
        bars.chain(m),
        crate::ticks::from_iter(ticks).with_fmt(BarTickFmt { ticks: names }),
    )
}
