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
    type X: PlotNum;
    type Y: PlotNum;
    type P: Iterator<Item = PlotTag<Self::X, Self::Y>>;
    fn into_plot(self) -> PlotRes<Self::P, Self::X, Self::Y>;
}

impl<P: Iterator<Item = PlotTag<X, Y>>, X: PlotNum, Y: PlotNum> IntoPlotIterator
    for PlotRes<P, X, Y>
{
    type X = X;
    type Y = Y;
    type P = P;
    fn into_plot(self) -> PlotRes<Self::P, Self::X, Self::Y> {
        self
    }
}

pub struct Foop<'a, I> {
    it: &'a mut I,
}
impl<'a, I: Iterator<Item = PlotTag<X, Y>>, X, Y> Foop<'a, I> {
    fn new(it: &'a mut I) -> Option<(Self, String, PlotMetaType)> {
        if let Some(o) = it.next() {
            match o {
                PlotTag::Start { name, typ } => Some((Self { it }, name, typ)),
                PlotTag::Plot(_, _) => panic!("expected start"),
                PlotTag::Finish() => panic!("expected start"),
            }
        } else {
            None
        }
    }
}

impl<'a, I: Iterator<Item = PlotTag<X, Y>>, X, Y> Iterator for Foop<'a, I> {
    type Item = (X, Y);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(o) = self.it.next() {
            match o {
                PlotTag::Start { .. } => panic!("did not expect start"),
                PlotTag::Plot(x, y) => Some((x, y)),
                PlotTag::Finish() => None,
            }
        } else {
            None
        }
    }
}

#[derive(Copy, Clone)]
pub struct PlotRes<I: Iterator<Item = PlotTag<X, Y>>, X, Y> {
    area: Area<X, Y>,
    it: I,
}
impl<I: Iterator<Item = PlotTag<X, Y>>, X: PlotNum, Y: PlotNum> PlotRes<I, X, Y> {
    pub fn chain<P: IntoPlotIterator<X = X, Y = Y>>(
        self,
        other: P,
    ) -> PlotRes<std::iter::Chain<I, P::P>, X, Y> {
        let other = other.into_plot();
        let mut area = self.area;
        area.grow_area(&other.area);
        PlotRes {
            area,
            it: self.it.chain(other.it),
        }
    }

    pub fn area(&self) -> &Area<X, Y> {
        &self.area
    }

    pub fn dyn_box<'a>(self) -> PlotRes<Box<dyn Iterator<Item = PlotTag<X, Y>> + 'a>, X, Y>
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
pub enum PlotTag<X, Y> {
    Start { name: String, typ: PlotMetaType },
    Plot(X, Y),
    Finish(),
}

///
/// Ensure that the origin point is within view.
///
pub fn origin<X: HasZero + PlotNum, Y: HasZero + PlotNum>(
) -> PlotRes<std::iter::Empty<PlotTag<X, Y>>, X, Y> {
    markers(Some(X::zero()), Some(Y::zero()))
}

///
/// Ensure the list of marker values are within view.
///
pub fn markers<XI: IntoIterator, YI: IntoIterator>(
    x: XI,
    y: YI,
) -> PlotRes<std::iter::Empty<PlotTag<XI::Item, YI::Item>>, XI::Item, YI::Item>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
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
    type X: PlotNum;
    type Y: PlotNum;
    type It: Iterator<Item = (Self::X, Self::Y)> + FusedIterator;
    fn unpack(self, area: &mut Area<Self::X, Self::Y>) -> Self::It;
}

///
/// A plot iterator that will be cloned to find the min max bounds.
///
pub fn cloned<X: PlotNum, Y: PlotNum, I: IntoIterator>(it: I) -> ClonedPlotIt<I::IntoIter>
where
    I::IntoIter: Clone,
    I::Item: build::unwrapper::Unwrapper<Item = (X, Y)>,
{
    ClonedPlotIt(it.into_iter())
}

#[derive(Copy, Clone)]
pub struct ClonedPlotIt<I>(I);

impl<X: PlotNum, Y: PlotNum, I: Iterator + FusedIterator + Clone> PlotIt for ClonedPlotIt<I>
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

pub struct SinglePlotBuilder {
    label: String,
}

#[derive(Clone)]
pub struct PlotIterCreator<I: Iterator> {
    start: Option<(PlotMetaType, String)>,
    it: I,
    posted_finish: bool,
}
impl<I: Iterator<Item = (X, Y)>, X, Y> PlotIterCreator<I> {
    fn new(label: String, typ: PlotMetaType, it: I) -> Self {
        Self {
            start: Some((typ, label)),
            it,
            posted_finish: false,
        }
    }
}

impl<I: Iterator<Item = (X, Y)> + FusedIterator, X, Y> Iterator for PlotIterCreator<I> {
    type Item = PlotTag<X, Y>;
    fn next(&mut self) -> Option<PlotTag<X, Y>> {
        if let Some((typ, name)) = self.start.take() {
            Some(PlotTag::Start { typ, name })
        } else {
            if let Some((x, y)) = self.it.next() {
                Some(PlotTag::Plot(x, y))
            } else {
                if !self.posted_finish {
                    self.posted_finish = true;
                    Some(PlotTag::Finish())
                } else {
                    None
                }
            }
        }
    }
}

impl SinglePlotBuilder {
    fn gen<P: PlotIt>(
        self,
        it: P,
        typ: PlotMetaType,
    ) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
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
    pub fn line<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
        self.gen(it, PlotMetaType::Plot(PlotType::Line))
    }

    pub(crate) fn bars<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
        self.gen(it, PlotMetaType::Plot(PlotType::Bars))
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
        self.gen(it, PlotMetaType::Plot(PlotType::Scatter))
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
        self.gen(it, PlotMetaType::Plot(PlotType::Histo))
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFill))
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::X, P::Y> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFillRaw))
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    pub fn text<X: PlotNum, Y: PlotNum>(
        self,
    ) -> PlotRes<PlotIterCreator<std::iter::Empty<(X, Y)>>, X, Y> {
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
