use hypermelon::Elem;
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

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, true])
        .with_dim(svg.get_viewbox())
        .build();

    let style = poloto::Theme::dark().append(".poloto_line{stroke-dasharray:2;stroke-width:2;}");

    let plots = plots!(
        poloto::build::plots_dyn((1000..1006).map(|i| {
            let name = hypermelon::format_move!("c({})", i);
            let it = (0..).zip(collatz(i));
            poloto::build::buffered_plot(it).line(name)
        })),
        poloto::build::origin()
    );

    poloto::data(plots)
        .with_opt(opt)
        .build()
        .labels("collatz", "x", "y")
        .append_to(svg.append(style))
        .render_stdout();
}
