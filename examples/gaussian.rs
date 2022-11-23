use poloto::build::plot;
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gau = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |&x: &f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let x: Vec<_> = poloto::util::range_iter([-5.0, 5.0], 200).collect();

    let y1: Vec<_> = x.iter().map(gau(1.0, 0.0)).collect();
    let y2: Vec<_> = x.iter().map(gau(0.5, 0.0)).collect();
    let y3: Vec<_> = x.iter().map(gau(0.3, 0.0)).collect();

    let plots = poloto::plots!(
        plot("σ=1.0").line().cloned(x.iter().zip(y1.iter())),
        plot("σ=0.5").line().cloned(x.iter().zip(y2.iter())),
        plot("σ=0.3").line().cloned(x.iter().zip(y3.iter())),
        poloto::build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
