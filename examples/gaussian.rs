use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gau = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |x: f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let r = poloto::range_iter([-5.0, 5.0], 200);
    let a = r.zip_output(gau(1.0, 0.)).buffered_plot().line("σ=1.0");
    let b = r.zip_output(gau(0.5, 0.)).buffered_plot().line("σ=0.5");
    let c = r.zip_output(gau(0.3, 0.)).buffered_plot().line("σ=0.3");
    let d = poloto::build::origin();

    let plots = quick_fmt!("gaussian", "x", "y", a, b, c, d);

    poloto::simple_stdout(plots)
}
