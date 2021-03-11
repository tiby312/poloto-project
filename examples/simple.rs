use poloto::iter::*;

pub const DATA: &[[f64; 2]] = &[[0.0, 10.0], [1.0, 20.0], [2.0, 15.0]];

//PIPE me to a file!
fn main() {
    let mut s = poloto::plot("simple", "x", "y");

    s.line("data", DATA.iter().copied().twice_iter());

    s.render_io(std::io::stdout()).unwrap();
}
