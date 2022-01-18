use poloto::formatm;

// PIPE me to a file!
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut plotter = poloto::plot(
        poloto::ctx::f64,
        poloto::ctx::f64,
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    use poloto::Croppable;
    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    plotter.line(
        "tan(x)",
        x.clone()
            .map(|x| [x, x.tan()])
            .crop_above(10.0)
            .crop_below(-10.0)
            .crop_left(2.0),
    );

    plotter.line("sin(2x)", x.clone().map(|x| [x, (2.0 * x).sin()]));

    plotter.line(
        "2*cos(x)",
        x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
    );

    println!("{}", poloto::disp(|a| poloto::simple_theme(a, plotter)));
}
