use poloto::build;
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

    let r = poloto::util::range_iter([-5.0, 5.0], 200);
    let a = build::buffered_plot(r.zip_output(gau(1.0, 0.))).line("σ=1.0");
    let b = build::buffered_plot(r.zip_output(gau(0.5, 0.))).line("σ=0.5");
    let c = build::buffered_plot(r.zip_output(gau(0.3, 0.))).line("σ=0.3");
    let d = build::origin();

    poloto::data(plots!(a, b, c, d))
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::simple_light())
        .render_stdout();
}
