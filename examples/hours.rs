use poloto::{plotnum::PlotNumContext, prelude::*};
// PIPE me to a file!
fn main() {
    // hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let data = poloto::data()
        .histogram("", (0..).zip(trend.into_iter()))
        .build();

    let mut cx = data.boundx().default_context();
    let mut cy = data.boundy().default_context();
    let tickx=cx.compute_ticks();
    let ticky=cy.compute_ticks();


    let mut plotter = data
        .plot_with(
            "Number of rides at theme park hourly",
            "Hour",
            "Number of rides",
            tickx,
            ticky,
            cx,
            cy
        );
        //.with_xtick(boundx.steps((0..).step_by(6), |w, v| write!(w, "{} hr", v)));

    println!("{}", poloto::disp(|w| plotter.simple_theme(w)));
}
