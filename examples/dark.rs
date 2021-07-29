use poloto::*;
use tagger::prelude::*;
fn main() {
    let x = (0..500).map(|x| (x as f32 / 500.0) * 10.0);

    let mut plotter = plot("cos per year", "year", "cows");

    plotter.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));

    plotter.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    println!("{}", theme_dark().appendm(plotter.render()))
}
