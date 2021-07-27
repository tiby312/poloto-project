
use crate::AsF64;
use crate::Plottable;

#[derive(Copy,Clone)]
enum Dir{
    Above,
    Below,
    Left,
    Right
}

///
/// Represents one cropping.
/// 
#[derive(Copy,Clone)]
pub struct Crop<I>{
    dir:Dir,
    val:f64,
    inner:I
}
impl<I:Iterator> Iterator for Crop<I> where I::Item:Plottable{
    type Item=[f64;2];
    fn next(&mut self)->Option<[f64;2]>{
        if let Some(g)=self.inner.next(){
            let [x,y]=g.make_plot();
            Some(match self.dir{
                Dir::Above=>{
                    if y>self.val{
                        [x,f64::NAN]
                    }else{
                        [x,y]
                    }
                },
                Dir::Below=>{
                    if y<self.val{
                        [x,f64::NAN]
                    }else{
                        [x,y]
                    }
                },
                Dir::Left=>{
                    if x>self.val{
                        [f64::NAN,y]
                    }else{
                        [x,y]
                    }
                },
                Dir::Right=>{
                    if x<self.val{
                        [f64::NAN,y]
                    }else{
                        [x,y]
                    }
                }
            })
        }else{
            None
        }
    }
}

///
/// Use this to crop.
/// 
/// 
/// 
pub trait Croppable:Sized{
    fn crop_above<K:AsF64>(self,val:K)->Crop<Self>{
        Crop{dir:Dir::Above,val:val.as_f64(),inner:self}
    }
    fn crop_below<K:AsF64>(self,val:K)->Crop<Self>{
        Crop{dir:Dir::Below,val:val.as_f64(),inner:self}
    }
    fn crop_left<K:AsF64>(self,val:K)->Crop<Self>{
        Crop{dir:Dir::Left,val:val.as_f64(),inner:self}
    }
    fn crop_right<K:AsF64>(self,val:K)->Crop<Self>{
        Crop{dir:Dir::Right,val:val.as_f64(),inner:self}
    }
}

impl<T:Iterator> Croppable for T where T::Item:Plottable{}