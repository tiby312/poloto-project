use poloto::prelude::*;
use poloto::build::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let s = StyleBuilder::new()
        .with_text_color("white")
        .with_back_color("black")
        .build();

    let mut plotter = PlotterBuilder::new()
        .with_data(DataBuilder::new().push(s))
        .build(
            "Some Trigonometry Plots 🥳",
            move_format!("This is the {} label", 'x'),
            "This is the y label",
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
