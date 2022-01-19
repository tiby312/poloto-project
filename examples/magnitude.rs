use poloto::prelude::*;
fn main() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let mut p = poloto::plot("cows per year", "year", "cow");
    p.scatter("", &data);

    println!("{}", poloto::disp(|a| { p.simple_theme(a) }));
}
