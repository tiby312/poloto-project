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
/// Create a [`PlotsDyn`]
///
pub fn plots_dyn<F: PlotIterator>(vec: Vec<F>) -> PlotsDyn<F> {
    PlotsDyn {
        flop: vec,
        bound_counter: 0,
        plot_counter: 0,
    }
}

///
/// Allows a user to collect plots inside of a loop instead of chaining plots together.
///
pub struct PlotsDyn<F: PlotIterator> {
    bound_counter: usize,
    plot_counter: usize,
    flop: Vec<F>,
}

impl<F: PlotIterator> PlotIterator for PlotsDyn<F> {
    type Item = F::Item;

    #[inline(always)]
    fn next_bound_point(&mut self) -> Option<Self::Item> {
        loop {
            if self.bound_counter >= self.flop.len() {
                return None;
            }
            if let Some(a) = self.flop[self.bound_counter].next_bound_point() {
                return Some(a);
            }
            self.bound_counter += 1;
        }
    }

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if self.plot_counter >= self.flop.len() {
            None
        } else {
            self.flop[self.plot_counter].next_typ()
        }
    }

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        if self.plot_counter >= self.flop.len() {
            return PlotResult::Finished;
        }
        let a = self.flop[self.plot_counter].next_plot_point();
        if let PlotResult::None = a {
            self.plot_counter += 1;
        }
        a
    }

    #[inline(always)]
    fn next_name(&mut self, write: &mut dyn fmt::Write) -> fmt::Result {
        self.flop[self.plot_counter].next_name(write)
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

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
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

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
        self.as_mut().next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.as_mut().next_typ()
    }
}

pub trait PlotIteratorAndMarkers {
    type X;
    type Y;
    type Iter: PlotIterator<Item = (Self::X, Self::Y)>;
    type XI: Iterator<Item = Self::X>;
    type YI: Iterator<Item = Self::Y>;
    fn unpack(self) -> (Self::Iter, Self::XI, Self::YI);
}

impl<X: PlotNum, Y: PlotNum, I: PlotIterator<Item = (X, Y)>> PlotIteratorAndMarkers for I {
    type X = X;
    type Y = Y;
    type Iter = Self;
    type XI = std::iter::Empty<X>;
    type YI = std::iter::Empty<Y>;
    fn unpack(self) -> (Self::Iter, Self::XI, Self::YI) {
        (self, std::iter::empty(), std::iter::empty())
    }
}

pub struct MarkersStruct<I: PlotIterator<Item = (XI::Item, YI::Item)>, XI: Iterator, YI: Iterator> {
    plots: I,
    xmarkers: XI,
    ymarkers: YI,
}

impl<I: PlotIterator<Item = (XI::Item, YI::Item)>, XI: Iterator, YI: Iterator>
    PlotIteratorAndMarkers for MarkersStruct<I, XI, YI>
{
    type X = XI::Item;
    type Y = YI::Item;
    type Iter = I;
    type XI = XI;
    type YI = YI;
    fn unpack(self) -> (Self::Iter, Self::XI, Self::YI) {
        (self.plots, self.xmarkers, self.ymarkers)
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
        Chain {
            a: self,
            b,
            started: false,
        }
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
    fn next_name(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
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
    pub fn name(&mut self, write: &mut dyn fmt::Write) -> fmt::Result {
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

pub enum SinglePlotInner<I: PlotIter> {
    Ready(I),
    First(I, I::It1),
    Second(I::It2),
    Done,
}
impl<I: PlotIter> SinglePlotInner<I> {
    fn is_done(&self) -> bool {
        if let SinglePlotInner::Done = self {
            true
        } else {
            false
        }
    }
    
}

///
/// Represents a single plot.
///
pub struct SinglePlot<I: PlotIter, D: Display> {
    inner: Option<SinglePlotInner<I>>,
    name: D,
    typ: PlotMetaType
}
impl<I: PlotIter, D: Display> SinglePlot<I, D>
where
    I::Item1: Unwrapper,
    I::Item2: Unwrapper,
{
    #[inline(always)]
    fn new(typ: PlotMetaType, name: D, plots: I) -> Self {
        SinglePlot {
            inner: Some(SinglePlotInner::Ready(plots)),
            name,
            typ
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
    fn next_bound_point(&mut self) -> Option<Self::Item> {
        if let SinglePlotInner::Ready(_) = self.inner.as_ref().unwrap() {
            if let SinglePlotInner::Ready(mut a) = self.inner.take().unwrap() {
                let b = a.first();
                self.inner = Some(SinglePlotInner::First(a, b));
            } else {
                unreachable!();
            }
        }

        if let SinglePlotInner::First(_, a) = &mut self.inner.as_mut().unwrap() {
            a.next().map(|x| x.unwrap())
        } else {
            panic!("next_bound_point called too late!")
        }
    }

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        if let SinglePlotInner::Second(a) = self.inner.as_mut().unwrap() {
            if let Some(k) = a.next() {
                PlotResult::Some(k.unwrap())
            } else if !self.inner.as_ref().unwrap().is_done() {
                self.inner = Some(SinglePlotInner::Done);
                PlotResult::None
            } else {
                unreachable!();
            }
        } else if let SinglePlotInner::Done = self.inner.as_ref().unwrap() {
            PlotResult::Finished
        } else {
            unreachable!();
        }
    }

    #[inline(always)]
    fn next_name(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.name)
    }

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if let SinglePlotInner::First(..) = self.inner.as_ref().unwrap() {
            if let SinglePlotInner::First(a, b) = self.inner.take().unwrap() {
                self.inner = Some(SinglePlotInner::Second(a.second(b)));
                Some(self.typ)
            } else {
                unreachable!();
            }
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
    started: bool,
}
impl<A: PlotIterator, B: PlotIterator<Item = A::Item>> PlotIterator for Chain<A, B> {
    type Item = A::Item;

    #[inline(always)]
    fn next_bound_point(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.a.next_bound_point() {
            Some(a)
        } else {
            self.b.next_bound_point()
        }
    }

    #[inline(always)]
    fn next_plot_point(&mut self) -> PlotResult<Self::Item> {
        match self.a.next_plot_point() {
            PlotResult::Some(a) => PlotResult::Some(a),
            PlotResult::None => PlotResult::None,
            PlotResult::Finished => self.b.next_plot_point(),
        }
    }

    #[inline(always)]
    fn next_name(&mut self, mut writer: &mut dyn fmt::Write) -> fmt::Result {
        if !self.started {
            self.a.next_name(&mut writer)
        } else {
            self.b.next_name(&mut writer)
        }
    }

    #[inline(always)]
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if let Some(a) = self.a.next_typ() {
            Some(a)
        } else {
            self.started = true;
            self.b.next_typ()
        }
    }
}

pub trait Markerable: PlotIterator<Item = (Self::X, Self::Y)> {
    type X;
    type Y;
    ///
    /// Specify x and y values that must fit into the viewport.
    ///
    fn markers<XI: IntoIterator<Item = Self::X>, YI: IntoIterator<Item = Self::Y>>(
        self,
        xmarkers: XI,
        ymarkers: YI,
    ) -> MarkersStruct<Self, XI::IntoIter, YI::IntoIter>
    where
        Self: Sized,
    {
        MarkersStruct {
            plots: self,
            xmarkers: xmarkers.into_iter(),
            ymarkers: ymarkers.into_iter(),
        }
    }
}
impl<X, Y, I: PlotIterator<Item = (X, Y)>> Markerable for I {
    type X = X;
    type Y = Y;
}

///
/// Specify x and y values that must fit into the viewport.
///
/// Also consider [`build::Markerable::markers()`]
///
pub fn markers<
    X,
    Y,
    P: PlotIterator<Item = (X, Y)>,
    XI: IntoIterator<Item = X>,
    YI: IntoIterator<Item = Y>,
>(
    plots: P,
    x: XI,
    y: YI,
) -> MarkersStruct<P, XI::IntoIter, YI::IntoIter> {
    MarkersStruct {
        plots,
        xmarkers: x.into_iter(),
        ymarkers: y.into_iter(),
    }
}
