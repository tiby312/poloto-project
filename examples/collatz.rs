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

    //Make the plotting area slightly larger.
    let dim = [1300.0, 600.0];

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, true])
        .with_dim(dim)
        .build();

    let plotter = quick_fmt_opt!(
        opt,
        "collatz",
        "x",
        "y",
        poloto::build::plots_dyn(
            (1000..1006)
                .map(|i| {
                    let name = formatm!("c({})", i);
                    (0..).zip(collatz(i)).buffered_plot().line(name)
                })
                .collect(),
        ),
        poloto::build::origin()
    );

    use hypermelon::prelude::*;
    let res = poloto::simple_theme::DefaultHeader::new()
        .with_viewbox(dim)
        .with_dim_width(800.0)
        .append(poloto::simple_theme::simple_theme_dark())
        .append(
            hypermelon::build::elem("style").append(hypermelon::build::raw(
                ".poloto_line{stroke-dasharray:2;stroke-width:2;}",
            )),
        )
        .append(plotter);

    hypermelon::render(res, hypermelon::stdout_fmt()).unwrap();
}
