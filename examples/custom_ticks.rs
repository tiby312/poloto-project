use poloto::prelude::*;
fn main() {
    // hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.into_iter());
    let data = poloto::build::histogram("", it).collect();

    let canvas = poloto::render::canvas().build();

    let (xtick, xtick_fmt) = poloto::ticks::from_iter((0..).step_by(6));
    let (ytick, ytick_fmt) = poloto::ticks::from_default(data.boundy(&canvas));

    let mut pp = data.stage_with(canvas).plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "Number of rides at theme park hourly",
            "Hour",
            "Number of rides",
            xtick_fmt.with_tick_fmt(|w, v| write!(w, "{} hr", v)),
            ytick_fmt,
        ),
    );

    print!("{}", poloto::disp(|w| pp.simple_theme(w)));
}
