//!
//! Tools for assembling plots
//!
//!
use super::*;

pub mod bar;
pub mod bounded_iter;
pub mod buffered_iter;
pub mod crop;
pub mod unwrapper;

pub mod marker;
pub use marker::markers;

pub mod plot_iter_impl;
pub use plot_iter_impl::plots_dyn;
use plot_iter_impl::{Chain, SinglePlot};

use unwrapper::Unwrapper;

///
/// Determine how to interpret the plot's point data when rendering.
///
#[derive(Copy, Clone, Debug)]
pub enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
    LineFillRaw,
    Bars,
}

///
/// Determine if this is a plot or just text.
///
#[derive(Copy, Clone, Debug)]
pub enum PlotMetaType {
    Plot(PlotType),
    Text,
}

///
/// Iterator that is accepted by plot functions like `line`,`scatter`, etc.
/// The second function will only get called after
/// the first iterator has been fully consumed.
///
pub trait PlotIter {
    type Item1;
    type Item2;
    type It1: Iterator<Item = Self::Item1>;
    type It2: Iterator<Item = Self::Item2>;

    /// Return an iterator that will be used to find min max bounds.
    fn first(&mut self) -> Self::It1;

    /// Return an iterator that returns the same data as before in order to scale the plots.
    fn second(self, last: Self::It1) -> Self::It2;
}

impl<I: IntoIterator + Clone> PlotIter for I {
    type Item1 = I::Item;
    type Item2 = I::Item;
    type It1 = I::IntoIter;
    type It2 = I::IntoIter;

    fn first(&mut self) -> Self::It1 {
        self.clone().into_iter()
    }
    fn second(self, _last: Self::It1) -> Self::It2 {
        self.into_iter()
    }
}

///
/// Create a boxed PlotIterator.
///
/// This should be used as a last resort after trying [`chain`](PlotIteratorExt::chain) and [`plots_dyn`].
///
#[deprecated(note = "use into_boxed() instead.")]
pub fn box_plot<'a, X>(
    a: impl PlotIterator<Item = X> + 'a,
) -> Box<dyn PlotIterator<Item = X> + 'a> {
    Box::new(a)
}

impl<'a, X: 'a> PlotIterator for &'a mut dyn PlotIterator<Item = X> {
    type Item = X;
    fn next_bound_point(&mut self) -> Option<Self::Item> {
        (*self).next_bound_point()
    }

    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        (*self).next_plot_point()
    }

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result> {
        (*self).next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        (*self).next_typ()
    }
}

impl<'a, X: 'a> PlotIterator for Box<dyn PlotIterator<Item = X> + 'a> {
    type Item = X;

    fn next_bound_point(&mut self) -> Option<Self::Item> {
        self.as_mut().next_bound_point()
    }

    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        self.as_mut().next_plot_point()
    }

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result> {
        self.as_mut().next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.as_mut().next_typ()
    }
}

///
/// Helper functions to assemble and prepare plots.
///
pub trait PlotIteratorExt: PlotIterator {
    fn chain<B: PlotIterator<Item = Self::Item>>(self, b: B) -> Chain<Self, B>
    where
        Self: Sized,
    {
        Chain::new(self, b)
    }

    ///
    /// Create a boxed PlotIterator.
    ///
    /// This should be used as a last resort after trying [`chain`](PlotIteratorExt::chain) and [`plots_dyn`].
    ///
    fn into_boxed<'a>(self) -> Box<dyn PlotIterator<Item = Self::Item> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }

    fn as_mut_dyn(&mut self) -> &mut dyn PlotIterator<Item = Self::Item>
    where
        Self: Sized,
    {
        self
    }
}

impl<I: PlotIterator> PlotIteratorExt for I {}

/// Iterator over all plots that have been assembled by the user.
/// This trait is used by the poloto renderer to iterate over and render all the plots.
///
/// Renderer will first call `next_bound()` until exhausted in order to find min/max bounds.
///
/// Then renderer will call `next_typ()` to determine  if there is a plot.
/// If next_typ() returned Some(), then it will then call next_name()
/// Then it will call next_plot continuously until it returns None.
///
pub trait PlotIterator {
    type Item;
    fn next_typ(&mut self) -> Option<PlotMetaType>;
    fn next_bound_point(&mut self) -> Option<Self::Item>;
    fn next_plot_point(&mut self) -> PlotResult<Self::Item>;
    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result>;
}

pub(crate) struct RenderablePlotIter<'a, A: PlotIterator> {
    flop: &'a mut A,
}
impl<'a, A: PlotIterator> RenderablePlotIter<'a, A> {
    #[inline(always)]
    pub fn new(flop: &'a mut A) -> Self {
        RenderablePlotIter { flop }
    }
    #[inline(always)]
    pub fn next_plot(&mut self) -> Option<SinglePlotAccessor<A>> {
        if let Some(typ) = self.flop.next_typ() {
            Some(SinglePlotAccessor {
                typ,
                flop: &mut self.flop,
            })
        } else {
            None
        }
    }
}

pub(crate) struct SinglePlotAccessor<'a, A: PlotIterator> {
    typ: PlotMetaType,
    flop: &'a mut A,
}
impl<'b, A: PlotIterator> SinglePlotAccessor<'b, A> {
    #[inline(always)]
    pub fn typ(&mut self) -> PlotMetaType {
        self.typ
    }

    #[inline(always)]
    pub fn name(&mut self, write: &mut dyn fmt::Write) -> Option<fmt::Result> {
        self.flop.next_name(write)
    }

    #[inline(always)]
    pub fn plots<'a>(&'a mut self) -> PlotIt<'a, 'b, A> {
        PlotIt { inner: self }
    }
}

pub(crate) struct PlotIt<'a, 'b, A: PlotIterator> {
    inner: &'a mut SinglePlotAccessor<'b, A>,
}
impl<'a, 'b, A: PlotIterator> Iterator for PlotIt<'a, 'b, A> {
    type Item = A::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if let PlotResult::Some(a) = self.inner.flop.next_plot_point() {
            Some(a)
        } else {
            None
        }
    }
}

///
/// Used to distinguish between one plot's points being rendered, vs all plot's points being rendered.
///
pub enum PlotResult<T> {
    Some(T),
    None,
    Finished,
}

///
/// Write some text in the legend. This doesnt increment the plot number.
///
pub fn text<X: PlotNum, Y: PlotNum, D: Display>(
    name: D,
) -> SinglePlot<std::iter::Empty<(X, Y)>, D> {
    SinglePlot::new(PlotMetaType::Text, name, std::iter::empty())
}

pub(crate) fn bars<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(
    name: D,
    it: I,
) -> SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    SinglePlot::new(PlotMetaType::Plot(PlotType::Bars), name, it)
}

/// Create a histogram from plots using SVG rect elements.
/// Each bar's left side will line up with a point.
/// Each rect element belongs to the `.poloto[N]fill` css class.
pub fn histogram<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(
    name: D,
    it: I,
) -> SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    SinglePlot::new(PlotMetaType::Plot(PlotType::Histo), name, it)
}

/// Create a scatter plot from plots, using a SVG path with lines with zero length.
/// Each point can be sized using the stroke width.
/// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
/// with the latter class overriding the former.
pub fn scatter<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    SinglePlot::new(PlotMetaType::Plot(PlotType::Scatter), name, it)
}

/// Create a line from plots that will be filled underneath using a SVG path element.
/// The path element belongs to the `.poloto[N]fill` css class.
pub fn line_fill<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(
    name: D,
    it: I,
) -> SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    SinglePlot::new(PlotMetaType::Plot(PlotType::LineFill), name, it)
}

/// Create a line from plots that will be filled using a SVG path element.
/// The first and last points will be connected and then filled in.
/// The path element belongs to the `.poloto[N]fill` css class.
pub fn line_fill_raw<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(
    name: D,
    it: I,
) -> SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    SinglePlot::new(PlotMetaType::Plot(PlotType::LineFillRaw), name, it)
}

/// Create a line from plots using a SVG path element.
/// The path element belongs to the `.poloto[N]fill` css class.    
pub fn line<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display>(name: D, it: I) -> SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    SinglePlot::new(PlotMetaType::Plot(PlotType::Line), name, it)
}

enum SinglePlotInner<I: PlotIter> {
    Ready(I),
    First(I, I::It1),
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
