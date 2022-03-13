

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

pub fn gen_bar<D: Display, X: PlotNum>(
    data: &mut Data<X, i128>,
    vals: impl IntoIterator<Item = (X, D)>,
) -> (TickInfo<Vec<i128>>, BarTickFmt<D>) {
    let (vals, names): (Vec<_>, Vec<_>) = vals.into_iter().unzip();

    let vals_len = vals.len();
    data.bars(
        "",
        vals.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i128::try_from(i).unwrap())),
    )
    .ymarker(-1)
    .ymarker(i128::try_from(vals_len).unwrap());

    let ticks = (0..vals_len).map(|x| i128::try_from(x).unwrap()).collect();

    (
        TickInfo {
            ticks,
            dash_size: None,
        },
        BarTickFmt { ticks: names },
    )
}
