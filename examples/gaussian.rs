use poloto::build;

// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gau = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |&x: &f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let xs: Vec<_> = poloto::util::range_iter([-5.0, 5.0], 200).collect();

    let y1: Vec<_> = xs.iter().map(gau(1.0, 0.0)).collect();
    let y2: Vec<_> = xs.iter().map(gau(0.5, 0.0)).collect();
    let y3: Vec<_> = xs.iter().map(gau(0.3, 0.0)).collect();

    let plots = poloto::plots!(
        build::plot("σ=1.0").line().cloned(xs.iter().zip(y1.iter())),
        build::plot("σ=0.5").line().cloned(xs.iter().zip(y2.iter())),
        build::plot("σ=0.3").line().cloned(xs.iter().zip(y3.iter())),
        build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
