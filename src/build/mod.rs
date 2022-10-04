//!
//! Tools for assembling plots
//!
//!

use super::*;

pub mod bar;
pub mod crop;
pub mod unwrapper;
use marker::Area;

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
pub trait PlotIterator {
    type X: PlotNum;
    type Y: PlotNum;
    fn increase_area(&mut self, area: &mut Area<Self::X, Self::Y>);
    fn next_typ(&mut self) -> Option<PlotMetaType>;
    fn next_plot_point(&mut self) -> PlotResult<(Self::X, Self::Y)>;
    fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result>;
}

///
/// Allows the user to chain together PlotIterators.
///
pub trait PlotIteratorExt: PlotIterator {
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
    fn chain<B: PlotIterator<X = Self::X, Y = Self::Y>>(self, b: B) -> Chain<Self, B>
    where
        Self: Sized,
    {
        Chain::new(self, b)
    }
}
impl<I: PlotIterator> PlotIteratorExt for I {}

pub(crate) struct RenderablePlotIter<'a, A> {
    flop: &'a mut A,
}
impl<'a, A> RenderablePlotIter<'a, A> {
    #[inline(always)]
    pub fn new(flop: &'a mut A) -> Self {
        RenderablePlotIter { flop }
    }
    #[inline(always)]
    pub fn next_plot(&mut self) -> Option<SinglePlotAccessor<A>>
    where
        A: PlotIterator,
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
    pub fn name(&mut self, write: &mut dyn fmt::Write) -> Option<fmt::Result>
    where
        A: PlotIterator,
    {
        self.flop.next_name(write)
    }

    #[inline(always)]
    pub fn plots(&mut self) -> impl Iterator<Item = (A::X, A::Y)> + '_
    where
        A: PlotIterator,
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

// pub struct SinglePlotBuilder<X, Y, I: Iterator<Item = (X, Y)>, D: Display> {
//     area: Area<X, Y>,
//     iter: I,
//     label: D,
// }
// impl<X: PlotNum, Y: PlotNum, D: Display> SinglePlotBuilder<X, Y, std::vec::IntoIter<(X, Y)>, D> {
//     pub fn new_buffered<I: Iterator<Item = (X, Y)>>(iter: I, label: D) -> Self {
//         let mut vec = Vec::with_capacity(iter.size_hint().0);
//         let mut area = Area::new();
//         for (x, y) in iter {
//             area.grow(Some(&x), Some(&y));
//             vec.push((x, y));
//         }
//         SinglePlotBuilder {
//             area,
//             iter: vec.into_iter(),
//             label,
//         }
//     }
// }

// impl<X: PlotNum, Y: PlotNum, I: Iterator<Item = (X, Y)>, D: Display> SinglePlotBuilder<X, Y, I, D> {
//     pub fn new_cloned(iter: I, label: D) -> Self
//     where
//         I: Clone,
//     {
//         let mut area = Area::new();
//         for (x, y) in iter.clone() {
//             area.grow(Some(&x), Some(&y));
//         }
//         SinglePlotBuilder { area, iter, label }
//     }

//     pub fn scatter(self) -> SinglePlot<X, Y, I, D> {
//         SinglePlot::new(
//             PlotMetaType::Plot(PlotType::Scatter),
//             self.label,
//             self.iter,
//             self.area,
//         )
//     }

//     pub fn line(self) -> SinglePlot<X, Y, I, D> {
//         SinglePlot::new(
//             PlotMetaType::Plot(PlotType::Line),
//             self.label,
//             self.iter,
//             self.area,
//         )
//     }

//     pub fn line_fill(self) -> SinglePlot<X, Y, I, D> {
//         SinglePlot::new(
//             PlotMetaType::Plot(PlotType::LineFill),
//             self.label,
//             self.iter,
//             self.area,
//         )
//     }

//     pub fn line_fill_raw(self) -> SinglePlot<X, Y, I, D> {
//         SinglePlot::new(
//             PlotMetaType::Plot(PlotType::LineFillRaw),
//             self.label,
//             self.iter,
//             self.area,
//         )
//     }

//     pub fn histogram(self) -> SinglePlot<X, Y, I, D> {
//         SinglePlot::new(
//             PlotMetaType::Plot(PlotType::Histo),
//             self.label,
//             self.iter,
//             self.area,
//         )
//     }

//     pub(crate) fn bars(self) -> SinglePlot<X, Y, I, D> {
//         SinglePlot::new(
//             PlotMetaType::Plot(PlotType::Bars),
//             self.label,
//             self.iter,
//             self.area,
//         )
//     }
// }

// pub fn text<X: PlotNum, Y: PlotNum, D: Display>(
//     name: D,
// ) -> SinglePlot<X, Y, std::iter::Empty<(X, Y)>, D> {
//     let f = SinglePlotBuilder::new_cloned(std::iter::empty(), name);
//     SinglePlot::new(PlotMetaType::Text, f.label, f.iter, f.area)
// }

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
pub fn plots_dyn<F: PlotIterator, I: IntoIterator<Item = F>>(
    stuff: I,
) -> plot_iter_impl::PlotsDyn<F> {
    plot_iter_impl::PlotsDyn::new(stuff.into_iter().collect::<Vec<_>>())
}

pub struct BoxedPlot<'a, X, Y> {
    inner: Box<dyn PlotIterator<X = X, Y = Y> + 'a>,
}

impl<'a, X, Y> BoxedPlot<'a, X, Y> {
    pub fn new<A: PlotIterator<X = X, Y = Y> + 'a>(a: A) -> BoxedPlot<'a, X, Y> {
        BoxedPlot { inner: Box::new(a) }
    }
}

impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> PlotIterator for BoxedPlot<'a, X, Y> {
    type X = X;
    type Y = Y;
    fn increase_area(&mut self, area: &mut Area<X, Y>) {
        self.inner.as_mut().increase_area(area);
    }
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

// pub fn buffered_plot<X: PlotNum, Y: PlotNum, I: Iterator, D: Display>(
//     label: D,
//     iter: I,
// ) -> build::SinglePlotBuilder<X, Y, std::vec::IntoIter<(X, Y)>, D>
// where
//     I::Item: Clone + build::unwrapper::Unwrapper<Item = (X, Y)>,
// {
//     build::SinglePlotBuilder::new_buffered(build::unwrapper::UnwrapperIter(iter), label)
// }

// pub fn cloned_plot<X: PlotNum, Y: PlotNum, I: Iterator, D: Display>(
//     label: D,
//     iter: I,
// ) -> build::SinglePlotBuilder<X, Y, build::unwrapper::UnwrapperIter<I>, D>
// where
//     I: Clone,
//     I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
// {
//     build::SinglePlotBuilder::new_cloned(build::unwrapper::UnwrapperIter(iter), label)
// }

pub struct PointBuilder<D: Display> {
    label: D,
    typ: PlotMetaType,
}

impl<D: Display> PointBuilder<D> {
    pub fn cloned<X: PlotNum, Y: PlotNum, I: Iterator>(
        self,
        it: I,
    ) -> SinglePlot<X, Y, build::unwrapper::UnwrapperIter<I>, D>
    where
        I: Clone,
        I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
    {
        let mut area = Area::new();
        for k in it.clone() {
            let (x, y) = k.unwrap();
            area.grow(Some(&x), Some(&y));
        }
        SinglePlot::new(
            self.typ,
            self.label,
            build::unwrapper::UnwrapperIter(it),
            area,
        )
    }

    pub fn buffered<X: PlotNum, Y: PlotNum, I: Iterator>(
        self,
        it: I,
    ) -> SinglePlot<X, Y, std::vec::IntoIter<(X, Y)>, D>
    where
        I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
    {
        let mut vec = Vec::with_capacity(it.size_hint().0);
        let mut area = Area::new();
        for j in it {
            let (x, y) = j.unwrap();
            area.grow(Some(&x), Some(&y));
            vec.push((x, y));
        }
        SinglePlot::new(self.typ, self.label, vec.into_iter(), area)
    }
}

pub struct SinglePlotBuilder1<D> {
    label: D,
}

impl<D: Display> SinglePlotBuilder1<D> {
    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.  
    pub fn line(self) -> PointBuilder<D> {
        PointBuilder {
            label: self.label,
            typ: PlotMetaType::Plot(PlotType::Line),
        }
    }

    pub(crate) fn bars(self) -> PointBuilder<D> {
        PointBuilder {
            label: self.label,
            typ: PlotMetaType::Plot(PlotType::Bars),
        }
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter(self) -> PointBuilder<D> {
        PointBuilder {
            label: self.label,
            typ: PlotMetaType::Plot(PlotType::Scatter),
        }
    }
    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram(self) -> PointBuilder<D> {
        PointBuilder {
            label: self.label,
            typ: PlotMetaType::Plot(PlotType::Histo),
        }
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill(self) -> PointBuilder<D> {
        PointBuilder {
            label: self.label,
            typ: PlotMetaType::Plot(PlotType::LineFill),
        }
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw(self) -> PointBuilder<D> {
        PointBuilder {
            label: self.label,
            typ: PlotMetaType::Plot(PlotType::LineFillRaw),
        }
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    pub fn text<X: PlotNum, Y: PlotNum>(self) -> SinglePlot<X, Y, std::iter::Empty<(X, Y)>, D> {
        SinglePlot::new(
            PlotMetaType::Text,
            self.label,
            std::iter::empty(),
            Area::new(),
        )
    }
}
pub fn plot<D: Display>(label: D) -> SinglePlotBuilder1<D> {
    SinglePlotBuilder1 { label }
}
