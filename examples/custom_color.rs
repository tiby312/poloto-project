use poloto::prelude::*;
fn main() -> std::fmt::Result {
    let data = [[1.0f64, 4.0], [2.0, 5.0], [3.0, 6.0]];

    let mut plotter = poloto::plot("cows per year", "year", "cows");
    plotter.line("cow", data.iter().map(|&x| x).twice_iter());

    //Make line purple.
    plotter.with_text("<style>.poloto0stroke{stroke:purple;}</style>");

    plotter.render_io(std::io::stdout())?;

    Ok(())
}
