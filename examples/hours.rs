use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let data = (0..).zip(trend.into_iter());

    let mut plotter = poloto::plot(
        "Number of rides at theme park hourly",
        "Hour",
        "Number of rides",
        poloto::steps((0..24).step_by(5), |w, v| write!(w, "{} hr", v)),
        i128::default_ctx().with_marker(0),
    );

    plotter.histogram("", data);

    println!("{}", poloto::disp(|w| plotter.simple_theme(w)));
}
