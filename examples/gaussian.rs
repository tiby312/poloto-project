use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gauss = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |x: f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let range = poloto::range_iter([-5.0, 5.0], 200);

    let l1 = range.zip_output(gauss(1.0, 0.)).line("σ = 1.0");
    let l2 = range.zip_output(gauss(0.5, 0.)).line("σ = 0.5");
    let l3 = range.zip_output(gauss(0.3, 0.)).line("σ = 0.3");
    let l4 = poloto::build::origin();

    let p = quick_fmt!("gaussian", "x", "y", l1, l2, l3, l4);

    print!("{}", poloto::disp(|w| p.simple_theme(w)));
}
