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
use poloto::prelude::*;
use tagger::prelude::*;

//Create a bunch of graphs with different scales to try to expose corner cases.
fn main() -> fmt::Result {
    let mut root = tagger::Element::new(tagger::upgrade(std::io::stdout()));

    root.elem("html", |writer| {
        let html = writer.write(|w| Ok(w))?;

        html.elem("div", |writer| {
            let div = writer.write(|w| w.attr("style", "display:flex;flex-wrap:wrap;"))?;

            for (i, test) in generate_test().iter().enumerate() {
                div.elem("svg", |writer| {
                    //Build the svg tag from scratch so we can use our own
                    //width and height
                    let svg = writer.write(|w| {
                        use poloto::default_tags::*;
                        w.attr("class", CLASS)?;
                        w.attr("xmlns", XMLNS)?;
                        w.with_attr("viewBox", wr!("0 0 {} {}", WIDTH, HEIGHT))?;
                        w.attr("width", "500px")?.attr("height", "100%")?;
                        Ok(w)
                    })?;

                    let mut s = poloto::PlotterBuilder::new().with_no_svg_tag().build(
                        move_format!("test {}", i),
                        "x",
                        "y",
                    );

                    s.scatter("test", test.iter().copied().twice_iter());

                    s.render(svg)
                })?;
            }
            Ok(div)
        })?;

        Ok(html)
    })?;
    Ok(())
}
