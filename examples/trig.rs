use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut plotter = poloto::plot_with_html(
        "Some Trigonometry Plots 🥳",
        move_format!("This is the {} label", 'x'),
        "This is the y label",
        poloto::HTML_CONFIG_DARK_DEFAULT,
    );

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    //The iterator will be cloned and ran twice.
    plotter.line("cos", x.clone().map(|x| [x, x.cos()]));

    //Collect the iterator before passing it to a plot function
    //if you are using an expensive iterator.
    plotter.scatter("sin", x.clone().map(|x| [x, x.sin()]).collect::<Vec<_>>());

    plotter.histogram(
        move_format!("sin-{}", 10),
        x.clone().step_by(3).map(|x| [x, x.sin() - 10.]),
    );

    plotter.line_fill(
        move_format!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]),
    );

    plotter.render_io(std::io::stdout())?;

    Ok(())
}
