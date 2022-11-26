//!
//! Tools for assembling plots
//!
//!

use super::*;

pub mod bar;
pub mod crop;
pub mod output_zip;
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
    /// let a=poloto::build::plot("test1").line(data1);
    /// let b=poloto::build::plot("test2").scatter(data2);
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
                flop: self.flop,
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

pub trait PlotIt {
    type X: PlotNum;
    type Y: PlotNum;
    type It: Iterator<Item = (Self::X, Self::Y)>;
    fn unpack(self, area: &mut Area<Self::X, Self::Y>) -> Self::It;
}

pub fn cloned<X: PlotNum, Y: PlotNum, I: Iterator>(it: I) -> ClonedPlotIt<I>
where
    I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
{
    ClonedPlotIt(it)
}

// pub fn buffered_1d<N: PlotNum, I: Iterator>(it: I) -> BufferedPlot1D<N>
// where
//     I::Item: Unwrapper<Item = N>,
// {
//     BufferedPlot1D::new(it)
// }
// pub fn cloned_1d<N: PlotNum, I: Iterator + Clone>(it: I) -> ClonedPlot1d<N, I>
// where
//     I::Item: Unwrapper<Item = N>,
// {
//     ClonedPlot1d::new(it)
// }

// pub struct ClonedPlot1dRef<'a, N, I> {
//     it: &'a I,
//     bound: &'a [Option<N>; 2],
// }

// impl<'a, N: PlotNum, I: Iterator + Clone> PlotIt1D for ClonedPlot1dRef<'a, N, I>
// where
//     I::Item: Unwrapper<Item = N>,
// {
//     type Item = N;
//     type It = UnwrapperIter<I>;

//     fn unpack(self) -> ([Option<Self::Item>; 2], Self::It) {
//         (*self.bound, UnwrapperIter(self.it.clone()))
//     }
// }
// pub struct ClonedPlot1d<N, I> {
//     it: I,
//     bound: [Option<N>; 2],
// }

// impl<N: PlotNum, I: Iterator + Clone> ClonedPlot1d<N, I>
// where
//     I::Item: Unwrapper<Item = N>,
// {
//     pub fn new(it: I) -> Self {
//         todo!()
//     }
//     pub fn bound(&self) -> ClonedPlot1dRef<N, I> {
//         todo!()
//     }
//     pub fn iter(&self) -> I {
//         self.it.clone()
//     }
// }
// impl<N: PlotNum, I: Iterator> PlotIt1D for ClonedPlot1d<N, I>
// where
//     I::Item: Unwrapper<Item = N>,
// {
//     type Item = N;
//     type It = UnwrapperIter<I>;

//     fn unpack(self) -> ([Option<Self::Item>; 2], Self::It) {
//         (self.bound, UnwrapperIter(self.it))
//     }
// }

// pub struct BufferedPlot1dRef<'a, N> {
//     bound: &'a [Option<N>; 2],
//     it: &'a [N],
// }

// impl<'a, N: PlotNum> PlotIt1D for BufferedPlot1dRef<'a, N> {
//     type Item = N;
//     type It = std::iter::Copied<std::slice::Iter<'a, N>>;

//     fn unpack(self) -> ([Option<Self::Item>; 2], Self::It) {
//         (*self.bound, self.it.iter().copied())
//     }
// }

// pub struct BufferedPlot1D<N> {
//     bound: [Option<N>; 2],
//     vec: Vec<N>,
// }
// impl<N: PlotNum> BufferedPlot1D<N> {
//     pub fn new<I: Iterator>(it: I) -> Self
//     where
//         I::Item: Unwrapper<Item = N>,
//     {
//         todo!()
//     }
//     pub fn iter(&self) -> std::slice::Iter<N> {
//         self.vec.iter()
//     }
//     pub fn bound(&self) -> BufferedPlot1dRef<N> {
//         todo!()
//     }
// }

// impl<N: PlotNum> PlotIt1D for BufferedPlot1D<N> {
//     type Item = N;
//     type It = std::vec::IntoIter<N>;

//     fn unpack(self) -> ([Option<Self::Item>; 2], Self::It) {
//         (self.bound, self.vec.into_iter())
//     }
// }

#[derive(Copy, Clone)]
pub struct ClonedPlotIt<I>(pub I);

impl<X: PlotNum, Y: PlotNum, I: Iterator + Clone> PlotIt for ClonedPlotIt<I>
where
    I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
{
    type X = X;
    type Y = Y;
    type It = build::unwrapper::UnwrapperIter<I>;

    fn unpack(self, area: &mut Area<Self::X, Self::Y>) -> Self::It {
        let it = self.0;
        for k in it.clone() {
            let (x, y) = k.unwrap();
            area.grow(Some(&x), Some(&y));
        }
        build::unwrapper::UnwrapperIter(it)
    }
}

// pub fn zip<IX: PlotIt1D, IY: PlotIt1D>(a: IX, b: IY) -> Zip<IX, IY> {
//     Zip(a, b)
// }
// pub struct Zip<IX: PlotIt1D, IY: PlotIt1D>(IX, IY);

// impl<IX: PlotIt1D, IY: PlotIt1D> PlotIt for Zip<IX, IY> {
//     type X = IX::Item;
//     type Y = IY::Item;
//     type It = std::iter::Zip<IX::It, IY::It>;

//     fn unpack(self, area: &mut Area<Self::X, Self::Y>) -> Self::It {
//         let (xarea, itx) = self.0.unpack();
//         let (yarea, ity) = self.1.unpack();

//         area.grow(xarea[0].as_ref(), None);
//         area.grow(xarea[1].as_ref(), None);
//         area.grow(None, yarea[0].as_ref());
//         area.grow(None, yarea[1].as_ref());

//         itx.zip(ity)
//     }
// }

// pub fn clonedbuffer<I: Iterator, X: PlotNum, Y: PlotNum, F: FnMut(&X) -> Option<Y>>(
//     it: I,
//     func: F,
// ) -> ClonedBuffer<I, F>
// where
//     I::Item: Unwrapper<Item = X>,
// {
//     ClonedBuffer { it, func }
// }

// pub struct ClonedBuffer<I, F> {
//     pub it: I,
//     pub func: F,
// }

// impl<X: PlotNum, Y: PlotNum, I: Iterator + Clone, F: FnMut(&X) -> Option<Y>> PlotIt
//     for ClonedBuffer<I, F>
// where
//     I::Item: build::unwrapper::Unwrapper<Item = X>,
// {
//     type X = X;
//     type Y = Y;
//     type It = std::iter::Zip<UnwrapperIter<I>, std::vec::IntoIter<Y>>;

//     fn unpack(mut self, area: &mut Area<Self::X, Self::Y>) -> Self::It {
//         let it = self.it;

//         let mut ys = Vec::new();
//         for j in it.clone() {
//             let j = j.unwrap();

//             let Some(y)=(self.func)(&j) else{
//                 break;
//             };
//             area.grow(Some(&j), Some(&y));

//             ys.push(y)
//         }

//         UnwrapperIter(it).zip(ys.into_iter())
//     }
// }

impl<X: PlotNum, Y: PlotNum, I: IntoIterator> PlotIt for I
where
    I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
{
    type X = X;
    type Y = Y;
    type It = std::vec::IntoIter<(X, Y)>;

    fn unpack(self, area: &mut Area<Self::X, Self::Y>) -> Self::It {
        let it = self.into_iter();

        let vec: Vec<_> = it.map(|j| j.unwrap()).collect();

        for (x, y) in vec.iter() {
            area.grow(Some(x), Some(y));
        }

        vec.into_iter()
    }
}

// pub struct PointBuilder<D: Display> {
//     label: D,
//     typ: PlotMetaType,
// }

// impl<D: Display> PointBuilder<D> {
//     pub fn data<II: PlotIt>(self, it: II) -> SinglePlot<II::X, II::Y, II::It, D> {
//         let mut area = Area::new();
//         let it = it.unpack(&mut area);
//         SinglePlot::new(self.typ, self.label, it, area)
//     }

//     pub fn cloned<X: PlotNum, Y: PlotNum, I: Iterator>(
//         self,
//         it: I,
//     ) -> SinglePlot<X, Y, build::unwrapper::UnwrapperIter<I>, D>
//     where
//         I: Clone,
//         I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
//     {
//         self.data(ClonedPlotIt(it))
//     }

//     pub fn buffered<X: PlotNum, Y: PlotNum, I: Iterator>(
//         self,
//         it: I,
//     ) -> SinglePlot<X, Y, std::vec::IntoIter<(X, Y)>, D>
//     where
//         I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
//     {
//         self.data(it)
//     }
// }

pub struct SinglePlotBuilder<D> {
    label: D,
}

impl<D: Display> SinglePlotBuilder<D> {
    // #[deprecated]
    // pub fn line(self) -> PointBuilder<D> {
    //     PointBuilder {
    //         label: self.label,
    //         typ: PlotMetaType::Plot(PlotType::Line),
    //     }
    // }

    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.  
    pub fn line<P: PlotIt>(self, it: P) -> SinglePlot<P::X, P::Y, P::It, D> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);
        SinglePlot::new(PlotMetaType::Plot(PlotType::Line), self.label, it, area)
    }

    pub(crate) fn bars2<P: PlotIt>(self, it: P) -> SinglePlot<P::X, P::Y, P::It, D> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);
        SinglePlot::new(PlotMetaType::Plot(PlotType::Bars), self.label, it, area)
    }

    // #[deprecated]
    // pub fn scatter(self) -> PointBuilder<D> {
    //     PointBuilder {
    //         label: self.label,
    //         typ: PlotMetaType::Plot(PlotType::Scatter),
    //     }
    // }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.

    pub fn scatter<P: PlotIt>(self, it: P) -> SinglePlot<P::X, P::Y, P::It, D> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);
        SinglePlot::new(PlotMetaType::Plot(PlotType::Scatter), self.label, it, area)
    }

    // #[deprecated]
    // pub fn histogram(self) -> PointBuilder<D> {
    //     PointBuilder {
    //         label: self.label,
    //         typ: PlotMetaType::Plot(PlotType::Histo),
    //     }
    // }
    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.

    pub fn histogram<P: PlotIt>(self, it: P) -> SinglePlot<P::X, P::Y, P::It, D> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);
        SinglePlot::new(PlotMetaType::Plot(PlotType::Histo), self.label, it, area)
    }

    // #[deprecated]
    // pub fn line_fill(self) -> PointBuilder<D> {
    //     PointBuilder {
    //         label: self.label,
    //         typ: PlotMetaType::Plot(PlotType::LineFill),
    //     }
    // }
    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.

    pub fn line_fill<P: PlotIt>(self, it: P) -> SinglePlot<P::X, P::Y, P::It, D> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);
        SinglePlot::new(PlotMetaType::Plot(PlotType::LineFill), self.label, it, area)
    }

    // #[deprecated]
    // pub fn line_fill_raw(self) -> PointBuilder<D> {
    //     PointBuilder {
    //         label: self.label,
    //         typ: PlotMetaType::Plot(PlotType::LineFillRaw),
    //     }
    // }
    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.

    pub fn line_fill_raw<P: PlotIt>(self, it: P) -> SinglePlot<P::X, P::Y, P::It, D> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);
        SinglePlot::new(
            PlotMetaType::Plot(PlotType::LineFillRaw),
            self.label,
            it,
            area,
        )
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
pub fn plot<D: Display>(label: D) -> SinglePlotBuilder<D> {
    SinglePlotBuilder { label }
}
