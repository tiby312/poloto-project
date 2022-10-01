use hypermelon::prelude::*;
use poloto::simple_theme::DefaultHeader;
use poloto::simple_theme::Theme;
use poloto::ticks::TickFmt;
use poloto::ticks::TickFormat;
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

    let plots = poloto::plots!(
        poloto::build::plots_dyn(
            (1000..1006)
                .map(|i| {
                    let name = hypermelon::format_move!("c({})", i);
                    let it = (0..).zip(collatz(i));
                    poloto::buffered_plot(it).line(name)
                })
                .collect(),
        ),
        poloto::build::origin()
    );

    let ff = poloto::ticks::DefaultTickFmt
        .with_ticks(|w, v| write!(w, "{} a", v))
        .with_where(|w, _| write!(w, "{}", "aaa"));
    let steps = poloto::ticks::from_iter((0..).step_by(6)).with_fmt(ff);

    poloto::data(plots)
        .with_xticks(steps)
        .build()
        .labels("title", "x", "y")
        .append_to(
            DefaultHeader::new()
                .append(Theme::light().append(hypermelon::build::raw("more_stuff"))),
        )
        .render_stdout();
}
