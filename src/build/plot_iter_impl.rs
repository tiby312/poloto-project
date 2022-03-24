use super::*;

use super::marker::Area;
use super::marker::Markerable;
///
/// Create a [`PlotsDyn`]
///
pub fn plots_dyn<F: PlotIterator>(vec: Vec<F>) -> PlotsDyn<F> {
    PlotsDyn {
        flop: vec,
        counter: 0,
    }
}

enum SinglePlotInner<I: PlotIter> {
    Ready(I),
    Second(I::It2),
    Done,
}
impl<I: PlotIter> SinglePlotInner<I> {
    fn is_done(&self) -> bool {
        matches!(self, SinglePlotInner::Done)
    }
    fn take(&mut self) -> SinglePlotInner<I> {
        let mut k = SinglePlotInner::Done;
        std::mem::swap(&mut k, self);
        k
    }
}

///
/// Represents a single plot.
///
pub struct SinglePlot<I: PlotIter, D: Display> {
    inner: SinglePlotInner<I>,
    name: D,
    typ: PlotMetaType,
}
impl<I: PlotIter, D: Display> SinglePlot<I, D>
where
    I::Item1: Unwrapper,
    I::Item2: Unwrapper,
{
    #[inline(always)]
    pub(crate) fn new(typ: PlotMetaType, name: D, plots: I) -> Self {
        SinglePlot {
            inner: SinglePlotInner::Ready(plots),
            name,
            typ,
        }
    }
}

impl<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display> Markerable for SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    type X = X;
    type Y = Y;
    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>) {
        if let SinglePlotInner::Ready(mut a) = self.inner.take() {
            let mut i = a.first();
            for k in &mut i {
                let (a, b) = k.unwrap();
                area.grow(Some(a), Some(b));
            }
            let s = a.second(i);
            self.inner = SinglePlotInner::Second(s);
        } else {
            unreachable!();
        }
    }
}

impl<X, I: PlotIter, D: Display> PlotIterator for SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = X>,
    I::Item2: Unwrapper<Item = X>,
{
    type Item = X;

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        match &mut self.inner {
            SinglePlotInner::Second(a) => {
                if let Some(k) = a.next() {
                    PlotResult::Some(k.unwrap())
                } else if !self.inner.is_done() {
                    self.inner = SinglePlotInner::Done;
                    PlotResult::None
                } else {
                    unreachable!();
                }
            }
            SinglePlotInner::Done => PlotResult::Finished,
            _ => {
                unreachable!()
            }
        }
    }

    #[inline(always)]
    fn next_name(&mut self, writer: &mut dyn fmt::Write) -> Option<fmt::Result> {
        if matches!(&self.inner, SinglePlotInner::Second(..)) {
            Some(write!(writer, "{}", self.name))
        } else {
            None
        }
    }

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if matches!(&self.inner, SinglePlotInner::Second(..)) {
            Some(self.typ)
        } else {
            None
        }
    }
}

///
/// Chain two plots together.
///
pub struct Chain<A, B> {
    a: A,
    b: B,
}
impl<A: PlotIterator, B: PlotIterator<Item = A::Item>> Chain<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Chain { a, b }
    }
}

impl<A: Markerable, B: Markerable<X = A::X, Y = A::Y>> Markerable for Chain<A, B> {
    type X = A::X;
    type Y = A::Y;
    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>) {
        self.a.increase_area(area);
        self.b.increase_area(area);
    }
}

impl<A: PlotIterator, B: PlotIterator<Item = A::Item>> PlotIterator for Chain<A, B> {
    type Item = A::Item;

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
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

impl<F: Markerable> Markerable for PlotsDyn<F> {
    type X = F::X;
    type Y = F::Y;
    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>) {
        for a in self.flop.iter_mut() {
            a.increase_area(area);
        }
    }
}
///
/// Allows a user to collect plots inside of a loop instead of chaining plots together.
///
pub struct PlotsDyn<F> {
    counter: usize,
    flop: Vec<F>,
}

impl<F: PlotIterator> PlotIterator for PlotsDyn<F> {
    type Item = F::Item;

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if self.counter >= self.flop.len() {
            None
        } else {
            self.flop[self.counter].next_typ()
        }
    }

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
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
