// See https://en.wikipedia.org/wiki/Gaussian_function
fn gaussian(sigma: f64, mu: f64) -> impl Fn(f64) -> f64 + Send + Sync {
    use std::f64::consts::TAU;
    move |x| (-0.5 * (x - mu).powi(2) / sigma.powi(2)).exp() * (sigma * TAU).sqrt().recip()
}

// PIPE me to a file!
fn main() {
    let range = (0..200).map(|x| x as f64 / 200.0).map(|x| x * 10.0 - 5.0);

    poloto::plot("gaussian", "x", "y")
        .line("σ = 1.0", range.clone().map(|x| [x, gaussian(1.0, 0.0)(x)]))
        .line("σ = 0.5", range.clone().map(|x| [x, gaussian(0.5, 0.0)(x)]))
        .line("σ = 0.3", range.clone().map(|x| [x, gaussian(0.3, 0.0)(x)]))
        .ymarker(0)
        .simple_theme(poloto::upgrade_write(std::io::stdout()));
}
