use poloto::prelude::*;
fn main() -> std::fmt::Result {
    let data = [[1.0f64, 4.0], [2.0, 5.0], [3.0, 6.0]];

    use poloto::build::{HeaderBuilder, PlotterBuilder, StyleBuilder, NUM_COLORS};
    //Make line purple.
    let mut plotter = PlotterBuilder::new()
        .with_header(
            HeaderBuilder::new()
                .push(
                    StyleBuilder::new()
                        .with_colors(["purple"; NUM_COLORS])
                        .build(),
                )
                .build(),
        )
        .build("cows per year", "year", "cows");

    plotter.line("cow", data.iter().map(|&x| x).twice_iter());

    plotter.render_io(std::io::stdout())?;

    Ok(())
}
