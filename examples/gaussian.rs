use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gaussian = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |x: f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let range = (0..200).map(|x| (x as f64 / 200.0) * 10.0 - 5.0);

    let g1 = gaussian(1.0, 0.0);
    let g2 = gaussian(0.5, 0.0);
    let g3 = gaussian(0.3, 0.0);

    let l1 = poloto::build::line("σ = 1.0", range.clone().map(|x| [x, g1(x)]));
    let l2 = poloto::build::line("σ = 0.5", range.clone().map(|x| [x, g2(x)]));
    let l3 = poloto::build::line("σ = 0.3", range.clone().map(|x| [x, g3(x)]));

    let mut plotter = l1
        .chain(l2)
        .chain(l3)
        .collect_with(None, Some(0.0))
        .stage()
        .plot("gaussian", "x", "y");

    print!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
