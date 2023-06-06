use hypermelon::{elem::Locked, prelude::Elem};

pub mod prelude {
    pub use super::RenderEvcxr;
}

pub trait RenderEvcxr {
    fn render_evcxr(self);
    fn render_evcxr_img(self);
    
}

impl<R: Elem + Locked> RenderEvcxr for poloto::render::Stage4<R> {
    fn render_evcxr(self) {
        evcxr_display_svg(self)
    }
    fn render_evcxr_img(self) {
        evcxr_display_img(self)
    }
}


pub fn encode_svg_as_img<R:Elem+Locked>(elem:R)-> impl Elem+Locked{
    let mut s = String::new();
    hypermelon::render(elem.inline(), &mut s).unwrap();

    //inline css part as well.
    let s = s.replace("\n", "");

    use base64::Engine;
    let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
    hypermelon::build::single("img").with(("src",s))
}

// pub fn encode_svg_as_img_escapable<R:Elem>(elem:R)-> impl Elem{
//     let mut s = String::new();
//     hypermelon::render_escapable(elem.inline(), &mut s).unwrap();

//     //inline css part as well.
//     let s = s.replace("\n", "");

//     use base64::Engine;
//     let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
//     hypermelon::build::single("img").with(("src",s))
// }


pub fn evcxr_disp_elem<E:Elem+Locked>(elem:E){
    let mut s=String::new();
    hypermelon::render(elem,&mut s).unwrap();
    evcxr_disp(s);
}

pub fn evcxr_disp<D:std::fmt::Display>(s:D){
    println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", s);
}

pub fn evcxr_display_img<R: Elem>(elem: R) {
    let mut s = String::new();
    hypermelon::render_escapable(elem.inline(), &mut s).unwrap();

    //inline css part as well.
    let s = s.replace("\n", "");

    use base64::Engine;
    let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
    let r=hypermelon::build::single("img").with(("src",s));
    let mut s=String::new();
    hypermelon::render(r,&mut s).unwrap();
    evcxr_disp(s);

}

///
/// the css is inlined so that github viewer will work
/// image/svg+xml cell with plain text
///
pub fn evcxr_display_svg<R: Elem>(elem: R) {
    let mut s = String::new();
    hypermelon::render_escapable(elem.inline(), &mut s).unwrap();

    //inline css part as well.
    let s = s.replace("\n", "");

    // use base64::Engine;
    // let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
    // let r=hypermelon::build::single("img").with(("src",s));
    // let mut s=String::new();
    // hypermelon::render(r,&mut s).unwrap();

    // let s=format!("data:image/svg+xml;ascii,{}",s);
    // let r=hypermelon::build::single("img").with(("src",s));
    // let mut s=String::new();
    // hypermelon::render(r,&mut s).unwrap();
    evcxr_disp(s);
}

// ///
// /// html cell with image/svg+xml mime with base64 encoding
// ///
// pub fn evcxr_display<R:Elem+Locked>(elem:R){
//     let a=encode(elem);
//     let mut s = String::new();
//     hypermelon::render(a, &mut s).unwrap();
//     println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", s);
// }

// pub fn encode<R:Elem+Locked>(a:R)->impl Elem+Locked{
//     let mut s=String::new();
//     hypermelon::render(a,&mut s).unwrap();
//     use base64::Engine;
//     let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
//     hypermelon::build::single("img").with(("src",s))
// }
