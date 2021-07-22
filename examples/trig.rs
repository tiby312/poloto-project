use tagger::prelude::*;

// PIPE me to a file!
fn main() {
    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    // Collect the iterator before passing it to a plot function
    // if you are using an expensive iterator.
    // The buffer has to live longer than the plotter, so we collect it here.
    let buffer = x.clone().map(|x| [x, x.sin()]).collect::<Vec<_>>();

    let mut plotter = poloto::plot_with_html(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
        poloto::HTML_CONFIG_DARK_DEFAULT,
    );

    // The iterator will be cloned and ran twice.
    plotter.line("cos", x.clone().map(|x| [x, x.cos()]));

    // When passing the buffer, make sure you pass it as a reference.
    // If you don't do this, then the buffer will be duplicated in memory as
    // the plotter will call `.clone()` on the iterator.
    plotter.scatter("sin", &buffer);

    plotter.histogram(
        formatm!("sin-{}", 10),
        x.clone().step_by(3).map(|x| [x, (x.sin() - 10.).round()]),
    );

    plotter.line_fill(
        formatm!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]),
    );

    println!("{}", plotter.render().unwrap());
}
