use poloto::prelude::*;
use tagger::prelude::*;
fn main() -> std::fmt::Result {
    let data = [[1.0f64, 4.0], [2.0, 5.0], [3.0, 6.0]];

    let mut buffer = String::new();

    let mut plotter = poloto::plot(&mut buffer);
    plotter.line(wr!("cow"), data.iter().map(|&x| x).twice_iter());

    //Make line purple.
    plotter.with_raw_text(wr!("{}","<style>.poloto0stroke{stroke:purple;}</style>"));

    plotter.render(wr!("cows per year"), wr!("year"), wr!("cows"))?;

    println!("{}", buffer);
    Ok(())
}
