// PIPE me to a file!
fn main() {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut data = poloto::data();
    for i in 1000..1006 {
        data.line(poloto::formatm!("c({})", i), (0..).zip(collatz(i)));
    }

    let dim = [1200.0, 600.0];

    data.ymarker(0).xtick_lines().ytick_lines().with_dim(dim);

    let mut plotter = data.build().plot("collatz", "x", "y");

    print!(
        "{}<style>{}{}</style>{}{}",
        poloto::disp(|a| poloto::simple_theme::write_header(a, [Some(800.0), None], dim)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
