// https://mathworld.wolfram.com/HeartCurve.html
fn heart(t: f64) -> [f64; 2] {
    [
        16.0 * t.sin().powi(3),
        13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
    ]
}

// PIPE me to a file!
fn main() {
    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    let mut s = poloto::plot("Heart Graph", "x", "y");

    s.line_fill("", range.map(|x| heart(x)));

    println!("{}", s.render(poloto::theme_light()));
}
