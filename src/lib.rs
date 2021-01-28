use svg::node::element::path::Data;
use svg::Document;
use svg::node::element::Path;


struct Wrapper<I:Iterator<Item=[f32;2]>+Clone>(I);

impl<I:Iterator<Item=[f32;2]>+Clone> Hay for Wrapper<I>{
    fn ref_iter<'a>(&'a self)->Box<dyn Iterator<Item=[f32;2]>+'a>{
        Box::new(self.0.clone())
    }
}


trait Hay{
    fn ref_iter<'a>(&'a self)->Box<dyn Iterator<Item=[f32;2]>+'a>;
}


struct Line<'a>{
    name:String,
    plots:Box<dyn Hay+'a>
}

pub struct SPlot<'a>{
    lines:Vec<Line<'a>>
}

impl<'a> SPlot<'a>{
    pub fn line<I:Iterator<Item=[f32;2]>+Clone+'a>(&mut self,name:String,plots:I)
    {
        self.lines.push(Line{name,plots:Box::new(Wrapper(plots))})
    }

    pub fn render(mut self){
        let mut data=Data::new();

        for Line{name,plots} in self.lines.into_iter(){
            let [minx,maxx,miny,maxy]=if let Some(m)=find_bounds(plots.ref_iter()){
                m
            }else{
                return;
            };

            
            let mut it=plots.ref_iter();
            if let Some(k)=it.next(){
                data=data.move_to((k[0],k[1]));
        
                for [x,y] in it{
                    data=data.line_by((x,y));
                }   
            }
            data=data.close();
        }
    }
}


fn main() {
    
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