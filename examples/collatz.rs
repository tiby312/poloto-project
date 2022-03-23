use poloto::prelude::*;

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

    use poloto::build::line;
    let data = poloto::build::markers(
        poloto::build::plots_dyn(
            (1000..1006)
                .map(|i| line(formatm!("c({})", i), (0..).zip(collatz(i))))
                .collect(),
        ),
        None,
        Some(0),
    );

    //Make the plotting area slightly larger.
    let dim = [1300.0, 600.0];

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, true])
        .with_dim(dim)
        .build();

    let plotter = simple_fmt!(opt, data, "collatz", "x", "y");

    use poloto::simple_theme;
    let hh = simple_theme::determine_height_from_width(plotter.get_dim(), simple_theme::DIM[0]);

    print!(
        "{}<style>{}{}</style>{}{}",
        poloto::disp(|a| poloto::simple_theme::write_header(a, [simple_theme::DIM[0], hh], dim)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:2;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
