use svg::node::element::path::Data;
use svg::node::element::Polyline;
use svg::Document;
use svg::node::element::Path;
use core::marker::PhantomData;

struct Wrapper<'a,I:Iterator<Item=[f32;2]>+Clone+'a>(Option<I>,PhantomData<&'a I>);

impl<'a,I:Iterator<Item=[f32;2]>+Clone+'a> PlotTrait<'a> for Wrapper<'a,I>{
    #[inline(always)]
    fn ref_iter(&self)->Box<dyn Iterator<Item=[f32;2]>+'a>{
        Box::new(self.0.as_ref().unwrap().clone())
    }

    #[inline(always)]
    fn into_iter(&mut self)->Box<dyn Iterator<Item=[f32;2]>+'a>{
        Box::new(self.0.take().unwrap())
    }
}


trait PlotTrait<'a>{
    fn ref_iter(&self)->Box<dyn Iterator<Item=[f32;2]>+'a>;
    fn into_iter(&mut self)->Box<dyn Iterator<Item=[f32;2]>+'a>;
}

/*
enum Plot<'a>{
    Lines{name:String,plots:Box<dyn PlotTrait<'a>+'a>},
}
*/
struct Plot<'a>{
    name:String,
    plots:Box<dyn PlotTrait<'a>+'a>,
    //type? line/histo/scatter
}

pub struct Splot<'a>{
    plots:Vec<Plot<'a>>,
}


pub struct Color{
    pub back:u32,
    pub fore:u32,
    plots:[u32;10]
}

pub const DEFAULT_COLOR:Color=Color{
    back:0,
    fore:0,
    plots:[0;10]
};

impl<'a> Splot<'a>{
    pub fn new()->Splot<'a>{
        Splot{plots:Vec::new()}
    }
    ///iterator will be iterated through twice by doing one call to clone().
    ///once to find min max bounds, second to construct plot
    pub fn lines<I:Iterator<Item=[f32;2]>+Clone+'a>(&mut self,name:impl ToString,plots:I)
    {
        self.plots.push(Plot{name:name.to_string(),plots:Box::new(Wrapper(Some(plots),PhantomData))})
    }

    pub fn render(mut self){

        let mut document = Document::new()
        .set("width",800)
        .set("height",600)
        .set("viewBox", (0,0, 800, 600));
        
        let [minx,maxx,miny,maxy]=if let Some(m)=find_bounds(self.plots.iter().flat_map(|a|a.plots.ref_iter())){
            m
        }else{
            return;
        };

        
        let scalex=(maxx-minx)/800.0;
        let scaley=(maxy-miny)/600.0;

        dbg!(minx,maxx,miny,maxy,scalex,scaley);
        
        
        for Plot{name,mut plots} in self.plots.into_iter(){
            
            let mut data=Polyline::new().set("fill","blue").set("stroke","#0074d9").set("stroke-width",3);
            
            let mut it=plots.into_iter();

            use std::fmt::Write;
            let mut points=String::new();
            if let Some([x,y])=it.next(){
                for [x,y] in it{
                    write!(&mut points,"{},{}\n",x*scalex,600.0-y*scaley);
                }   
            }
            //dbg!(&points);
            data=data.set("points",points);
            //data=data.close();

            /*
            let path = Path::new()
            .set("fill", "blue")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);
*/
            document=document.add(data);

        
            
        }
        svg::save("image.svg", &document).unwrap();
    
    }
}


fn main() {
    
    let mut s=Splot::new();
    s.lines("yo", (0..800).map(|x|[x as f32,x as f32/2.0]) );
    s.render();
    /*
    let data = Data::new()
        .move_to((10, 10))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();
    
    let path = Path::new()
        .set("fill", "blue")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);
    
    let data=svg::node::Text::new("hay");
    let k=svg::node::element::Text::new().add(data).set("x","40").set("y","40"); 

    let document = Document::new()
        .set("viewBox", (-10, -10, 90, 90))
        .add(path)
        .add(k);
    
    svg::save("image.svg", &document).unwrap();
    */
}



fn find_bounds(mut it:impl IntoIterator<Item=[f32;2]>)->Option<[f32;4]>{
    let mut ii=it.into_iter();
    if let Some([x,y])=ii.next(){
        let mut val=[x,x,y,y];
        ii.fold(&mut val,|val,[x,y]|{
            if x<val[0]{
                val[0]=x;
            }else if x>val[1]{
                val[1]=x;
            }
            if y<val[2]{
                val[2]=y;
            }else if y>val[3]{
                val[3]=y;
            }
            val
        });
        Some(val)
    }else{
        None
    }
}