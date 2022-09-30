use hypermelon::prelude::*;
use poloto::prelude::*;

use hypermelon::build as hb;
use poloto::simple_theme as ps;

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

    let header = ps::DefaultHeader::new().with_viewbox_width(1200.0);

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
        .with_dim(header.get_viewbox())
        .build();

    let plotter = poloto::data(plots)
        .with_opt(opt)
        .build()
        .labels("collatz", "x", "y");

    let style = hb::elem("style")
        .append(ps::simple_theme_dark())
        .append(hb::raw(".poloto_line{stroke-dasharray:2;stroke-width:2;}"));

    let res = header.append(style).append(plotter);

    hypermelon::render(res, hypermelon::stdout_fmt()).unwrap();
}
