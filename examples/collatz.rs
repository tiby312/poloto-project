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

    let svg = poloto::Header::new().with_viewbox_width(1200.0);

    let dyn_plots = poloto::build::plots_dyn(
        (1000..1006)
            .map(|i| {
                let name = hypermelon::format_move!("c({})", i);
                let it = (0..).zip(collatz(i));
                poloto::buffered_plot(it).line(name)
            })
            .collect(),
    );

    let plots = plots!(dyn_plots, poloto::build::origin());

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, true])
        .with_dim(svg.get_viewbox())
        .build();

    let svg = svg
        .add(poloto::Theme::dark().with_style(".poloto_line{stroke-dasharray:2;stroke-width:2;}"));

    poloto::data(plots)
        .with_opt(opt)
        .build()
        .labels("collatz", "x", "y")
        .append_to(svg)
        .render_stdout();
}
