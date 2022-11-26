use super::*;

use super::marker::Area;

///
/// Represents a single plot.
///
#[derive(Clone)]
pub struct SinglePlot<X, Y, I, D> {
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

impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)>, D: Display> IntoPlotIterator
    for SinglePlot<X, Y, I, D>
{
    type P = Self;
    fn create(self) -> Self {
        self
    }
}

impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)>, D: Display> PlotIterator
    for SinglePlot<X, Y, I, D>
{
    type X = X;
    type Y = Y;
    fn increase_area(&mut self, area: &mut Area<X, Y>) {
        area.grow_area(&self.area);
    }
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

impl<A: PlotIterator, B: PlotIterator<X = A::X, Y = A::Y>> IntoPlotIterator for Chain<A, B> {
    type P = Self;
    fn create(self) -> Self {
        self
    }
}

impl<A: PlotIterator, B: PlotIterator<X = A::X, Y = A::Y>> PlotIterator for Chain<A, B> {
    type X = A::X;
    type Y = A::Y;
    fn increase_area(&mut self, area: &mut Area<A::X, A::Y>) {
        self.a.increase_area(area);
        self.b.increase_area(area);
    }
    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(A::X, A::Y)> {
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

impl<I: IntoIterator<Item = F>, F: PlotIterator> IntoPlotIterator for I {
    type P = PlotsDyn<F>;
    fn create(self) -> Self::P {
        plot_iter_impl::PlotsDyn::new(self.into_iter().collect())
    }
}

impl<F: PlotIterator> IntoPlotIterator for PlotsDyn<F> {
    type P = Self;
    fn create(self) -> Self {
        self
    }
}

impl<F: PlotIterator> PlotIterator for PlotsDyn<F> {
    type X = F::X;
    type Y = F::Y;
    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>) {
        for a in self.flop.iter_mut() {
            a.increase_area(area);
        }
    }
    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if self.counter >= self.flop.len() {
            None
        } else {
            self.flop[self.counter].next_typ()
        }
    }

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)> {
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

impl<XI: Iterator, YI: Iterator> IntoPlotIterator for Marker<XI, YI>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
    type P = Self;
    fn create(self) -> Self {
        self
    }
}

impl<XI: Iterator, YI: Iterator> PlotIterator for Marker<XI, YI>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
    type X = XI::Item;
    type Y = YI::Item;
    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        None
    }
    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)> {
        PlotResult::Finished
    }
    #[inline(always)]
    fn next_name(&mut self, _: &mut dyn fmt::Write) -> Option<fmt::Result> {
        None
    }
    fn increase_area(&mut self, area: &mut Area<XI::Item, YI::Item>) {
        for a in &mut self.x {
            area.grow(Some(&a), None);
        }
        for a in &mut self.y {
            area.grow(None, Some(&a));
        }
    }
}
