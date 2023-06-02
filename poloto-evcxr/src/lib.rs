use hypermelon::{elem::Locked, prelude::Elem};




///
/// image/svg+xml cell with plain text
/// 
pub fn evcxr_display_svg<R:Elem+Locked>(elem:R){
    let mut s=String::new();
    hypermelon::render(elem,&mut s).unwrap();
    println!("EVCXR_BEGIN_CONTENT image/svg+xml\n{}\nEVCXR_END_CONTENT", s);
}


///
/// html cell with image/svg+xml mime with base64 encoding
/// 
pub fn evcxr_display<R:Elem+Locked>(elem:R){
    let a=encode(elem);
    let mut s = String::new();
    hypermelon::render(a, &mut s).unwrap();
    println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", s);
}


pub fn encode<R:Elem+Locked>(a:R)->impl Elem+Locked{
    let mut s=String::new();
    hypermelon::render(a,&mut s).unwrap();
    use base64::Engine;
    let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
    hypermelon::build::single("img").with(("src",s))
}
