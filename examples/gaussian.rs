use poloto::build::plot;
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

    let r: Vec<_> = poloto::util::range_iter([-5.0, 5.0], 200).collect();

    let it1 = r.iter().copied().zip_output(gau(1.0, 0.));
    let it2 = r.iter().copied().zip_output(gau(0.5, 0.));
    let it3 = r.iter().copied().zip_output(gau(0.3, 0.));

    let plots = poloto::plots!(
        plot("σ=1.0").line().buffered(it1),
        plot("σ=0.5").line().buffered(it2),
        plot("σ=0.3").line().buffered(it3),
        poloto::build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
