//!
//! Create bar charts
//!
use super::*;
use std::convert::TryFrom;
struct BarTickFmt<D> {
    ticks: Vec<D>,
}

impl<D: Display> crate::ticks::tick_fmt::TickFmt<i128> for BarTickFmt<D> {
    fn write_tick(&mut self, writer: &mut dyn std::fmt::Write, val: &i128) -> fmt::Result {
        let j = &self.ticks[usize::try_from(*val).unwrap()];
        write!(writer, "{}", j)
    }
}

pub fn gen_simple<K: Display, D: Display, X: PlotNum + HasDefaultTicks>(
    name: K,
    data: impl IntoIterator<Item = (X, D)>,
    marker: impl IntoIterator<Item = X>,
) -> Stage2<impl IntoPlotIterator<X = X, Y = i128>, impl TickDist<Num = X>, impl TickDist<Num = i128>>
{
    let (plots, ytick_fmt) = gen_bar(name, data, marker);

    let opt = crate::render::render_opt()
        .with_tick_lines([true, false])
        .move_into();

    crate::render::Stage1::from_parts(plots, X::default_ticks(), ytick_fmt, opt).build()
}

pub fn gen_bar<K: Display, D: Display, X: PlotNum>(
    name: K,
    vals: impl IntoIterator<Item = (X, D)>,
    marker: impl IntoIterator<Item = X>,
) -> (
    impl IntoPlotIterator<X = X, Y = i128>,
    impl TickDistGen<i128>,
) {
    let (vals, names): (Vec<_>, Vec<_>) = vals.into_iter().unzip();

    let vals_len = vals.len();

    let bars = build::plot(name).bars(
        vals.into_iter()
            .enumerate()
            .map(|(i, x)| (x, i128::try_from(i).unwrap())),
    );

    let ticks = (0..vals_len)
        .map(|x| i128::try_from(x).unwrap())
        .collect::<Vec<_>>()
        .into_iter();

    let m = build::markers(marker, [-1, i128::try_from(vals_len).unwrap()]);

    (
        bars.chain(m),
        crate::ticks::TickDistribution::new(ticks).with_fmt(BarTickFmt { ticks: names }),
    )
}
