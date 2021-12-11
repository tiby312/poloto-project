// PIPE me to a file!
fn main() {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            //Base case
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut plotter = poloto::plot("collatz", "x", "y");

    plotter.ymarker(0);

    for i in 1000..1006 {
        plotter.line(poloto::formatm!("c({})", i), (0..).zip(collatz(i)));
    }

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::SVG_END
    )
}
