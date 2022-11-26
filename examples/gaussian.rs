use poloto::build;
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gau = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |x: f64| [x, (-0.5 * (x - mu).powi(2) / s).exp() * k]
    };

    let input = [(1.0, "σ=1.0"), (0.5, "σ=0.5"), (0.3, "σ=0.3")];

    let plots = input.map(|(i, name)| {
        let xs = poloto::util::range_iter([-5.0, 5.0], 200);
        build::plot(name).line(xs.map(gau(i, 0.0)))
    });

    poloto::data(poloto::plots!(build::origin(), plots))
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
