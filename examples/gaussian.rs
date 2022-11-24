use poloto::build::{plot, ClonedPlotIt, ClonedPlot1D, PlotItSOA};
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
    
    let ii=r.iter().map(gau(1.0, 0.)).map(|x|x[1]);
    let plots = poloto::plots!(
        plot("σ=1.0").line().data(PlotItSOA(ClonedPlot1D(r.iter()),ClonedPlot1D(ii))),
        plot("σ=0.5").line().data(ClonedPlotIt(r.iter().map(gau(0.5, 0.)))),
        plot("σ=0.3").line().data(ClonedPlotIt(r.iter().map(gau(0.3, 0.)))),
        poloto::build::origin()
    );

    poloto::data(plots)
        .build_and_label(("gaussian", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
