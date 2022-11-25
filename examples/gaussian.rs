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

    //TODO impl!!!
    let xs = build::BufferedPlot1D::new(poloto::util::range_iter([-5.0, 5.0], 200));
    
    let makep = |val: f64| {
        let ys = xs.iter().map(gau(val, 0.));
        build::zip(xs.bound(), build::buffered(ys))
    };

    let plots = poloto::plots!(
        build::plot("σ=1.0").line().data(makep(1.0)),
        build::plot("σ=0.5").line().data(makep(0.5)),
        build::plot("σ=0.3").line().data(makep(0.3)),
        build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
