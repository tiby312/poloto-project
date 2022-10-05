use hypermelon::prelude::*;
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

    let svg = poloto::header().with_viewbox_width(1200.0);

    let opt = poloto::render::render_opt()
        .with_tick_lines([true, true])
        .with_viewbox(svg.get_viewbox())
        .move_into();

    let style =
        poloto::render::Theme::dark().append(".poloto_line{stroke-dasharray:2;stroke-width:2;}");

    let a = poloto::build::plots_dyn((1000..1006).map(|i| {
        let name = format_move!("c({})", i);
        let it = (0..).zip(collatz(i));
        poloto::build::plot(name).line().buffered(it)
    }));

    let b = poloto::build::origin();

    poloto::data(plots!(a, b))
        .map_opt(|_| opt)
        .build_and_label(("collatz", "x", "y"))
        .append_to(svg.append(style))
        .render_stdout();
}
