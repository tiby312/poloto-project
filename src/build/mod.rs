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
pub fn plots_dyn<F: RenderablePlotIterator>() -> PlotsDynBuilder<F> {
    PlotsDynBuilder::new()
}

pub struct PlotsDynBuilder<F: RenderablePlotIterator> {
    inner: Vec<F>,
}
impl<F: RenderablePlotIterator> Default for PlotsDynBuilder<F> {
    fn default() -> Self {
        Self::new()
    }
}
impl<F: RenderablePlotIterator> PlotsDynBuilder<F> {
    pub fn new() -> Self {
        PlotsDynBuilder { inner: vec![] }
    }
    pub fn add(&mut self, a: F) -> &mut Self {
        self.inner.push(a);
        self
    }
    pub fn move_into(&mut self) -> PlotsDynBuilder<F> {
        let mut flop = vec![];
        std::mem::swap(&mut flop, &mut self.inner);

        PlotsDynBuilder { inner: flop }
    }

    pub fn build(&mut self) -> PlotsDyn<F> {
        let a = self.move_into();
        PlotsDyn {
            bound_counter: 0,
            plot_counter: 0,
            flop: a.inner,
        }
    }
}

///
/// Allows a user to collect plots inside of a loop instead of chaining plots together.
///
pub struct PlotsDyn<F: RenderablePlotIterator> {
    bound_counter: usize,
    plot_counter: usize,
    flop: Vec<F>,
}

impl<F: RenderablePlotIterator> RenderablePlotIterator for PlotsDyn<F> {
    type X = F::X;
    type Y = F::Y;
    fn next_bound_point(&mut self) -> Option<(Self::X, Self::Y)> {
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
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if self.plot_counter >= self.flop.len() {
            None
        } else {
            self.flop[self.plot_counter].next_typ()
        }
    }
    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)> {
        if self.plot_counter == self.flop.len() {
            return PlotResult::Finished;
        }
        let a = self.flop[self.plot_counter].next_plot_point();
        if let PlotResult::None = a {
            self.plot_counter += 1;
        }
        a
    }

    fn next_name(&mut self, write: &mut dyn fmt::Write) -> fmt::Result {
        self.flop[self.plot_counter].next_name(write)
    }
}

///
/// Create a boxed RenderablePlotIterator.
///
/// This should be used as a last resort after trying [`chain`](RenderablePlotIteratorExt::chain) and [`plots_dyn`].
///
pub fn box_plot<'a, X: PlotNum, Y: PlotNum>(
    a: impl RenderablePlotIterator<X = X, Y = Y> + 'a,
) -> Box<dyn RenderablePlotIterator<X = X, Y = Y> + 'a> {
    Box::new(a)
}

impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> RenderablePlotIterator
    for Box<dyn RenderablePlotIterator<X = X, Y = Y> + 'a>
{
    type X = X;

    type Y = Y;

    fn next_bound_point(&mut self) -> Option<(Self::X, Self::Y)> {
        self.as_mut().next_bound_point()
    }

    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)> {
        self.as_mut().next_plot_point()
    }

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> fmt::Result {
        self.as_mut().next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.as_mut().next_typ()
    }
}

///
/// Helper functions to assemble and prepare plots.
///
pub trait RenderablePlotIteratorExt: RenderablePlotIterator {
    fn chain<B: RenderablePlotIterator>(self, b: B) -> Chain<Self, B>
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
    /// Compute min/max bounds and prepare for next stage in pipeline.
    ///
    fn build(self) -> Data<Self>
    where
        Self: Sized,
    {
        self.build_with(None, None)
    }

    ///
    /// Similar to `build` except additionally specify marker values that the viewport must fit.
    ///
    fn build_with(
        mut self,
        xmarker: impl IntoIterator<Item = Self::X>,
        ymarker: impl IntoIterator<Item = Self::Y>,
    ) -> Data<Self>
    where
        Self: Sized,
    {
        let ii = std::iter::from_fn(|| self.next_bound_point());

        let (boundx, boundy) = util::find_bounds(ii, xmarker, ymarker);

        let boundx = ticks::DataBound {
            min: boundx[0],
            max: boundx[1],
        };
        let boundy = ticks::DataBound {
            min: boundy[0],
            max: boundy[1],
        };

        Data::new(boundx, boundy, self)
    }
}
impl<I: RenderablePlotIterator> RenderablePlotIteratorExt for I {}

/// Iterator over all plots that have been assembled by the user.
/// This trait is used by the poloto renderer to iterate over and render all the plots.
///
/// Renderer will first call `next_bound()` until exhausted in order to find min/max bounds.
///
/// Then renderer will call `next_typ()` to determine  if there is a plot.
/// If next_typ() returned Some(), then it will then call next_name()
/// Then it will call next_plot continuously until it returns None.
///
pub trait RenderablePlotIterator {
    type X: PlotNum;
    type Y: PlotNum;
    fn next_typ(&mut self) -> Option<PlotMetaType>;
    fn next_bound_point(&mut self) -> Option<(Self::X, Self::Y)>;
    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)>;
    fn next_name(&mut self, w: &mut dyn fmt::Write) -> fmt::Result;
}

pub(crate) struct RenderablePlotIter<A: RenderablePlotIterator> {
    flop: A,
}
impl<A: RenderablePlotIterator> RenderablePlotIter<A> {
    pub fn new(flop: A) -> Self {
        RenderablePlotIter { flop }
    }
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

pub(crate) struct SinglePlotAccessor<'a, A: RenderablePlotIterator> {
    typ: PlotMetaType,
    flop: &'a mut A,
}
impl<'b, A: RenderablePlotIterator> SinglePlotAccessor<'b, A> {
    pub fn typ(&mut self) -> PlotMetaType {
        self.typ
    }
    pub fn name(&mut self, write: &mut dyn fmt::Write) -> fmt::Result {
        self.flop.next_name(write)
    }
    pub fn plots<'a>(&'a mut self) -> impl Iterator<Item = (A::X, A::Y)> + 'a {
        std::iter::from_fn(|| {
            if let PlotResult::Some(a) = self.flop.next_plot_point() {
                Some(a)
            } else {
                None
            }
        })
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

///
/// Represents a single plot.
///
pub struct SinglePlot<I: PlotIter, D: Display> {
    buffer1: Option<I::It1>,
    buffer2: Option<I::It2>,
    plots: Option<I>,
    name: D,
    typ: PlotMetaType,
    hit_end: bool,
    started: bool,
}
impl<I: PlotIter, D: Display> SinglePlot<I, D>
where
    I::Item1: Unwrapper,
    I::Item2: Unwrapper,
{
    fn new(typ: PlotMetaType, name: D, plots: I) -> Self {
        SinglePlot {
            buffer1: None,
            buffer2: None,
            plots: Some(plots),
            name,
            typ,
            hit_end: false,
            started: false,
        }
    }
}
impl<X: PlotNum, Y: PlotNum, I: PlotIter, D: Display> RenderablePlotIterator for SinglePlot<I, D>
where
    I::Item1: Unwrapper<Item = (X, Y)>,
    I::Item2: Unwrapper<Item = (X, Y)>,
{
    type X = X;
    type Y = Y;
    fn next_bound_point(&mut self) -> Option<(Self::X, Self::Y)> {
        if self.buffer1.is_none() {
            self.buffer1 = Some(self.plots.as_mut().unwrap().first());
        }

        self.buffer1.as_mut().unwrap().next().map(|x| x.unwrap())
    }

    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)> {
        if let Some(d) = self.buffer1.take() {
            self.buffer2 = Some(self.plots.take().unwrap().second(d));
        }

        if let Some(bb) = self.buffer2.as_mut() {
            if let Some(k) = bb.next() {
                PlotResult::Some(k.unwrap())
            } else if !self.hit_end {
                self.hit_end = true;
                PlotResult::None
            } else {
                PlotResult::Finished
            }
        } else {
            PlotResult::Finished
        }
    }

    fn next_name(&mut self, writer: &mut dyn fmt::Write) -> fmt::Result {
        write!(writer, "{}", self.name)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if !self.started {
            self.started = true;
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
    started: bool,
}
impl<A: RenderablePlotIterator, B: RenderablePlotIterator<X = A::X, Y = A::Y>>
    RenderablePlotIterator for Chain<A, B>
{
    type X = A::X;
    type Y = A::Y;
    fn next_bound_point(&mut self) -> Option<(Self::X, Self::Y)> {
        if let Some(a) = self.a.next_bound_point() {
            Some(a)
        } else {
            self.b.next_bound_point()
        }
    }

    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)> {
        match self.a.next_plot_point() {
            PlotResult::Some(a) => PlotResult::Some(a),
            PlotResult::None => PlotResult::None,
            PlotResult::Finished => self.b.next_plot_point(),
        }
    }

    fn next_name(&mut self, mut writer: &mut dyn fmt::Write) -> fmt::Result {
        if !self.started {
            self.a.next_name(&mut writer)
        } else {
            self.b.next_name(&mut writer)
        }
    }
    fn next_typ(&mut self) -> Option<PlotMetaType> {
        if let Some(a) = self.a.next_typ() {
            Some(a)
        } else {
            self.started = true;
            self.b.next_typ()
        }
    }
}
