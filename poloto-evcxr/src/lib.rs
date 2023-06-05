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
        let mut s = String::new();
        hypermelon::render(self.inline(), &mut s).unwrap();

        //inline css part as well.
        let s = s.replace("\n", "");

        use base64::Engine;
        let s=format!("data:image/svg+xml;base64,{}",base64::engine::general_purpose::STANDARD.encode(&s));
        let r=hypermelon::build::single("img").with(("src",s));
        let mut s=String::new();
        hypermelon::render(r,&mut s).unwrap();
        println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", s);
    }
}

///
/// the css is inlined so that github viewer will work
/// image/svg+xml cell with plain text
///
pub fn evcxr_display_svg<R: Elem + Locked>(elem: R) {
    let mut s = String::new();
    hypermelon::render(elem.inline(), &mut s).unwrap();

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

    println!("EVCXR_BEGIN_CONTENT text/html\n{}\nEVCXR_END_CONTENT", s);
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
