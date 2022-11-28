//!
//! Tools for assembling plots
//!
//!

use std::iter::FusedIterator;

use super::*;

pub mod bar;
pub mod crop;
pub mod output_zip;
pub mod unwrapper;
use marker::Area;

pub mod marker;

pub mod plot_iter_impl;

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

pub trait IntoPlotIterator {
    type L: Point;
    type P: Iterator<Item = PlotTag<Self::L>>;
    fn into_plot(self) -> PlotRes<Self::P, Self::L>;
}

impl<P: Iterator<Item = PlotTag<L>>, L: Point> IntoPlotIterator for PlotRes<P, L> {
    type L = L;
    type P = P;
    fn into_plot(self) -> PlotRes<Self::P, Self::L> {
        self
    }
}

pub(crate) struct Foop<'a, I> {
    it: &'a mut I,
}
impl<'a, I: Iterator<Item = PlotTag<L>>, L: Point> Foop<'a, I> {
    fn new(it: &'a mut I) -> Option<(Self, String, PlotMetaType)> {
        if let Some(o) = it.next() {
            match o {
                PlotTag::Start { name, typ } => Some((Self { it }, name, typ)),
                PlotTag::Plot(_) => panic!("expected start"),
                PlotTag::Finish() => panic!("expected start"),
            }
        } else {
            None
        }
    }
}

impl<'a, I: Iterator<Item = PlotTag<L>>, L: Point> Iterator for Foop<'a, I> {
    type Item = L;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(o) = self.it.next() {
            match o {
                PlotTag::Start { .. } => panic!("did not expect start"),
                PlotTag::Plot(a) => Some(a),
                PlotTag::Finish() => None,
            }
        } else {
            None
        }
    }
}


pub trait Point {
    type X: PlotNum;
    type Y: PlotNum;
    fn get(&self) -> (&Self::X, &Self::Y);
}
impl<X: PlotNum, Y: PlotNum> Point for (X, Y) {
    type X = X;
    type Y = Y;

    fn get(&self) -> (&Self::X, &Self::Y) {
        (&self.0, &self.1)
    }
}

#[derive(Copy, Clone)]
pub struct PlotRes<I: Iterator<Item = PlotTag<L>>, L: Point> {
    area: Area<L::X, L::Y>,
    it: I,
}
impl<I: Iterator<Item = PlotTag<L>>, L: Point> PlotRes<I, L> {
    pub fn chain<P: IntoPlotIterator<L = L>>(
        self,
        other: P,
    ) -> PlotRes<std::iter::Chain<I, P::P>, L> {
        let other = other.into_plot();
        let mut area = self.area;
        area.grow_area(&other.area);
        PlotRes {
            area,
            it: self.it.chain(other.it),
        }
    }

    pub fn area(&self) -> &Area<L::X, L::Y> {
        &self.area
    }

    pub fn dyn_box<'a>(self) -> PlotRes<Box<dyn Iterator<Item = PlotTag<L>> + 'a>, L>
    where
        I: 'a,
    {
        PlotRes {
            it: Box::new(self.it),
            area: self.area,
        }
    }

    pub(crate) fn next(&mut self) -> Option<(Foop<I>, String, PlotMetaType)> {
        Foop::new(&mut self.it)
    }
}

// pub trait PlotIterator:Iterator<Item=PlotTag<Self::X,Self::Y>>{
//     type X:PlotNum;
//     type Y:PlotNum;
// }
// impl<X:PlotNum,Y,I:Iterator<Item=PlotTag<X,Y>>> PlotIterator for I{
//     type X=X;
//     type Y=Y;
// }

#[derive(Clone)]
pub enum PlotTag<L: Point> {
    Start { name: String, typ: PlotMetaType },
    Plot(L),
    Finish(),
}

///
/// Ensure that the origin point is within view.
///
pub fn origin<L: Point>() -> PlotRes<EmptyPlot<L>, L>
where
    L::X: HasZero,
    L::Y: HasZero,
{
    markers(Some(L::X::zero()), Some(L::Y::zero()))
}

type EmptyPlot<L> = std::iter::Empty<PlotTag<L>>;

///
/// Ensure the list of marker values are within view.
///
pub fn markers<XI: IntoIterator<Item = L::X>, YI: IntoIterator<Item = L::Y>, L: Point>(
    x: XI,
    y: YI,
) -> PlotRes<EmptyPlot<L>, L> {
    let mut area = Area::new();
    for a in x {
        area.grow(Some(&a), None);
    }
    for a in y {
        area.grow(None, Some(&a));
    }

    PlotRes {
        area,
        it: std::iter::empty(),
    }
}

// ///
// /// Create a [`PlotsDyn`](plot_iter_impl::PlotsDyn)
// ///
// pub fn plots_dyn<X,Y,F: Iterator<Item=PlotTag<X,Y>>, I: IntoIterator<Item = F>>(
//     stuff: I,
// ) -> plot_iter_impl::PlotsDyn<F> {
//     plot_iter_impl::PlotsDyn::new(stuff.into_iter().collect::<Vec<_>>())
// }

///
/// Return min max bounds as well as the points of one plot.
///
pub trait PlotIt {
    type L: Point;
    type It: Iterator<Item = Self::L> + FusedIterator;
    fn unpack(self, area: &mut Area<<Self::L as Point>::X, <Self::L as Point>::Y>) -> Self::It;
}

///
/// A plot iterator that will be cloned to find the min max bounds.
///
pub fn cloned<L: Point, I: IntoIterator>(it: I) -> ClonedPlotIt<I::IntoIter>
where
    I::IntoIter: Clone,
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    ClonedPlotIt(it.into_iter())
}

#[derive(Copy, Clone)]
pub struct ClonedPlotIt<I>(I);

impl<L: Point, I: Iterator + FusedIterator + Clone> PlotIt for ClonedPlotIt<I>
where
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    type L = L;
    type It = build::unwrapper::UnwrapperIter<I>;

    fn unpack(self, area: &mut Area<L::X, L::Y>) -> Self::It {
        let it = self.0;
        for k in it.clone() {
            let l = k.unwrap();
            let (x, y) = l.get();
            area.grow(Some(x), Some(y));
        }
        build::unwrapper::UnwrapperIter(it)
    }
}

impl<L: Point, I: IntoIterator> PlotIt for I
where
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    type L = L;
    type It = std::vec::IntoIter<L>;

    fn unpack(self, area: &mut Area<L::X, L::Y>) -> Self::It {
        let it = self.into_iter();

        let vec: Vec<_> = it.map(|j| j.unwrap()).collect();

        for l in vec.iter() {
            let (x, y) = l.get();
            area.grow(Some(x), Some(y));
        }

        vec.into_iter()
    }
}

pub struct SinglePlotBuilder {
    label: String,
}

#[derive(Clone)]
pub struct PlotIterCreator<I: Iterator> {
    start: Option<(PlotMetaType, String)>,
    it: I,
    posted_finish: bool,
}
impl<I: Iterator<Item = L>, L: Point> PlotIterCreator<I> {
    fn new(label: String, typ: PlotMetaType, it: I) -> Self {
        Self {
            start: Some((typ, label)),
            it,
            posted_finish: false,
        }
    }
}

impl<I: Iterator<Item = L> + FusedIterator, L: Point> Iterator for PlotIterCreator<I> {
    type Item = PlotTag<L>;
    fn next(&mut self) -> Option<PlotTag<L>> {
        if let Some((typ, name)) = self.start.take() {
            Some(PlotTag::Start { typ, name })
        } else if let Some(l) = self.it.next() {
            Some(PlotTag::Plot(l))
        } else if !self.posted_finish {
            self.posted_finish = true;
            Some(PlotTag::Finish())
        } else {
            None
        }
    }
}

impl SinglePlotBuilder {
    fn gen<P: PlotIt>(self, it: P, typ: PlotMetaType) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);

        PlotRes {
            area,
            it: PlotIterCreator::new(self.label, typ, it),
        }
    }
    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.  
    ///
    pub fn line<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Line))
    }

    pub(crate) fn bars<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Bars))
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Scatter))
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Histo))
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFill))
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFillRaw))
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    pub fn text<L: Point>(self) -> PlotRes<PlotIterCreator<std::iter::Empty<L>>, L> {
        let area = Area::new();
        PlotRes {
            area,
            it: PlotIterCreator::new(self.label, PlotMetaType::Text, std::iter::empty()),
        }
    }
}

///
/// Start creating one plot.
///
pub fn plot<D: Display>(name: D) -> SinglePlotBuilder {
    //TODO provide falliable version

    let mut label = String::new();
    use std::fmt::Write;
    write!(&mut label, "{}", name).unwrap();
    SinglePlotBuilder { label }
}
