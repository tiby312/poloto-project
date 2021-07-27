use tagger::prelude::*;

// PIPE me to a file!
fn main() {
    let x: Vec<_> = (0..500).map(|x| (x as f64 / 500.0) * 10.0).collect();

    let mut plotter = poloto::Plotter::new(
        poloto::default_svg().appendm(single!(poloto::HTML_CONFIG_DARK_DEFAULT)),
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    // Filter out large asymptotic manually before feeding it to the plotter.
    plotter.line(
        "tan(x)",
        x.iter()
            .map(|&x| [x, x.tan()])
            .filter(|&[_, y]| y < 10.0 && y > -10.0),
    );

    plotter.line("sin(2x)", x.iter().map(|&x| [x, (2.0 * x).sin()]));

    plotter.line("2*cos(x)", x.iter().map(|&x| [x, 2.0 * x.cos()]));

    println!("{}", plotter.render());
}
