use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let data = {
        let mut data = poloto::data();
        data.histogram("", (0..).zip(trend.into_iter()));
        data.build()
    };

    let boundx = data.boundx();

    let mut plotter = data
        .plot(
            "Number of rides at theme park hourly",
            "Hour",
            "Number of rides",
        )
        .with_xtick(boundx.steps((0..).step_by(6), |w, v| write!(w, "{} hr", v)));

    println!("{}", poloto::disp(|w| plotter.simple_theme(w)));
}
