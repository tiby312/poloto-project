const fn generate_test() -> [&'static [[f32; 2]]; 8] {
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

use tagger::attr_builder;
use tagger::prelude::*;

//Create a bunch of graphs with different scales to try to expose corner cases.
fn main() {
    let mut html = elem!("html");

    let mut div = elem!(
        "div",
        attr_builder()
            .attr("style", "display:flex;flex-wrap:wrap;")
            .build()
    );

    for (i, &test) in generate_test().iter().enumerate() {
        let d = attr_builder()
            .attr_whole(poloto::SVG_HEADER_DEFAULT_WITHOUT_TAG)
            .attr("width", "500px")
            .attr("height", "100%")
            .build();

        let mut svg = elem!("svg", d);

        let mut s = poloto::plot_with_html_raw(
            formatm!("test {}", i),
            "x",
            "y",
            "",
            poloto::HTML_CONFIG_LIGHT_DEFAULT,
            "",
        );

        s.scatter("", test);

        svg.append(single!(s.render().unwrap()));
        div.append(svg);
    }

    html.append(div);

    println!("{}", html);
}
