use super::*;

use super::marker::Area;
use super::marker::Markerable;

///
/// Represents a single plot.
///
#[derive(Clone)]
pub struct SinglePlot<X, Y, I: Iterator<Item = (X, Y)>, D: Display> {
    iter: I,
    area: Area<X, Y>,
    name: D,
    typ: PlotMetaType,
    done: bool,
}
impl<X, Y, I: Iterator<Item = (X, Y)>, D: Display> SinglePlot<X, Y, I, D> {
    #[inline(always)]
    pub(crate) fn new(typ: PlotMetaType, name: D, iter: I, area: Area<X, Y>) -> Self {
        SinglePlot {
            iter,
            area,
            name,
            typ,
            done: false,
        }
    }
}

impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)>, D: Display> Markerable<X, Y>
    for SinglePlot<X, Y, I, D>
{
    fn increase_area(&mut self, area: &mut Area<X, Y>) {
        area.grow_area(&self.area);
    }
}

impl<X, Y, I: Iterator<Item = (X, Y)>, D: Display> PlotIterator<X, Y> for SinglePlot<X, Y, I, D> {
    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
        if let Some(a) = self.iter.next() {
            PlotResult::Some(a)
        } else if !self.done {
            self.done = true;
            PlotResult::None
        } else {
            PlotResult::Finished
        }
    }

    #[inline(always)]
    fn next_name(&mut self, writer: &mut dyn fmt::Write) -> Option<fmt::Result> {
        if !self.done {
            Some(write!(writer, "{}", self.name))
        } else {
            None
        }
    }

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if !self.done {
            Some(self.typ)
        } else {
            None
        }
    }
}

///
/// Chain two plots together.
///
#[derive(Clone)]
pub struct Chain<A, B> {
    a: A,
    b: B,
}
impl<A, B> Chain<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Chain { a, b }
    }
}

impl<X, Y, A: Markerable<X, Y>, B: Markerable<X, Y>> Markerable<X, Y> for Chain<A, B> {
    fn increase_area(&mut self, area: &mut Area<X, Y>) {
        self.a.increase_area(area);
        self.b.increase_area(area);
    }
}

impl<X, Y, A: PlotIterator<X, Y>, B: PlotIterator<X, Y>> PlotIterator<X, Y> for Chain<A, B> {
    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
        match self.a.next_plot_point() {
            PlotResult::Some(a) => PlotResult::Some(a),
            PlotResult::None => PlotResult::None,
            PlotResult::Finished => self.b.next_plot_point(),
        }
    }

    #[inline(always)]
    fn next_name(&mut self, mut writer: &mut dyn fmt::Write) -> Option<fmt::Result> {
        if let Some(a) = self.a.next_name(&mut writer) {
            Some(a)
        } else {
            self.b.next_name(&mut writer)
        }
    }

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if let Some(a) = self.a.next_typ() {
            Some(a)
        } else {
            self.b.next_typ()
        }
    }
}

impl<X, Y, F: Markerable<X, Y>> Markerable<X, Y> for PlotsDyn<F> {
    fn increase_area(&mut self, area: &mut Area<X, Y>) {
        for a in self.flop.iter_mut() {
            a.increase_area(area);
        }
    }
}
///
/// Allows a user to collect plots inside of a loop instead of chaining plots together.
///
#[derive(Clone)]
pub struct PlotsDyn<F> {
    counter: usize,
    flop: Vec<F>,
}
impl<F> PlotsDyn<F> {
    pub fn new(vec: Vec<F>) -> Self {
        PlotsDyn {
            counter: 0,
            flop: vec,
        }
    }
}
impl<X, Y, F: PlotIterator<X, Y>> PlotIterator<X, Y> for PlotsDyn<F> {
    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if self.counter >= self.flop.len() {
            None
        } else {
            self.flop[self.counter].next_typ()
        }
    }

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
        if self.counter >= self.flop.len() {
            return PlotResult::Finished;
        }
        let a = self.flop[self.counter].next_plot_point();
        if let PlotResult::None = a {
            self.counter += 1;
        }
        a
    }

    #[inline(always)]
    fn next_name(&mut self, write: &mut dyn fmt::Write) -> Option<fmt::Result> {
        self.flop[self.counter].next_name(write)
    }
}

#[derive(Clone)]
pub struct Marker<XI, YI> {
    x: XI,
    y: YI,
}

impl<XI: Iterator, YI: Iterator> Marker<XI, YI> {
    pub fn new<XII: IntoIterator<IntoIter = XI>, YII: IntoIterator<IntoIter = YI>>(
        x: XII,
        y: YII,
    ) -> Self {
        Marker {
            x: x.into_iter(),
            y: y.into_iter(),
        }
    }
}
impl<X, Y, XI: Iterator<Item = X>, YI: Iterator<Item = Y>> PlotIterator<X, Y> for Marker<XI, YI> {
    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        None
    }
    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
        PlotResult::Finished
    }
    #[inline(always)]
    fn next_name(&mut self, _: &mut dyn fmt::Write) -> Option<fmt::Result> {
        None
    }
}

impl<XI: Iterator, YI: Iterator> Markerable<XI::Item, YI::Item> for Marker<XI, YI>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
    fn increase_area(&mut self, area: &mut Area<XI::Item, YI::Item>) {
        for a in &mut self.x {
            area.grow(Some(&a), None);
        }
        for a in &mut self.y {
            area.grow(None, Some(&a));
        }
    }
}
