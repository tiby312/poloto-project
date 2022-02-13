use poloto::{num::integer::IntegerTickGen, plotnum::TickGenerator, prelude::*};
// PIPE me to a file!
fn main() {
    // hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let data = poloto::data()
        .histogram("", (0..).zip(trend.into_iter()))
        .build();

    let (xticks, xtick_fmt) = IntegerTickGen::generate(data.boundx());

    let xstep=xtick_fmt.step();
    let xtick_fmt=xtick_fmt.with_tick_fmt(|w,v|write!(w,"{}",v.dynamic_format(xstep)));

    let (yticks, ytick_fmt) = IntegerTickGen::generate(data.boundy());

    let mut plotter = data.plot_with(
        "Number of rides at theme park hourly",
        "Hour",
        "Number of rides",
        xticks,
        yticks,
        xtick_fmt,
        ytick_fmt,
    );

    println!("{}", poloto::disp(|w| plotter.simple_theme(w)));
}
