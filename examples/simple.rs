use poloto::prelude::*;

//PIPE me to a file!
fn main() {
    let data = vec![
        [1850.0, 1.0],
        [1940.0, 20.0],
        [1945.0, 15.0],
        [1989.0, 7.0],
        [1995.0, 20.0],
        [2007.0, 14.0],
    ];

    let mut s = poloto::plot("simple", "x", "y");

    s.line_fill("data", data.twice_iter());

    s.render_io(std::io::stdout()).unwrap();
}
