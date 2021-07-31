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

use tagger::prelude::*;

//Create a bunch of graphs with different scales to try to expose corner cases.
fn main() {
    let mut html = elem!("html");

    let mut div = elem!(
        "div",
        ("style","display:flex;flex-wrap:wrap;")
    );

    for (i, &test) in generate_test().iter().enumerate() {
        let mut svg = elem!(
            "svg",
            ("width","500px"),
            ("height","100%")
        );

        svg.append(poloto::HTML_CONFIG_LIGHT_DEFAULT);

        let mut s = poloto::plot(formatm!("test {}", i), "x", "y");

        s.scatter("", test);

        div.append(svg.appendm(s.render()));
    }

    html.append(div);

    println!("{}", html.display());
}
