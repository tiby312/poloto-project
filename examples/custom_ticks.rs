use hypermelon::format_move;
use poloto::build;
fn main() {
    // hourly trend over one day.
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.iter());

    let plots = poloto::plots!(
        build::plot("").histogram().cloned(it),
        build::markers([24], [])
    );

    let data = poloto::data(plots);

    let ticks =
        poloto::ticks::from_iter((0..).step_by(6)).with_tick_fmt(|&v| format_move!("{} hr", v));

    data.map_xticks(|_| ticks)
        .build_and_label(("title", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
