// PIPE me to a file!
fn main() {
    // https://mathworld.wolfram.com/HeartCurve.html
    let heart = |t: f64| {
        [
            16.0 * t.sin().powi(3),
            13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
        ]
    };

    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    let mut plotter = poloto::plot("Heart Graph", "x", "y");

    plotter.line_fill_raw("heart", range.map(heart));
    plotter.preserve_aspect();

    plotter
        .xmarker(-20.)
        .xmarker(20.)
        .ymarker(-20.)
        .ymarker(20.);

    println!(
        "{}",
        poloto::disp(|a| poloto::simple_theme_dark(a, plotter))
    );
}
