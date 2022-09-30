use poloto::prelude::*;
fn main() {
    // hourly trend over one day.
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.iter().copied());

    let data = poloto::data(plots!(
        it.cloned_plot().histogram(""),
        poloto::build::markers([24], [])
    ));

    let data = data.build("title", "x", "y");
    let data = data
        .with_opt(poloto::render::render_opt())
        .build("title", "x", "y");
    let data = data
        .with_xticks(poloto::ticks::from_iter((0..).step_by(6)))
        .build("title", "x", "y");

    poloto::simple_stdout(plotter)
}
