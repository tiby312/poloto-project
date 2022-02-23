use poloto::prelude::*;
fn main() {
    // hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let data = poloto::data()
        .histogram("", (0..).zip(trend.into_iter()))
        .build();

    let (xtick, xtick_fmt) = poloto::steps(data.boundx(), (0..).step_by(6));
    let (ytick, ytick_fmt) = poloto::ticks_from_default(data.boundy());

    let mut plotter = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "Number of rides at theme park hourly",
            "Hour",
            "Number of rides",
            xtick_fmt,
            ytick_fmt,
        ),
    );

    print!("{}", poloto::disp(|w| plotter.simple_theme(w)));
}
