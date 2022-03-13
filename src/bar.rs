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

fn bars<'a, 'b, X: PlotNum, Y: PlotNum, I>(
    data: &'b mut DataBuilder<'a, X, Y>,
    name: impl Display + 'a,
    plots: I,
) -> &'b mut DataBuilder<'a, X, Y>
where
    I: PlotIter + 'a,
    I::Item1: Plottable<Item = (X, Y)>,
    I::Item2: Plottable<Item = (X, Y)>,
{
    data.plots.push(Box::new(PlotStruct::new(
        plots.map_plot(|x| x.make_plot(), |x| x.make_plot()),
        name,
        PlotMetaType::Plot(PlotType::Bars),
    )));
    data
}

pub fn gen_bar<D: Display, X: PlotNum>(
    data: &mut DataBuilder<X, i128>,
    vals: impl IntoIterator<Item = (X, D)>,
) -> (TickInfo<Vec<i128>>, BarTickFmt<D>) {
    let (vals, names): (Vec<_>, Vec<_>) = vals.into_iter().unzip();

    let vals_len = vals.len();
    bars(
        data,
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
