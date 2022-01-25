use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let title = poloto::fmt::name_ext(|w, (x, xs), _| {
        write!(w, "from {} to {} in steps of {}", x[0], x[1], xs)
    });

    let mut plotter = poloto::plot(
        title,
        formatm!("This is the {} label", 'x'),
        "This is the y label",
        f64::default_ctx(),
        f64::default_ctx(),
    );

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    plotter.line_fill(
        "tan(x)",
        x.clone()
            .map(|x| [x, x.tan()])
            .crop_above(10.0)
            .crop_below(0.0)
            .crop_left(2.0),
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
