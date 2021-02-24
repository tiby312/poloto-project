const fn generate_test() -> [&'static [[f64; 2]]; 8] {
    let test0 = &[[0.0, 6000.0], [0.0, 200.0]];

    let test1 = &[[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let test2 = &[[0.1, 0.1], [0.3, 0.6]];

    let test3 = &[[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let test4 = &[];

    let test5 = &[[-1000000000000.0, 0.0], [-1000000000000.0, 0.0]];

    let test6 = &[[0.0, 100000000.0], [1.0, 100000000.00001]];

    let test7 = &[[0.0, 50424323.0], [1.0, -10000.0]];

    [test0, test1, test2, test3, test4, test5, test6, test7]
}

use core::fmt;
use tagger::prelude::*;
fn main() -> fmt::Result {
    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.elem("html", |writer| {
        let html = writer.write(|w| Ok(w))?;

        html.elem("div", |writer| {
            let div = writer.write(|w| w.attr("style", "display:flex;flex-wrap:wrap;"))?;

            for test in generate_test().iter() {
                div.elem("svg", |writer| {
                    let svg = writer.write(|w| {
                        w.attr("width", "500px")?.attr("height", "100%")?;
                        poloto::default_svg_tag::default()(w)?;
                        Ok(w)
                    })?;

                    let mut s =
                        poloto::plot("Demo: you can use CSS patterns if you embed SVG!", "x", "y");

                    s.scatter("test", test.iter().copied());

                    s.render(svg)
                })?;
            }
            Ok(div)
        })?;

        Ok(html)
    })?;
    Ok(())
}
