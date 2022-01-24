use poloto::prelude::*;
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut plotter = poloto::plot(
        "cos per year",
        "year",
        "cows",
        poloto::ctx::<f64>(),
        poloto::ctx::<f64>(),
    );

    plotter.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));

    plotter.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    println!("{}", poloto::disp(|a| plotter.simple_theme_dark(a)));
}
