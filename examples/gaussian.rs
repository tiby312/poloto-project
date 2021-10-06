// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gaussian = |sigma: f64, mu: f64| {
        move |x: f64| {
            (-0.5 * (x - mu).powi(2) / sigma.powi(2)).exp()
                * (sigma * std::f64::consts::TAU).sqrt().recip()
        }
    };

    let range: Vec<_> = (0..200)
        .map(|x| x as f64 / 200.0)
        .map(|x| x * 10.0 - 5.0)
        .collect();

    poloto::plot("gaussian", "x", "y")
        .line("σ = 1.0", range.iter().map(|&x| [x, gaussian(1.0, 0.0)(x)]))
        .line("σ = 0.5", range.iter().map(|&x| [x, gaussian(0.5, 0.0)(x)]))
        .line("σ = 0.3", range.iter().map(|&x| [x, gaussian(0.3, 0.0)(x)]))
        .ymarker(0)
        .simple_theme(poloto::upgrade_write(std::io::stdout()));
}
