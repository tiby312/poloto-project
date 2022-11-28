use std::iter::Flatten;

use super::*;

use super::marker::Area;

#[derive(Clone)]
pub struct MapPlotResIter<I>(I);

impl<I: Iterator<Item = PlotRes<F, L>>, F: FusedIterator<Item = PlotTag<L>>, L: Point> FusedIterator
    for MapPlotResIter<I>
{
}
impl<I: Iterator<Item = PlotRes<F, L>>, F: ExactSizeIterator<Item = PlotTag<L>>, L: Point>
    ExactSizeIterator for MapPlotResIter<I>
{
}

impl<I: Iterator<Item = PlotRes<F, L>>, F: Iterator<Item = PlotTag<L>>, L: Point> Iterator
    for MapPlotResIter<I>
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| x.it)
    }
}

impl<F: Iterator<Item = PlotTag<L>>, L: Point> IntoPlotIterator for Vec<PlotRes<F, L>> {
    type L = L;
    type P = Flatten<MapPlotResIter<std::vec::IntoIter<PlotRes<F, L>>>>;
    fn into_plot(self) -> PlotRes<Self::P, L> {
        let mut area = Area::new();
        for a in self.iter() {
            area.grow_area(&a.area);
        }

        let it = MapPlotResIter(self.into_iter()).flatten();

        PlotRes { area, it }
    }
}

impl<const K: usize, F: Iterator<Item = PlotTag<L>>, L: Point> IntoPlotIterator
    for [PlotRes<F, L>; K]
{
    type L = L;
    type P = Flatten<MapPlotResIter<std::array::IntoIter<PlotRes<F, L>, K>>>;
    fn into_plot(self) -> PlotRes<Self::P, L> {
        let mut area = Area::new();
        for a in self.iter() {
            area.grow_area(&a.area);
        }

        let it = MapPlotResIter(self.into_iter()).flatten();

        PlotRes { area, it }
    }
}
