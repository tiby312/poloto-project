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

    let data = poloto::build::plots_dyn(
        (1000..1006)
            .map(|i| poloto::build::line(formatm!("c({})", i), (0..).zip(collatz(i))))
            .collect(),
    );

    //Make the plotting area slightly larger.
    let dim = [1300.0, 600.0];

    let canvas = poloto::render::canvas()
        .with_tick_lines([true, true])
        .with_dim(dim)
        .build();

    let mut plotter = data
        .build_with([], [0])
        .stage_with(&canvas)
        .plot("collatz", "x", "y");

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
