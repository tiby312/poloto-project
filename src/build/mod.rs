//!
//! Tools for assembling plots
//!
//!

use super::*;

//pub mod bar;
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
    Text
}

pub trait IntoPlotIterator {
    type X:PlotNum;
    type Y:PlotNum;
    type P: Iterator<Item=PlotTag<Self::X,Self::Y>>;
    fn into_plot(self) -> PlotRes<Self::P,Self::X,Self::Y>;
}

impl<P: Iterator<Item=PlotTag<X,Y>>,X:PlotNum,Y:PlotNum> IntoPlotIterator for PlotRes<P,X,Y> {
    type X=X;
    type Y=Y;
    type P = P;
    fn into_plot(self) -> PlotRes<Self::P,Self::X,Self::Y> {
        self
    }
}



pub struct PlotRes<I:Iterator<Item=PlotTag<X,Y>>,X,Y>{
    area:Area<X,Y>,
    it:I
}
impl<I:Iterator<Item=PlotTag<X,Y>>,X:PlotNum,Y:PlotNum> PlotRes<I,X,Y>{
    pub fn chain<P:Iterator<Item=PlotTag<X,Y>>>(self,other:PlotRes<P,X,Y>)->PlotRes<std::iter::Chain<I,P>,X,Y>{
        let mut area=self.area;
        self.area.grow_area(&other.area);
        PlotRes { area , it: self.it.chain(other.it) }
    }

    fn bounds(&self)->&Area<X,Y>{
        &self.area
    }


    pub(crate) fn iter(&mut self)->impl Iterator<Item=PlotTag<X,Y>>+'_{
        (&mut self.it).take_while(|w|{
            match w{
                PlotTag::Finish()=>false,
                _=>true
            }
        })
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


pub enum PlotTag<X,Y>{
    Start{
        name:String,
        typ:PlotMetaType
    },
    Plot(X,Y),
    Finish()
}








///
/// Ensure that the origin point is within view.
///
pub fn origin<X: HasZero + PlotNum, Y: HasZero + PlotNum>(
) -> PlotRes<std::iter::Empty<PlotTag<X,Y>>,X,Y> {
    markers(Some(X::zero()), Some(Y::zero()))
}

///
/// Ensure the list of marker values are within view.
///
pub fn markers<XI: IntoIterator, YI: IntoIterator>(
    x: XI,
    y: YI,
) -> PlotRes<std::iter::Empty<PlotTag<XI::Item,YI::Item>>,XI::Item,YI::Item>
where
    XI::Item: PlotNum,
    YI::Item: PlotNum,
{
    let mut area= Area::new();
    for a in x {
        area.grow(Some(&a), None);
    }
    for a in y {
        area.grow(None, Some(&a));
    }

    PlotRes{
        area,
        it:std::iter::empty()
    }
}

///
/// Create a [`PlotsDyn`](plot_iter_impl::PlotsDyn)
///
pub fn plots_dyn<X,Y,F: Iterator<Item=PlotTag<X,Y>>, I: IntoIterator<Item = F>>(
    stuff: I,
) -> plot_iter_impl::PlotsDyn<F> {
    plot_iter_impl::PlotsDyn::new(stuff.into_iter().collect::<Vec<_>>())
}

// pub struct BoxedPlot<'a, X, Y> {
//     inner: Box<dyn PlotIterator<X = X, Y = Y> + 'a>,
// }

// impl<'a, X, Y> BoxedPlot<'a, X, Y> {
//     pub fn new<A: PlotIterator<X = X, Y = Y> + 'a>(a: A) -> BoxedPlot<'a, X, Y> {
//         BoxedPlot { inner: Box::new(a) }
//     }
// }

// // impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> IntoPlotIterator for BoxedPlot<'a, X, Y> {
// //     type P = Self;
// //     fn into_plot(self) -> Self {
// //         self
// //     }
// // }
// impl<'a, X: PlotNum + 'a, Y: PlotNum + 'a> PlotIterator for BoxedPlot<'a, X, Y> {
//     type X = X;
//     type Y = Y;
//     fn increase_area(&mut self, area: &mut Area<X, Y>) {
//         self.inner.as_mut().increase_area(area);
//     }
//     fn next_plot_point(&mut self) -> PlotResult<(X, Y)> {
//         self.inner.as_mut().next_plot_point()
//     }

//     fn next_name(&mut self, w: &mut dyn fmt::Write) -> Option<fmt::Result> {
//         self.inner.as_mut().next_name(w)
//     }

//     fn next_typ(&mut self) -> Option<PlotMetaType> {
//         self.inner.as_mut().next_typ()
//     }
// }

///
/// Return min max bounds as well as the points of one plot.
///
pub trait PlotIt {
    type X: PlotNum;
    type Y: PlotNum;
    type It: Iterator<Item = (Self::X, Self::Y)>;
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


pub struct PlotIterCreator<I:Iterator>{
    start:Option<(PlotMetaType,String)>,
    it:I
}
impl<I:Iterator<Item=(X,Y)>,X,Y> PlotIterCreator<I>{
    fn new(label:String,typ:PlotMetaType,it:I)->Self{
        Self { start: Some((typ,label)), it }
    }
}


impl<I:Iterator<Item=(X,Y)>,X,Y> Iterator for PlotIterCreator<I>{
    type Item=PlotTag<X,Y>;
    fn next(&mut self)->Option<PlotTag<X,Y>>{
        if let Some((typ,name))=self.start.take(){
            Some(
                PlotTag::Start{
                    typ,
                    name
                }
            )
        }else{
            if let Some((x,y))=self.it.next(){
                Some(PlotTag::Plot(x,y))
            }else{
                Some(
                    PlotTag::Finish()
                )
            }
        }

    }
}

impl SinglePlotBuilder {
    
    fn gen<P: PlotIt>(self, it: P,typ:PlotMetaType) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        let mut area = Area::new();
        let it = it.unpack(&mut area);


        PlotRes{
            area,
            it:PlotIterCreator::new(self.label,typ,it)
        }
    }
    /// Create a line from plots using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.  
    /// 
    pub fn line<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Plot(PlotType::Line))
    }
    

    pub(crate) fn bars<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Plot(PlotType::Bars))
    }
    
    /// Create a scatter plot from plots, using a SVG path with lines with zero length.
    /// Each point can be sized using the stroke width.
    /// The path belongs to the CSS classes `poloto_scatter` and `.poloto[N]stroke` css class
    /// with the latter class overriding the former.
    pub fn scatter<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Plot(PlotType::Scatter))
    }
    
    /// Create a histogram from plots using SVG rect elements.
    /// Each bar's left side will line up with a point.
    /// Each rect element belongs to the `.poloto[N]fill` css class.
    pub fn histogram<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Plot(PlotType::Histo))
    }
    
    
    /// Create a line from plots that will be filled underneath using a SVG path element.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Plot(PlotType::LineFill))
    }
    

    /// Create a line from plots that will be filled using a SVG path element.
    /// The first and last points will be connected and then filled in.
    /// The path element belongs to the `.poloto[N]fill` css class.
    pub fn line_fill_raw<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Plot(PlotType::LineFillRaw))
    }
    

    ///
    /// Write some text in the legend. This doesnt increment the plot number.
    ///
    pub fn text<P: PlotIt>(self,it: P) -> PlotRes<PlotIterCreator<P::It>,P::X,P::Y> {
        self.gen(it,PlotMetaType::Text)
    }
    
}

///
/// Start creating one plot.
///
pub fn plot<D: Display>(label: D) -> SinglePlotBuilder {

    //TODO provide falliable version
    
    let mut label=String::new();
    use std::fmt::Write;
    write!(&mut label,"{}",label).unwrap();
    SinglePlotBuilder { label }
}
