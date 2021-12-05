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
                use std::fmt::Write;
                write!(
                    e.writer(),
                    "{}{}{}",
                    CUSTOM_SVG,
                    poloto::disp_mut(|f| {
                        poloto::plot(formatm!("test {}", i), "x", "y")
                            .scatter("", test)
                            .render(f);
                    }),
                    poloto::SVG_END
                )
                .unwrap();
            }
        })
    });
}

pub const CUSTOM_SVG: &str = r####"
<svg class="poloto_background poloto" width="500px" height="100%" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">
<style>
.poloto {
    stroke-linecap:round;
    stroke-linejoin:round;
    font-family: 'Tahoma', sans-serif;
    stroke-width:2;
    }
    .scatter{stroke-width:7}
    .poloto_text{fill: black;}
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none;stroke-dasharray:none}
    .poloto_background{background-color: AliceBlue;}
    .poloto0stroke{stroke:  blue;}
    .poloto1stroke{stroke:  red;}
    .poloto2stroke{stroke:  green;}
    .poloto3stroke{stroke:  gold;}
    .poloto4stroke{stroke:  aqua;}
    .poloto5stroke{stroke:  lime;}
    .poloto6stroke{stroke:  orange;}
    .poloto7stroke{stroke:  chocolate;}
    .poloto0fill{fill:blue;}
    .poloto1fill{fill:red;}
    .poloto2fill{fill:green;}
    .poloto3fill{fill:gold;}
    .poloto4fill{fill:aqua;}
    .poloto5fill{fill:lime;}
    .poloto6fill{fill:orange;}
    .poloto7fill{fill:chocolate;}
</style>
"####;
