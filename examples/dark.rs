use poloto::prelude::*;
fn main() -> std::fmt::Result {
    let mut plotter = poloto::plot_with_html(
        "cows per year",
        "year",
        "cows",
        poloto::HTML_CONFIG_DARK_DEFAULT,
    );

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
