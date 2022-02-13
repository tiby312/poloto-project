use poloto::prelude::*;
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let data = poloto::data();
    data.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));
    data.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    let mut plotter = data.plot("cos per year", "year", "cows");

    println!("{}", poloto::disp_mut(|a| plotter.simple_theme_dark(a)));
}
