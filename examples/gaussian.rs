use std::f64::consts::TAU;
fn gaussian(sigma: f64, mu: f64) -> impl Fn(f64) -> f64 {
    let aa = (sigma * TAU).sqrt().recip();
    move |x| (-0.5 * (x - mu).powi(2) / sigma.powi(2)).exp() * aa
}

// PIPE me to a file!
fn main() {
    let range = (0..10000)
        .map(|x| x as f64 / 10000.0)
        .map(|x| x * 10.0 - 5.0);

    let mut s = poloto::plot("gaussian", "x", "y");

    s.line("σ = 1.0", range.clone().map(|x| [x, gaussian(1.0, 0.0)(x)]));
    s.line("σ = 0.5", range.clone().map(|x| [x, gaussian(0.5, 0.0)(x)]));
    s.line("σ = 0.3", range.clone().map(|x| [x, gaussian(0.3, 0.0)(x)]));

    println!("{}", s.render());
}
