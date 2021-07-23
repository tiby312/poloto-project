use poloto::*;
use tagger::prelude::*;
fn main() {
    let x = (0..500).map(|x| (x as f32 / 500.0) * 10.0);

    let mut plotter = Plotter::new(
        default_svg().appendm(single!(poloto::HTML_CONFIG_DARK_DEFAULT)),
        "cos per year",
        "year",
        "cows",
    );

    plotter.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));

    plotter.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    println!("{}", plotter.render())
}
