use tagger::prelude::*;
fn main() {
    let x = (0..500).map(|x| (x as f32 / 500.0) * 10.0);

    let mut plotter = poloto::plot_with_html(
        "cows per year",
        "year",
        "cows",
        poloto::HTML_CONFIG_DARK_DEFAULT,
    );

    plotter.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));

    plotter.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    println!("{}",plotter.render().unwrap())
}
