use poloto::build;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gau = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |&x: &f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let r: Vec<_> = poloto::util::range_iter([-5.0, 5.0], 200).collect();

    let k1: Vec<_> = r.iter().map(gau(1.0, 0.)).collect();
    let k2: Vec<_> = r.iter().map(gau(0.5, 0.)).collect();
    let k3: Vec<_> = r.iter().map(gau(0.3, 0.)).collect();

    let plots = poloto::plots!(
        r.iter().zip(k1.iter()).cloned_plot_soa().line("σ=1.0"),
        r.iter().zip(k2.iter()).cloned_plot_soa().line("σ=0.5"),
        r.iter().zip(k3.iter()).cloned_plot_soa().line("σ=0.3"),
        build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
