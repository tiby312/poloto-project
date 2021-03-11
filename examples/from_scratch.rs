use poloto::prelude::*;
use poloto::*;
fn main() -> std::fmt::Result {
    let s = StyleBuilder::new()
        .with_text_color("white")
        .with_back_color("black")
        .with_colors(["red"; default_tags::NUM_COLORS])
        .build();

    let mut plotter = PlotterBuilder::new()
        .with_data(DataBuilder::new().push(s))
        .build("cows per year", "year", "cows");

    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    plotter.line("test1", x.clone().map(|x| [x, x.cos()]).twice_iter());

    plotter.line("test2", x.clone().map(|x| [x, x.sin()]).twice_iter());

    plotter.line("test3", x.clone().map(|x| [x, x.tan()]).twice_iter());

    plotter.render_io(std::io::stdout())?;

    Ok(())
}
