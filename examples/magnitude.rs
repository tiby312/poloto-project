use poloto::prelude::*;
fn main() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let mut p = poloto::data();
    p.scatter("", &data);

    let mut p = p.plot("cows per year", "year", "cow");

    println!("{}", poloto::disp(|a| { p.simple_theme(a) }));
}
