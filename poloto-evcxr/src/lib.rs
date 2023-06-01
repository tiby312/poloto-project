use hypermelon::{elem::Locked, prelude::Elem};




pub fn evcxr_display<R:Elem+Locked>(elem:R){
    let a=wrap_img_tag(elem);
    let mut s = String::new();
    hypermelon::render(a, &mut s).unwrap();
    println!("EVCXR_BEGIN_CONTENT image/svg+xml\n{}EVCXR_END_CONTENT", s);
}


pub fn wrap_img_tag<R:Elem+Locked>(a:R)->impl Elem+Locked{
    //TODO Elem should have a render to string function?
    let mut s=String::new();
    hypermelon::render(a,&mut s).unwrap();
    use base64::Engine;
        
    let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
    hypermelon::build::single("img").with(("src",s))
}
