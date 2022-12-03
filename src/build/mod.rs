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


#[derive(Copy,Clone)]
pub struct OneFmt<D>(D);

impl<D:Display> Foo for OneFmt<D>{
    fn next(&mut self,a:&mut dyn fmt::Write)->fmt::Result{
        write!(a,"{}",self.0)
    }
}


#[derive(Copy,Clone)]
pub struct NoFmt;
impl Foo for NoFmt{
    fn next(&mut self,_:&mut dyn fmt::Write)->fmt::Result{
        Ok(())
    }
}

pub trait Foo{
    fn next(&mut self,a:&mut dyn fmt::Write)->fmt::Result;
}
impl<I:Iterator<Item=D>,D:Foo> Foo for I{
    fn next(&mut self,w:&mut dyn fmt::Write)->fmt::Result{
        if let Some(mut a)=self.next(){
            a.next(w)?;
        }

        Ok(())
    }
}


#[derive(Copy,Clone)]
pub struct FooChain<A,B>{
    a:Option<A>,
    b:Option<B>
}

impl<A:Foo,B:Foo> Foo for FooChain<A,B>{
    fn next(&mut self,w:&mut dyn fmt::Write)->fmt::Result{
        if let Some(mut a)=self.a.take(){
            a.next(w)
        }else{
            if let Some(mut a)=self.b.take(){
                a.next(w)
            }else{
                Ok(())
            }
        }
    }
}

pub trait PlotIterator {
    type L: Point;
    type P: Iterator<Item = PlotTag<Self::L>>;
    type F:Foo;
    fn unpack(self) -> PlotRes<Self::P, Self::L,Self::F>;

    fn chain<P: PlotIterator<L = Self::L>>(
        self,
        other: P,
    ) -> PlotRes<std::iter::Chain<Self::P, P::P>, Self::L,FooChain<Self::F,P::F>>
    where
        Self: Sized,
    {
        let PlotRes {
            area: curr_area,
            it: p1,
            fmt:f1
        } = self.unpack();
        let PlotRes {
            area: other_area,
            it: p,
            fmt:f2
        } = other.unpack();
        let mut area = curr_area;
        area.grow_area(&other_area);
        PlotRes {
            area,
            it: p1.chain(p),
            fmt:FooChain{a:Some(f1),b:Some(f2)}
        }
    }

    fn dyn_box<'a>(self) -> PlotRes<DynIt<'a, Self::L>, Self::L,Self::F>
    where
        Self::P: 'a,
        Self: Sized,
    {
        let PlotRes { area, it ,fmt} = self.unpack();
        PlotRes {
            it: Box::new(it),
            area,
            fmt
        }
    }
}

type DynIt<'a, L> = Box<dyn Iterator<Item = PlotTag<L>> + 'a>;

#[derive(Copy, Clone)]
pub struct PlotRes<I: Iterator<Item = PlotTag<L>>, L: Point,F:Foo> {
    pub(crate) fmt:F,
    pub(crate) area: Area<L::X, L::Y>,
    pub(crate) it: I,
}

impl<P: Iterator<Item = PlotTag<L>>, L: Point,F:Foo> PlotIterator for PlotRes<P, L,F> {
    type L = L;
    type P = P;
    type F=F;

    fn unpack(self) -> PlotRes<Self::P, Self::L,F> {
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
    Start {
        typ: PlotMetaType,
        size_hint: (usize, Option<usize>),
    },
    Plot(L),
    Finish(),
}

///
/// Ensure that the origin point is within view.
///
pub fn origin<L: Point>() -> PlotRes<std::iter::Empty<PlotTag<L>>, L,NoFmt>
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
) -> PlotRes<std::iter::Empty<PlotTag<L>>, L,NoFmt> {
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
        fmt:NoFmt
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

pub struct SinglePlotBuilder<D:Display>{
    label: D,
}

#[derive(Clone)]
pub struct PlotIterCreator<I> {
    start: Option<PlotMetaType>,
    it: Fuse<I>,
    posted_finish: bool,
}
impl<I: Iterator<Item = L>, L: Point> PlotIterCreator<I> {
    fn new(typ: PlotMetaType, it: I) -> Self {
        Self {
            start: Some(typ),
            it: it.fuse(),
            posted_finish: false,
        }
    }
}

impl<I: ExactSizeIterator<Item = L>, L: Point> ExactSizeIterator for PlotIterCreator<I> {}
impl<I: Iterator<Item = L>, L: Point> FusedIterator for PlotIterCreator<I> {}
impl<I: Iterator<Item = L>, L: Point> Iterator for PlotIterCreator<I> {
    type Item = PlotTag<L>;
    fn next(&mut self) -> Option<PlotTag<L>> {
        if let Some(typ) = self.start.take() {
            Some(PlotTag::Start {
                typ,
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

impl<D:Display> SinglePlotBuilder<D> {
    fn gen<P: PlotIt>(self, it: P, typ: PlotMetaType) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);

        PlotRes {
            area,
            it: PlotIterCreator::new( typ, it),
            fmt:OneFmt(self.label)
        }
    }
    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.  
    ///
    pub fn line<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        self.gen(it, PlotMetaType::Plot(PlotType::Line))
    }

    pub(crate) fn bars<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        self.gen(it, PlotMetaType::Plot(PlotType::Bars))
    }

    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        self.gen(it, PlotMetaType::Plot(PlotType::Scatter))
    }

    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        self.gen(it, PlotMetaType::Plot(PlotType::Histo))
    }

    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFill))
    }

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw<P: PlotIt>(self, it: P) -> PlotRes<PlotIterCreator<P::It>, P::L,OneFmt<D>> {
        self.gen(it, PlotMetaType::Plot(PlotType::LineFillRaw))
    }

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    pub fn text<L: Point>(self) -> PlotRes<PlotIterCreator<std::iter::Empty<L>>, L,OneFmt<D>> {
        let area = Area::new();
        PlotRes {
            area,
            it: PlotIterCreator::new(PlotMetaType::Text, std::iter::empty()),
            fmt:OneFmt(self.label)
        }
    }
}

///
/// Start creating one plot.
///
pub fn plot<D: Display>(label: D) -> SinglePlotBuilder<D> {
    // //TODO provide falliable version

    // let mut label = String::new();
    // use std::fmt::Write;
    // write!(&mut label, "{}", name).unwrap();
    SinglePlotBuilder { label }
}

impl<I: IntoIterator<Item = P>, P: PlotIterator<L = L>, L: Point> PlotIterator for I {
    type L = L;
    type P = std::iter::Flatten<std::vec::IntoIter<P::P>>;
    type F=std::vec::IntoIter<P::F>;
    fn unpack(self) -> PlotRes<Self::P, Self::L,Self::F> {
        let (areas, its): (Vec<_>, Vec<_>) = self
            .into_iter()
            .map(|x| {
                let PlotRes { area, it,fmt } = x.unpack();
                (area, (it,fmt))
            })
            .unzip();

        let (its,fmt):(Vec<_>,Vec<_>)=its.into_iter().unzip();

        let mut area = Area::new();
        for a in areas {
            area.grow_area(&a);
        }

        let it = its.into_iter().flatten();

        PlotRes { area, it,fmt:fmt.into_iter() }
    }
}
