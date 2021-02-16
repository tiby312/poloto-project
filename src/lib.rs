//!
//! poloto - plot to SVG and style with CSS
//!
//!
//! ### How do I change the color of the plots?
//!
//! You can doing it by overriding the css. If you embed the generated svg into a html file,
//! then you can add this example:
//! ```css
//! .poloto{
//!    --poloto_bg_color:"black";
//!    --poloto_fg_color:"white;
//!    --poloto_color0:"red";
//!    --poloto_color1:"green";
//!    --poloto_color2:"yellow";
//!    --poloto_color3:"orange";
//!    --poloto_color4:"purple";
//!    --poloto_color5:"pink";
//!    --poloto_color6:"aqua";
//!    --poloto_color7:"red";
//! }
//! ```  
//! By default these variables are not defined, so the svg falls back on some default colors.
//!
//! ### Can I change the styling of the plots?
//!
//! Yes! You can harness the power of CSS both in the svg, or outside
//! in html with an embeded svg. Some things you can do:
//!
//! * Change the color scheme to fit your html theme.
//! * Highlight one plot, make it dashed, or add hover effect
//! * Animate things using @keyframes
//!
//! Depending on whether you are adding a new style attribute or overriding
//! an existing one, you might have to increase the specificty of your css clause to make sure it overrides
//! the svg css clause.
//! ### Usage
//!
//! * Plots containing NaN or Infinity are ignored.
//! * After 6 plots, the colors cycle back and are repeated.
//!
//! ### Why not scale the intervals to end nicely with the ends of the axis lines?
//!
//! Doing this you would have to either have more dead space, or exclude
//! plots that the user would expect to get plotted. Neither of these sounded
//! better than the option of just having the intervals stop not necessarily
//! at the end of the axis lines.
//!
//! ### Example
//!
//! See the graphs in this report: [broccoli_book](https://tiby312.github.io/broccoli_report/)
//!
use core::marker::PhantomData;

use core::fmt::Write;


mod util;

use core::fmt;
mod render;






//TODO determine variance.
struct Wrapper<I: Iterator<Item = [f32; 2]>,F:FnOnce(&mut W)->fmt::Result,W:Write>{
    it:I,
    name:Option<F>,
    _p:PhantomData<W>
}
impl<'a,
    I: Iterator<Item = [f32; 2]> + 'a,
    F:FnOnce(&mut W)->fmt::Result,
    W:Write
> Wrapper<I,F,W> {
    fn new(it:I,name:F)->Self{
        Wrapper{
            it,
            name:Some(name),
            _p:PhantomData
        }
    }
}

impl<'a,
    I: Iterator<Item = [f32; 2]> + 'a,
    F:FnOnce(&mut W)->fmt::Result,
    W:Write
> PlotTrait<'a> for Wrapper<I,F,W> {
    type W=W;
    #[inline(always)]
    fn get_iter_mut(&mut self) -> &mut (dyn Iterator<Item = [f32; 2]> + 'a) {
        &mut self.it
    }

    fn write_name(&mut self,wr:&mut Self::W)->fmt::Result{
        (self.name.take().unwrap())(wr)
    }
}

trait PlotTrait<'a> {
    type W:Write;
    fn get_iter_mut(&mut self) -> &mut (dyn Iterator<Item = [f32; 2]> + 'a);
    fn write_name(&mut self,wr:&mut Self::W)->fmt::Result;
}

enum PlotType {
    Scatter,
    Line,
    Histo,
    LineFill,
}

struct Plot<'a,W:Write> {
    plot_type: PlotType,
    plots: Box<dyn PlotTrait<'a,W=W> + 'a>,
}

struct PlotDecomp<'a,W:Write> {
    plot_type:PlotType,
    orig: Box<dyn PlotTrait<'a,W=W> + 'a>,
    plots: Vec<[f32; 2]>,
}

use tagger::element_move::FlatElement;

///Keeps track of plots.
///User supplies iterators that will be iterated on when
///render is called.
///BELOW IMPORTANT:::
///We hold the writer for type inference for the _fmt functions
///We don't actually write anything to it until render() is called.
///This way you don't need to handle formatting errors in multipe places
///Only when you call render.
pub struct Plotter<'a,T:Write,N:Labels> {
    element:FlatElement<T>,
    names:N,
    plots: Vec<Plot<'a,T>>,

    //We could written the svg tag inside of the plot() function,
    //but to make that function infalliable, lets do it instead in the
    //render() function defering it using this boolean
    make_svg:bool
}

impl<'a,T:Write+'a,N:Labels<W=T>+'a> Plotter<'a,T,N> {
    /// Create a plotter
    ///
    /// # Example
    ///
    /// ```
    /// let plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// ```
    fn new(element:FlatElement<T>,names:N,make_svg:bool) -> Plotter<'a,T,N> {

        
        Plotter {
            element,            
            plots: Vec::new(),
            names,
            make_svg
        }
    }

    /// Create a line from plots.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.line("cow",data.iter().map(|&x|x))
    /// ```
    pub fn line_fmt<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: impl FnOnce(&mut T)->fmt::Result+'a, plots: I) {
        
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(
                Wrapper::new(plots.into_iter(),name)),
        })
        
    }

    pub fn line<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: &'a str, plots: I) {
        self.line_fmt(move |w|write!(w,"{}",name),plots)
    }

    
    /// Create a line from plots that will be filled underneath.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.line_fill("cow",data.iter().map(|&x|x))
    /// ```
    pub fn line_fill_fmt<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: impl FnOnce(&mut T)->fmt::Result+'a,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            plots: Box::new(Wrapper::new(plots.into_iter(),name)),
        })
    }
    pub fn line_fill<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: &'a str,
        plots: I,
    ) {
        self.line_fill_fmt(move |w|write!(w,"{}",name),plots)
    }
    

    /// Create a scatter plot from plots.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.scatter("cow",data.iter().map(|&x|x))
    /// ```
    pub fn scatter_fmt<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: impl FnOnce(&mut T)->fmt::Result+'a,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            plots: Box::new(Wrapper::new(plots.into_iter(),name)),
        })
    }

    pub fn scatter<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: &'a str,
        plots: I,
    ) {
        self.scatter_fmt(move |w|write!(w,"{}",name),plots)
    }



    

    /// Create a histogram from plots.
    /// Each bar's left side will line up with a point
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// plotter.histogram("cow",data.iter().map(|&x|x))
    /// ```
    pub fn histogram_fmt<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: impl FnOnce(&mut T)->fmt::Result+'a,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            plots: Box::new(Wrapper::new(plots.into_iter(),name)),
        })
    }
    
    pub fn histogram<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: &'a str,
        plots: I,
    ) {
        self.histogram_fmt(move |w|write!(w,"{}",name),plots)
    }
/*
    ///You can override the css in regular html if you embed the generated svg.
    ///This gives you a lot of flexibility giving your the power to dynamically
    ///change the theme of your svg.
    ///
    ///However, if you want to embed the svg as an image, you lose this ability.
    ///If embedding as IMG is desired, instead the user can insert a custom style into the generated svg itself.
    ///
    ///All the plot functions don't actually add anything to the document until a  `render` function is called.
    ///So calls to this will append elements to the start of the document.
    ///
    /// # Example
    ///
    /// ```
    /// let data=[
    ///         [1.0f32,4.0],
    ///         [2.0,5.0],
    ///         [3.0,6.0]
    /// ];
    /// let mut plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// // Make the line purple.
    /// plotter.append(svg::node::Text::new("<style>.poloto{--poloto_color0:purple;}</style>"));
    /// plotter.line("cow",data.iter().map(|&x|x));
    /// ```    
    
    pub fn render_to_element<T:core::fmt::Write>(self,element:tagger::Element<T>)->PlotterBuilder<T>{
        /*
        let root=tagger::root(writer);
        let svg=root.tag_build_flat("svg")
        .set("class","poloto")
        .set("height",render::HEIGHT)
        .set("width",render::WIDTH)
        .set("viewBox",format!("0 0 {} {}",render::WIDTH,render::HEIGHT))
        .set("xmlns","http://www.w3.org/2000/svg")
        .end();
        */
        //render::render(self);
    }
    */

    pub fn render(self)->fmt::Result{
        render::render(self)
    }
    
}


pub struct PlotterBuilder<T:Write>{
    inner:FlatElement<T>,
    make_svg:bool
}


pub fn default_svg<T:Write>(writer:T)->FlatElement<T>{
    use tagger::prelude::*;
    let root=tagger::root(writer);
    let svg=root.tag_build_flat("svg")
    .set("class","poloto")
    .set("height",render::HEIGHT)
    .set("width",render::WIDTH)
    .set("viewBox",format!("0 0 {} {}",render::WIDTH,render::HEIGHT))
    .set("xmlns","http://www.w3.org/2000/svg")
    .end();
    svg
}

pub fn plot_io<T:std::io::Write>(write:T)->PlotterBuilder<tagger::WriterAdaptor<T>>{
    let inner=tagger::root(tagger::upgrade_writer(write));

    PlotterBuilder{
        inner,
        make_svg:true
    }
}
pub fn plot<T:core::fmt::Write>(write:T)->PlotterBuilder<T>{
    let inner=tagger::root(write);

    PlotterBuilder{
        inner,
        make_svg:true
    }
}
pub fn plot_from_element<T:core::fmt::Write>(element:tagger::element_move::FlatElement<T>)->PlotterBuilder<T>{
    let inner=element;

    PlotterBuilder{
        inner,
        make_svg:false
    }
    
}



impl<'a,T:Write+'a> PlotterBuilder<T>{
    
    pub fn finish(self,
        title:&'a str,
        xname:&'a str,
        yname:&'a str)->Plotter<'a,T,impl Labels<W=T>+'a>{
            Plotter::new(self.inner,NameSetter{
                title:move |w:&mut T|write!(w,"{}",title),
                xname:move |w:&mut T|write!(w,"{}",xname),
                yname:move |w:&mut T|write!(w,"{}",yname),
                _p:PhantomData
            },self.make_svg)
    }
    pub fn finish_fmt<A,B,C>(self,
        title:A,
        xname:B,
        yname:C,
    )->Plotter<'a,T,impl Labels<W=T>+'a> //NameSetter<A,B,C,T>
    where 
    T:'a,
    A:FnOnce(&mut T)->fmt::Result+'a,
    B:FnOnce(&mut T)->fmt::Result+'a,
    C:FnOnce(&mut T)->fmt::Result+'a,
    {
        Plotter::new(self.inner,NameSetter{
            title,
            xname,
            yname,
            _p:PhantomData
        },self.make_svg)
    }
}

///A trait used internally to group functions that
///generate the title, xaxis name, and yaxis name.
pub trait Labels{
    type W:fmt::Write;
    type A:FnOnce(&mut Self::W)->fmt::Result;
    type B:FnOnce(&mut Self::W)->fmt::Result;
    type C:FnOnce(&mut Self::W)->fmt::Result;
    fn split(self)->(Self::A,Self::B,Self::C);
}


struct NameSetter<A,B,C,T>{
    title:A,
    xname:B,
    yname:C,
    _p:PhantomData<T>
}


impl<A,B,C,T> Labels for NameSetter<A,B,C,T> where
    A:FnOnce(&mut T)->fmt::Result,
    B:FnOnce(&mut T)->fmt::Result,
    C:FnOnce(&mut T)->fmt::Result,
    T:Write
{
    type W=T;
    type A=A;
    type B=B;
    type C=C;
    fn split(self)->(A,B,C){
        (self.title,self.xname,self.yname)
    }
}

