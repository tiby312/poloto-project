const fn generate_test() -> [&'static [[f64; 2]]; 9] {
    let test0 = &[[0.0, 6000.0], [0.0, 200.0]];

    let test1 = &[[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let test2 = &[[0.1, 0.1], [0.3, 0.6]];

    let test3 = &[[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let test4 = &[];

    let test5 = &[[-1000000000000.0, 0.0], [-1000000000000.0, 0.0]];

    let test6 = &[[0.0, 100000000.0], [1.0, 100000000.00001]];

    let test7 = &[[0.0, 50424323.0], [1.0, -10000.0]];

    let test8 = &[[-38.0, -38.0], [33.0, 33.0]];

    [
        test0, test1, test2, test3, test4, test5, test6, test7, test8,
    ]
}

use poloto::formatm;

//Create a bunch of graphs with different scales to try to expose corner cases.
fn main() {
    let mut e = tagger::new(tagger::upgrade_write(std::io::stdout()));

    e.elem("html", tagger::no_attr()).build(|e| {
        e.elem("div", |d| {
            d.attr("style", "display:flex;flex-wrap:wrap;");
        })
        .build(|e| {
            for (i, &test) in generate_test().iter().enumerate() {
                poloto::default_svg(
                    e,
                    |d| {
                        d.attr("width", "500px").attr("height", "100%");
                    },
                    |e| {
                        e.put_raw(format_args!(
                            "<style>{}</style>",
                            poloto::STYLE_CONFIG_LIGHT_DEFAULT
                        ));

                        poloto::plot(formatm!("test {}", i), "x", "y")
                            .scatter("", test)
                            .render(e.writer());
                    },
                );
            }
        })
    });
}
