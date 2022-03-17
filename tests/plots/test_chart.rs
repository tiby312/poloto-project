use super::*;

const fn generate_test() -> [&'static [[f64; 2]]; 9] {
    let test0 = &[[0.0, 6000.0], [0.0, 200.0]];

    let test1 = &[[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let test2 = &[[0.1, 0.1], [0.3, 0.6]];

    let test3 = &[[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let test4 = &[];

    let test5 = &[[-1000000000000.0, 0.0], [-1000000000000.0, 0.0]];

    let test6 = &[[0.0, 100000000.0], [1.0, 100000000.00001]];

    let test7 = &[[0.0, 50424323.0]];

    let test8 = &[[-38.0, -38.0], [33.0, 33.0]];

    [
        test0, test1, test2, test3, test4, test5, test6, test7, test8,
    ]
}
const fn generate_test_int() -> [&'static [[i128; 2]]; 9] {
    let test0 = &[[0, 6000], [0, 200]];

    let test1 = &[[-1000000000000, 0], [1000000000000, 0]];

    let test2 = &[
        [-1000000000000, 0],
        [-1000000000005, 0],
        [-1000000000002, 0],
    ];

    //failed
    let test3 = &[[-1000000000000, 0]];

    let test4 = &[];

    //failed
    let test5 = &[[-1000000000000, 0], [-1000000000000, 0]];

    let test6 = &[[0, 100000000], [1, 100000001]];

    let test7 = &[[0, 50424323], [1, -10000]];

    let test8 = &[[-38, -38], [33, 33]];

    [
        test0, test1, test2, test3, test4, test5, test6, test7, test8,
    ]
}
use poloto::formatm;

//Create a bunch of graphs with different scales to try to expose corner cases.
#[test]
fn test_chart() -> std::fmt::Result {
    let w = util::create_test_file("test_chart.html");

    let mut e = tagger::new(w);

    e.elem("html", |e| e.attr("style", "background-color:#262626"))?
        .build(|e| {
            e.elem("div", |d| d.attr("style", "display:flex;flex-wrap:wrap;"))?
                .build(|e| {
                    for (i, &test) in generate_test().iter().enumerate() {
                        use std::fmt::Write;
                        write!(
                            e.writer_escapable(),
                            "{}<style>{}{}</style>{}{}",
                            CUSTOM_SVG,
                            poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
                            ".poloto_scatter{stroke-width:20}",
                            poloto::disp(|a| {
                                let s = poloto::build::scatter("", test).build();
                                s.stage().plot(formatm!("test {}", i), "x", "y").render(a)
                            }),
                            poloto::simple_theme::SVG_END
                        )?;
                    }

                    for (i, &test) in generate_test_int().iter().enumerate() {
                        use std::fmt::Write;
                        write!(
                            e.writer_escapable(),
                            "{}<style>{}{}</style>{}{}",
                            CUSTOM_SVG,
                            poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
                            ".poloto_scatter{stroke-width:20}",
                            poloto::disp(|a| {
                                let s = poloto::build::scatter("", test).build();

                                s.stage().plot(formatm!("test {}", i), "x", "y").render(a)
                            }),
                            poloto::simple_theme::SVG_END
                        )?;
                    }

                    Ok(())
                })
        })
}

pub const CUSTOM_SVG: &str = r####"<svg class="poloto_background poloto" width="500px" height="100%" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">"####;
