use poloto::build::{plot, BufferedPlot1D, ClonedPlot1D, ClonedPlotIt, PlotIt1D, Zip};
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gau = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |&x: &f64| [x, (-0.5 * (x - mu).powi(2) / s).exp() * k]
    };

    let r: Vec<_> = poloto::util::range_iter([-5.0, 5.0], 200).collect();

    let ii = r.iter().map(gau(1.0, 0.)).map(|x| x[1]);

    let c1 = ClonedPlot1D(r.iter()).zip(BufferedPlot1D(ii));

    let c2 = ClonedPlotIt(r.iter().map(gau(0.5, 0.)));
    let c3 = ClonedPlotIt(r.iter().map(gau(0.3, 0.)));

    let plots = poloto::plots!(
        plot("σ=1.0").line().data(c1),
        plot("σ=0.5").line().data(c2),
        plot("σ=0.3").line().data(c3),
        poloto::build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
