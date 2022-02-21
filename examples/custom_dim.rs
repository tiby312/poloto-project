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
    data.ymarker(0);
    data.with_dim(2000.0, 1000.0);

    let mut plotter = data.build().plot("collatz", "x", "y");

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::disp_const(|w| poloto::simple_theme::write_header(w, 2000.0, 1000.0)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
