use poloto::prelude::*;
fn main() {
    // hourly trend over one day.
    let trend = vec![
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let it = (0..).zip(trend.into_iter());

    let data = poloto::data(poloto::build::histogram("", it).markers([24], []));

    let opt = poloto::render::render_opt();
    let (_, by) = data.bounds(&opt);
    let xtick_fmt = poloto::ticks::from_iter((0..).step_by(6));
    let ytick_fmt = poloto::ticks::from_default(by);

    let pp = data.plot_with(
        opt,
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
