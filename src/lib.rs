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

struct Wrapper<I: Iterator<Item = [f32; 2]>,F:FnOnce(&mut W)->fmt::Result,W:Write>{
    it:I,
    name:Option<F>,
    _p:PhantomData<W>
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
pub struct Plotter<'a,T:Write,N:NameMaker> {
    element:FlatElement<T>,
    names:N,
    plots: Vec<Plot<'a,T>> //TODO
}

impl<'a,T:Write+'a,N:NameMaker<W=T>> Plotter<'a,T,N> {
    /// Create a plotter
    ///
    /// # Example
    ///
    /// ```
    /// let plotter = poloto::Plotter::new("Number of Cows per Year","Year","Cow");
    /// ```
    pub fn new(writer:T,names:N) -> Plotter<'a,T,N> {
        use tagger::prelude::*;
        let root=tagger::root(writer);
        let svg=root.tag_build_flat("svg")
        .set("class","poloto")
        .set("height",render::HEIGHT)
        .set("width",render::WIDTH)
        .set("viewBox",format!("0 0 {} {}",render::WIDTH,render::HEIGHT))
        .set("xmlns","http://www.w3.org/2000/svg")
        .end();

        Plotter {
            element:svg,            
            plots: Vec::new(),
            names
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
    pub fn line<I: IntoIterator<Item = [f32; 2]> + 'a>(&mut self, name: impl FnOnce(&mut T)->fmt::Result+'a, plots: I) {
        
        self.plots.push(Plot {
            plot_type: PlotType::Line,
            plots: Box::new(
                Wrapper{
                it:plots.into_iter(),
                name:Some(name),
                _p:PhantomData
            }),
        })
        
    }

    /*
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
    pub fn line_fill<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: &'a str,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::LineFill,
            name,
            plots: Box::new(Wrapper(Some(plots))),
        })
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
    pub fn scatter<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: &'a str,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Scatter,
            name,
            plots: Box::new(Wrapper(Some(plots))),
        })
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
    pub fn histogram<I: IntoIterator<Item = [f32; 2]> + 'a>(
        &mut self,
        name: &'a str,
        plots: I,
    ) {
        self.plots.push(Plot {
            plot_type: PlotType::Histo,
            name,
            plots: Box::new(Wrapper(Some(plots))),
        })
    }
    */
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
    
    pub fn render_to_element<T:core::fmt::Write>(self,element:tagger::Element<T>)->RenderBuilder<T>{
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
    pub fn render<T:core::fmt::Write>(self)->RenderBuilder<T>{

    }
    */
}


pub struct RenderBuilder<T:Write>{
    inner:T
}

impl<T:std::io::Write> RenderBuilder<tagger::WriterAdaptor<T>>{
    pub fn new_io(write:T)->RenderBuilder<tagger::WriterAdaptor<T>>{
        RenderBuilder{
            inner:tagger::upgrade_writer(write)
        }
    }
}



impl<T:Write> RenderBuilder<T>{
    pub fn new(inner:T)->RenderBuilder<T>{
        RenderBuilder{
            inner
        }
    }
    
    pub fn finish<'a,A,B,C>(self,
        title:A,
        xname:B,
        yname:C,
    )->Plotter<'a,T,impl NameMaker<W=T>> //NameSetter<A,B,C,T>
    where 
    T:'a,
    A:FnOnce(&mut T)->fmt::Result,
    B:FnOnce(&mut T)->fmt::Result,
    C:FnOnce(&mut T)->fmt::Result,
    {
        Plotter::new(self.inner,NameSetter{
            title,
            xname,
            yname,
            _p:PhantomData
        })
    }
}



pub trait NameMaker{
    type W:Write;
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


impl<A,B,C,T> NameMaker for NameSetter<A,B,C,T> where
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

