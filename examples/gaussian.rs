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

    let range = poloto::range_iter([-5.0, 5.0], 200);

    let g1 = gaussian(1.0, 0.0);
    let g2 = gaussian(0.5, 0.0);
    let g3 = gaussian(0.3, 0.0);

    use poloto::build::line;
    let l1 = line("σ = 1.0", range.clone().map(|x| [x, g1(x)]));
    let l2 = line("σ = 0.5", range.clone().map(|x| [x, g2(x)]));
    let l3 = line("σ = 0.3", range.clone().map(|x| [x, g3(x)]));

    let canvas = poloto::render::canvas();
    let mut plotter = canvas
        .build_with(plots!(l1, l2, l3), [], [0.0])
        .plot("gaussian", "x", "y");

    print!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
