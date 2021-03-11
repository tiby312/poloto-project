use poloto::prelude::*;

//PIPE me to a file!
fn main() {
    let data = vec![
        [0.0, 1.0],
        [1.0, 20.0],
        [2.0, 15.0],
        [3.0, 7.0],
        [5.0, 20.0],
        [6.0, 14.0],
    ];

    let mut s = poloto::plot("simple", "x", "y");

    s.line_fill("data", data.twice_iter());

    s.render_io(std::io::stdout()).unwrap();
}
