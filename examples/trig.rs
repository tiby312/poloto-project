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

    //Call twice_iter to allow the iterator to be cloned and ran twice.
    plotter.line("cos", x.clone().map(|x| [x, x.cos()]).twice_iter());

    //Call `buffer_iter` to communicate that iterator results
    //should be stored to a Vec buffer for the second iteration.
    plotter.scatter("sin", x.clone().map(|x| [x, x.sin()]).buffer_iter());

    plotter.histogram(
        move_format!("sin-{}", 10),
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .buffer_iter(),
    );

    plotter.line_fill(
        move_format!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]).buffer_iter(),
    );

    plotter.render_io(std::io::stdout())?;

    Ok(())
}
