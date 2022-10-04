//!
//! Create bar charts
//!
use super::*;
use std::convert::TryFrom;
struct BarTickFmt<D> {
    ticks: Vec<D>,
}

impl<D: Display> crate::ticks::TickFmt<i128> for BarTickFmt<D> {
    fn write_tick(&mut self, writer: &mut dyn std::fmt::Write, val: &i128) -> fmt::Result {
        let j = &self.ticks[usize::try_from(*val).unwrap()];
        write!(writer, "{}", j)
    }
}

pub fn gen_simple<K: Display, D: Display, X: PlotNum + HasDefaultTicks>(
    name: K,
    data: impl IntoIterator<Item = (X, D)>,
    marker: impl IntoIterator<Item = X>,
) -> DataBuilt<impl PlotIterator<X = X, Y = i128>, impl TickDist<Num = X>, impl TickDist<Num = i128>>
{
    let (plots, ytick_fmt) = gen_bar(name, data, marker);

    let opt = crate::render::render_opt_builder()
        .with_tick_lines([true, false])
        .move_into();

    //TODO somehow forbid user from messing with these settings after its returned?
    crate::render::Data::new(plots, default_ticks::<X>(), ytick_fmt, opt).build()
}

pub fn gen_bar<K: Display, D: Display, X: PlotNum>(
    name: K,
    vals: impl IntoIterator<Item = (X, D)>,
    marker: impl IntoIterator<Item = X>,
) -> (impl PlotIterator<X = X, Y = i128>, impl GenTickDist<i128>) {
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

    let m = build::markers(marker, [-1, i128::try_from(vals_len).unwrap()]);

    (
        bars.chain(m),
        crate::ticks::TickDistRes::new(ticks).with_fmt(BarTickFmt { ticks: names }),
    )
}
