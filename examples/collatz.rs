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

    //Make the plotting area slightly larger.
    let dim = [1300.0, 600.0];

    let canvas = poloto::gen_canvas()
        .xtick_lines()
        .ytick_lines()
        .with_dim(dim)
        .build();
    let mut plotter = data.build().plot_with_canvas(canvas, "collatz", "x", "y");

    let hh = poloto::simple_theme::determine_height_from_width(plotter.get_dim(), 800.0);

    print!(
        "{}<style>{}{}</style>{}{}",
        poloto::disp(|a| poloto::simple_theme::write_header(a, [800.0, hh], dim)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:2;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
