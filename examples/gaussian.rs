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
    let l1 = line("σ = 1.0", range.zip_output(g1));
    let l2 = line("σ = 0.5", range.zip_output(g2));
    let l3 = line("σ = 0.3", range.zip_output(g3));
    let og = poloto::build::origin();

    let p = quick_fmt!("gaussian", "x", "y", l1, l2, l3, og);

    print!("{}", poloto::disp(|w| p.simple_theme(w)));
}
