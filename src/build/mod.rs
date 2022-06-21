//!
//! Tools for assembling plots
//!
//!

use super::*;

pub mod bar;
pub mod crop;
pub mod iter;
pub mod unwrapper;
use marker::Area;
use marker::Markerable;

pub mod marker;

pub mod plot_iter_impl;
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
/// Iterator over all plots that have been assembled by the user.
/// This trait is used by the poloto renderer to iterate over and render all the plots.
///
pub trait PlotIterator<X, Y> {
    fn next_typ(&mut self) -> Option<PlotMetaType>;
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)>;
    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result>;
}

///
/// Allows the user to chain together PlotIterators.
///
pub trait PlotIteratorExt<X, Y>: PlotIterator<X, Y> {
    /// Chain together PlotIterators.
    ///
    /// ```
    /// use poloto::build::PlotIteratorExt;
    /// let data1=[[5,2],[4,3]];
    /// let data2=[[2,4],[2,2]];
    /// let a=poloto::build::line("test1",&data1);
    /// let b=poloto::build::scatter("test2",&data2);
    /// a.chain(b);
    /// ```
    ///
    fn chain<B: PlotIterator<X, Y>>(self, b: B) -> Chain<Self, B>
    where
        Self: Sized,
    {
        Chain::new(self, b)
    }
}
impl<X, Y, I: PlotIterator<X, Y>> PlotIteratorExt<X, Y> for I {}

pub(crate) struct RenderablePlotIter<'a, A> {
    flop: &'a mut A,
}
impl<'a, A> RenderablePlotIter<'a, A> {
    #[inline(always)]
    pub fn new(flop: &'a mut A) -> Self {
        RenderablePlotIter { flop }
    }
    #[inline(always)]
    pub fn next_plot<X, Y>(&mut self) -> Option<SinglePlotAccessor<A>>
    where
        A: PlotIterator<X, Y>,
    {
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

pub(crate) struct SinglePlotAccessor<'a, A> {
    typ: PlotMetaType,
    flop: &'a mut A,
}
impl<'b, A> SinglePlotAccessor<'b, A> {
    #[inline(always)]
    pub fn typ(&mut self) -> PlotMetaType {
        self.typ
    }

    #[inline(always)]
    pub fn name<X, Y>(&mut self, write: &mut dyn fmt::Write) -> Option<fmt::Result>
    where
        A: PlotIterator<X, Y>,
    {
        self.flop.next_name(write)
    }

    #[inline(always)]
    pub fn plots<X, Y>(&mut self) -> impl Iterator<Item = (X, Y)> + '_
    where
        A: PlotIterator<X, Y>,
    {
        //TODO borrow this tick in broccoli for iterating over all elements
        let f: &mut _ = self.flop;
        std::iter::from_fn(move || {
            if let PlotResult::Some(a) = f.next_plot_point() {
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

pub struct SinglePlotBuilder<X, Y, I: Iterator<Item = (X, Y)>> {
    area: Area<X, Y>,
    iter: I,
}
impl<X: PlotNum, Y: PlotNum> SinglePlotBuilder<X, Y, std::vec::IntoIter<(X, Y)>> {
    pub fn new_buffered<I: Iterator<Item = (X, Y)>>(iter: I) -> Self {
        let mut vec = Vec::with_capacity(iter.size_hint().0);
        let mut area = Area::new();
        for (x, y) in iter {
            area.grow(Some(&x), Some(&y));
            vec.push((x, y));
        }
        SinglePlotBuilder {
            area,
            iter: vec.into_iter(),
        }
    }
}

impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)>> SinglePlotBuilder<X, Y, I> {
    pub fn new_cloned(iter: I) -> Self
    where
        I: Clone,
    {
        let mut area = Area::new();
        for (x, y) in iter.clone() {
            area.grow(Some(&x), Some(&y));
        }
        SinglePlotBuilder { area, iter }
    }

    /*
    pub fn new_rect_bound_plot(x: [X; 2], y: [Y; 2], iter: I) -> Self {
        let mut area = Area::new();
        area.grow(Some(&x[0]), Some(&y[0]));
        area.grow(Some(&x[1]), Some(&y[1]));
        SinglePlotBuilder { area, iter }
    }

    pub fn new_custom_bound_plot<J: Iterator<Item = (X, Y)>>(bound: J, iter: I) -> Self {
        let mut area = Area::new();
        for (x, y) in bound {
            area.grow(Some(&x), Some(&y));
        }
        SinglePlotBuilder { area, iter }
    }
    */

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter<D: Display>(self, name: D) -> SinglePlot<X, Y, I, D> {
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::Scatter),
            name,
            self.iter,
            self.area,
        )
    }

    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.    
    pub fn line<D: Display>(self, name: D) -> SinglePlot<X, Y, I, D> {
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::Line),
            name,
            self.iter,
            self.area,
        )
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill<D: Display>(self, name: D) -> SinglePlot<X, Y, I, D> {
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::LineFill),
            name,
            self.iter,
            self.area,
        )
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw<D: Display>(self, name: D) -> SinglePlot<X, Y, I, D> {
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::LineFillRaw),
            name,
            self.iter,
            self.area,
        )
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram<D: Display>(self, name: D) -> SinglePlot<X, Y, I, D> {
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::Histo),
            name,
            self.iter,
            self.area,
        )
    }

    pub(crate) fn bars<D: Display>(self, name: D) -> SinglePlot<X, Y, I, D> {
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::Bars),
            name,
            self.iter,
            self.area,
        )
    }
}

///
/// Write some text in the legend. This doesnt increment the plot number.
///
pub fn text<X: PlotNum, Y: PlotNum, D: Display>(
    name: D,
) -> SinglePlot<X, Y, std::iter::Empty<(X, Y)>, D> {
    let f = SinglePlotBuilder::new_cloned(std::iter::empty());
    SinglePlot::new(PlotMetaType::Text, name, f.iter, f.area)
}

///
/// Ensure that the origin point is within view.
///
pub fn origin<X: HasZero + PlotNum, Y: HasZero + PlotNum>(
) -> plot_iter_impl::Marker<std::option::IntoIter<X>, std::option::IntoIter<Y>> {
    markers(Some(X::zero()), Some(Y::zero()))
}

///
/// Ensure the list of marker values are within view.
///
pub fn markers<XI: IntoIterator, YI: IntoIterator>(
    x: XI,
    y: YI,
) -> plot_iter_impl::Marker<XI::IntoIter, YI::IntoIter>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
    plot_iter_impl::Marker::new(x, y)
}

///
/// Create a [`PlotsDyn`](plot_iter_impl::PlotsDyn)
///
pub fn plots_dyn<X, Y, F: PlotIterator<X, Y>>(vec: Vec<F>) -> plot_iter_impl::PlotsDyn<F> {
    plot_iter_impl::PlotsDyn::new(vec)
}

trait PlotIteratorAndMarkers<X, Y>: Markerable<X, Y> + PlotIterator<X, Y> {}

impl<X, Y, I: Markerable<X, Y> + PlotIterator<X, Y>> PlotIteratorAndMarkers<X, Y> for I {}

impl<'a, X: 'a, Y: 'a> PlotIterator<X, Y> for &'a mut dyn PlotIterator<X, Y> {
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
        (*self).next_plot_point()
    }

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result> {
        (*self).next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        (*self).next_typ()
    }
}

pub struct BoxedPlot<'a, X, Y> {
    inner: Box<dyn PlotIteratorAndMarkers<X, Y> + 'a>,
}

impl<'a, X, Y> BoxedPlot<'a, X, Y> {
    pub fn new<A: Markerable<X, Y> + PlotIterator<X, Y> + 'a>(a: A) -> BoxedPlot<'a, X, Y> {
        BoxedPlot { inner: Box::new(a) }
    }
}

impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> Markerable<X, Y> for BoxedPlot<'a, X, Y> {
    fn increase_area(&mut self, area: &mut Area<X, Y>) {
        self.inner.as_mut().increase_area(area);
    }
}

impl<'a, X: 'a, Y: 'a> PlotIterator<X, Y> for BoxedPlot<'a, X, Y> {
    fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
        self.inner.as_mut().next_plot_point()
    }

    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result> {
        self.inner.as_mut().next_name(w)
    }

    fn next_typ(&mut self) -> Option<PlotMetaType> {
        self.inner.as_mut().next_typ()
    }
}
