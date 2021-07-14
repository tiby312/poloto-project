// Include prelude for acess to move_format macro
use poloto::prelude::*;

// PIPE me to a file!
fn main() -> core::fmt::Result {
    

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    // Collect the iterator before passing it to a plot function
    // if you are using an expensive iterator.
    // The buffer has to live longer than the plotter, so we collect it here.
    let buffer=x.clone().map(|x| [x, x.sin()]).collect::<Vec<_>>();
    
    
    let mut plotter = poloto::plot_with_html(
        "Some Trigonometry Plots 🥳",
        move_format!("This is the {} label", 'x'),
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
