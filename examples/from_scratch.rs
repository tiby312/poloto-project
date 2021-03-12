use poloto::prelude::*;
use poloto::*;
fn main() -> std::fmt::Result {
    let s = StyleBuilder::new()
        .with_text_color("white")
        .with_back_color("black")
        .build();

    let mut plotter = PlotterBuilder::new()
        .with_data(DataBuilder::new().push(s))
        .build("cows per year", "year", "cows");

    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    plotter.line(
        move_format!("test {}", 1),
        x.clone().map(|x| [x, x.cos()]).twice_iter(),
    );

    plotter.line(
        move_format!("test {}", 2),
        x.clone().map(|x| [x, x.sin()]).twice_iter(),
    );

    plotter.render_io(std::io::stdout())?;

    Ok(())
}
