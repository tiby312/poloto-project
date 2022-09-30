use poloto::prelude::*;
fn main() {
    // hourly trend over one day.
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.iter().copied());

    let plots = poloto::plots!(
        it.cloned_plot().histogram(""),
        poloto::build::markers([24], [])
    );

    poloto::data(plots)
        .with_xticks(poloto::ticks::from_iter((0..).step_by(6)))
        .labels("title", "x", "y")
        .simple_theme()
        .render_stdout();
}
