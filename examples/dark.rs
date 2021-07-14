use poloto::prelude::*;
fn main() -> std::fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut plotter = poloto::plot_with_html(
        "cows per year",
        "year",
        "cows",
        poloto::HTML_CONFIG_DARK_DEFAULT,
    );

    plotter.line(move_format!("test {}", 1), x.clone().map(|x| [x, x.cos()]));

    plotter.line(move_format!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    plotter.render_io(std::io::stdout())
}
