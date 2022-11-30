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
pub mod plotit;

use plotit::*;
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

pub trait PlotIterator {
    type L: Point;
    type P: Iterator<Item = PlotTag<Self::L>>;
    fn unpack(self) -> PlotRes<Self::P, Self::L>;

    fn chain<P: PlotIterator<L = Self::L>>(
        self,
        other: P,
    ) -> PlotRes<std::iter::Chain<Self::P, P::P>, Self::L>
    where
        Self: Sized,
    {
        let PlotRes {
            area: curr_area,
            it: p1,
        } = self.unpack();
        let PlotRes {
            area: other_area,
            it: p,
        } = other.unpack();
        let mut area = curr_area;
        area.grow_area(&other_area);
        PlotRes {
            area,
            it: p1.chain(p),
        }
    }

    fn dyn_box<'a>(self) -> PlotRes<DynIt<'a, Self::L>, Self::L>
    where
        Self::P: 'a,
        Self: Sized,
    {
        let PlotRes { area, it } = self.unpack();
        PlotRes {
            it: Box::new(it),
            area,
        }
    }
}

type DynIt<'a, L> = Box<dyn Iterator<Item = PlotTag<L>> + 'a>;

#[derive(Copy, Clone)]
pub struct PlotRes<I: Iterator<Item = PlotTag<L>>, L: Point> {
    pub(crate) area: Area<L::X, L::Y>,
    pub(crate) it: I,
}

impl<P: Iterator<Item = PlotTag<L>>, L: Point> PlotIterator for PlotRes<P, L> {
    type L = L;
    type P = P;

    fn unpack(self) -> PlotRes<Self::P, Self::L> {
        self
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

#[derive(Clone)]
pub enum PlotTag<L: Point> {
    Start { name: String, typ: PlotMetaType },
    Plot(L),
    Finish(),
}

///
/// Ensure that the origin point is within view.
///
pub fn origin<L: Point>() -> PlotRes<std::iter::Empty<PlotTag<L>>, L>
where
    L::X: HasZero,
    L::Y: HasZero,
{
    markers(Some(L::X::zero()), Some(L::Y::zero()))
}

///
/// Ensure the list of marker values are within view.
///
pub fn markers<XI: IntoIterator<Item = L::X>, YI: IntoIterator<Item = L::Y>, L: Point>(
    x: XI,
    y: YI,
) -> PlotRes<std::iter::Empty<PlotTag<L>>, L> {
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

///
/// A plot iterator that will be cloned to find the min max bounds.
///
pub fn cloned<L: Point, I: IntoIterator>(it: I) -> ClonedPlotIt<I::IntoIter>
where
    I::IntoIter: Clone + FusedIterator,
    I::Item: build::unwrapper::Unwrapper<Item = L>,
{
    ClonedPlotIt::new(it.into_iter())
}



// use std::iter::Map;
// pub struct PlotIterCreator<I:Iterator> where I::Item:Point{
//     it:std::iter::Chain<std::iter::Chain<std::iter::Once<build::PlotTag<I::Item>>, Map<I, fn(I::Item) -> build::PlotTag<I::Item>>>, std::iter::Once<build::PlotTag<I::Item>>>
// }

// impl<I: Iterator<Item = L>, L: Point> PlotIterCreator<I> {
//     fn new(name: String, typ: PlotMetaType, it: I) -> Self {
//         let start=std::iter::once(PlotTag::Start { name, typ});
//         let mid:Map<I,fn(L)->PlotTag<L>>=it.map(PlotTag::Plot);
//         let end = std::iter::once(PlotTag::Finish());
    
//         PlotIterCreator{
//             it:start.chain(mid).chain(end)
//         }
//     }

// }
// impl<I: FusedIterator<Item=L>, L: Point> FusedIterator for PlotIterCreator<I> {
// }

// impl<I: Iterator<Item = L>, L: Point> Iterator for PlotIterCreator<I> {
//     type Item=PlotTag<L>;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.it.next()
//     }
// }








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

impl<I: Iterator<Item = L> + FusedIterator, L: Point> FusedIterator for PlotIterCreator<I> {}
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

impl<I: IntoIterator<Item = P>, P: PlotIterator<L = L>, L: Point> PlotIterator for I {
    type L = L;
    type P = std::iter::Flatten<std::vec::IntoIter<P::P>>;
    fn unpack(self) -> PlotRes<Self::P, Self::L> {
        let (areas, its): (Vec<_>, Vec<_>) = self
            .into_iter()
            .map(|x| {
                let PlotRes { area, it } = x.unpack();
                (area, it)
            })
            .unzip();

        let mut area = Area::new();
        for a in areas {
            area.grow_area(&a);
        }

        let it = its.into_iter().flatten();

        PlotRes { area, it }
    }
}
