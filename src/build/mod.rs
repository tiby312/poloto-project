//!
//! Tools for assembling plots
//!
//!

use std::iter::{Fuse, FusedIterator};

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

///
/// Display label helper for chaining.
///
#[derive(Copy, Clone)]
pub enum ChainDisplay<A, B> {
    A(A),
    B(B),
}
impl<A: Display, B: Display> fmt::Display for ChainDisplay<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ChainDisplay::A(a) => write!(f, "{}", a),
            ChainDisplay::B(b) => write!(f, "{}", b),
        }
    }
}

///
/// Chain two iterators that produce plot tags.
///
#[derive(Copy, Clone)]
pub struct Chain<A, B> {
    a: A,
    b: B,
}

impl<L: Point, D1: Display, D2: Display, A, B> FusedIterator for Chain<A, B>
where
    A: FusedIterator<Item = PlotTag<L, D1>>,
    B: FusedIterator<Item = PlotTag<L, D2>>,
{
}

impl<L: Point, D1: Display, D2: Display, A, B> Iterator for Chain<A, B>
where
    A: FusedIterator<Item = PlotTag<L, D1>>,
    B: FusedIterator<Item = PlotTag<L, D2>>,
{
    type Item = PlotTag<L, ChainDisplay<D1, D2>>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a,b)=self.a.size_hint();
        let (c,d)=self.b.size_hint();

        let k=match(b,d){
            (Some(a),Some(b))=>Some(a+b),
            (Some(a),_)=>Some(a),
            (_,Some(b))=>Some(b),
            (_,_)=>None
        };
        (a+c,k)
    }
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(a) = self.a.next() {
            Some(match a {
                PlotTag::Start {
                    name,
                    typ,
                    size_hint,
                } => PlotTag::Start {
                    name: ChainDisplay::A(name),
                    typ,
                    size_hint,
                },
                PlotTag::Plot(p) => PlotTag::Plot(p),
                PlotTag::Finish() => PlotTag::Finish(),
            })
        } else {
            if let Some(a) = self.b.next() {
                Some(match a {
                    PlotTag::Start {
                        name,
                        typ,
                        size_hint,
                    } => PlotTag::Start {
                        name: ChainDisplay::B(name),
                        typ,
                        size_hint,
                    },
                    PlotTag::Plot(p) => PlotTag::Plot(p),
                    PlotTag::Finish() => PlotTag::Finish(),
                })
            } else {
                None
            }
        }
    }
}



// pub fn chain<L:Point,A:PlotIterator<L=L>,B:PlotIterator<L=L>>(a:A,b:B)->PlotRes<impl Iterator<Item=PlotTag<L,ChainDisplay<A::D,B::D>>>,L>{
//     let PlotRes {
//         area: curr_area,
//         it: p1,
//     } = a.unpack();
//     let PlotRes {
//         area: other_area,
//         it: p,
//     } = b.unpack();
//     let mut area = curr_area;
//     area.grow_area(&other_area);

//     let a=p1.map(|a|{
//         match a {
//             PlotTag::Start {
//                 name,
//                 typ,
//                 size_hint,
//             } => PlotTag::Start {
//                 name: ChainDisplay::A(name),
//                 typ,
//                 size_hint,
//             },
//             PlotTag::Plot(p) => PlotTag::Plot(p),
//             PlotTag::Finish() => PlotTag::Finish(),
//         }
//     });
//     let b=p.map(|a|{
//         match a {
//             PlotTag::Start {
//                 name,
//                 typ,
//                 size_hint,
//             } => PlotTag::Start {
//                 name: ChainDisplay::B(name),
//                 typ,
//                 size_hint,
//             },
//             PlotTag::Plot(p) => PlotTag::Plot(p),
//             PlotTag::Finish() => PlotTag::Finish(),
//         }
//     });
    
//     PlotRes {
//         area,
//         it: a.chain(b),
//     }
// }

pub trait PlotIterator {
    type L: Point;
    type P: Iterator<Item = PlotTag<Self::L, Self::D>>;
    type D: Display;
    fn unpack(self) -> PlotRes<Self::P, Self::L>;

    fn chain<P: PlotIterator<L = Self::L>>(
        self,
        other: P,
    ) -> PlotRes<Chain<Fuse<Self::P>, Fuse<P::P>>, Self::L>
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
            it: Chain {
                a: p1.fuse(),
                b: p.fuse(),
            },
        }
    }

    fn dyn_box<'a>(self) -> PlotRes<DynIt<'a, Self::L, Self::D>, Self::L>
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

type DynIt<'a, L, D> = Box<dyn Iterator<Item = PlotTag<L, D>> + 'a>;

#[derive(Copy, Clone)]
pub struct PlotRes<I: Iterator, L: Point> {
    pub(crate) area: Area<L::X, L::Y>,
    pub(crate) it: I,
}

impl<P: Iterator<Item = PlotTag<L, D>>, L: Point, D: Display> PlotIterator for PlotRes<P, L> {
    type L = L;
    type P = P;
    type D = D;

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
pub enum PlotTag<L: Point, D: Display> {
    Start {
        name: D,
        typ: PlotMetaType,
        size_hint: (usize, Option<usize>),
    },
    Plot(L),
    Finish(),
}

///
/// Ensure that the origin point is within view.
///
pub fn origin<L: Point>() -> PlotRes<std::iter::Empty<PlotTag<L, &'static str>>, L>
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
) -> PlotRes<std::iter::Empty<PlotTag<L, &'static str>>, L> {
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
    I::IntoIter: Clone,
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

pub struct SinglePlotBuilder<D> {
    label: D,
}

#[derive(Clone)]
pub struct PlotIterCreator<I, D> {
    start: Option<(PlotMetaType, D)>,
    it: Fuse<I>,
    posted_finish: bool,
}
impl<I: Iterator<Item = L>, L: Point, D: Display> PlotIterCreator<I, D> {
    fn new(label: D, typ: PlotMetaType, it: I) -> Self {
        Self {
            start: Some((typ, label)),
            it: it.fuse(),
            posted_finish: false,
        }
    }
}

impl<I: ExactSizeIterator<Item = L>, L: Point, D: Display> ExactSizeIterator
    for PlotIterCreator<I, D>
{
}
impl<I: Iterator<Item = L>, L: Point, D: Display> FusedIterator for PlotIterCreator<I, D> {}
impl<I: Iterator<Item = L>, L: Point, D: Display> Iterator for PlotIterCreator<I, D> {
    type Item = PlotTag<L, D>;
    fn next(&mut self) -> Option<PlotTag<L, D>> {
        if let Some((typ, name)) = self.start.take() {
            Some(PlotTag::Start {
                typ,
                name,
                size_hint: self.size_hint(),
            })
        } else if let Some(l) = self.it.next() {
            Some(PlotTag::Plot(l))
        } else if !self.posted_finish {
            self.posted_finish = true;
            Some(PlotTag::Finish())
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (a, b) = self.it.size_hint();
        (a + 2, b.map(|b| b + 2))
    }
}

impl<D: Display> SinglePlotBuilder<D> {
    fn gen<P: PlotIt>(self, it: P, typ: PlotMetaType) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
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
    pub fn line<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Line))
    }

    pub(crate) fn bars<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Bars))
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Scatter))
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::Histo))
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFill))
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It, D>, P::L> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFillRaw))
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    pub fn text<L: Point>(self) -> PlotRes<PlotIterCreator<std::iter::Empty<L>, D>, L> {
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
pub fn plot<D: Display>(label: D) -> SinglePlotBuilder<D> {
    SinglePlotBuilder { label }
}

impl<I: IntoIterator<Item = P>, P: PlotIterator<L = L>, L: Point> PlotIterator for I {
    type L = L;
    type P = std::iter::Flatten<std::vec::IntoIter<P::P>>;
    type D = P::D;
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
