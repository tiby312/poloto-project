use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut s = poloto::data();
    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    s.line(
        "tan(x)",
        poloto::buffered_iter::buffered(
            x.clone()
                .map(|x| [x, x.tan()])
                .crop_above(10.0)
                .crop_below(-10.0)
                .crop_left(2.0),
        ),
    );

    s.line(
        "sin(2x)",
        poloto::bounded_iter::from_rect(
            [0.0, 10.0],
            [0.0, 10.0],
            x.clone().map(|x| [x, (2.0 * x).sin()]),
        ),
    );

    s.line(
        "2*cos(x)",
        poloto::buffered_iter::buffered(x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4)),
    );

    let mut plotter = s.build().plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
