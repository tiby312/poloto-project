// PIPE me to a file!
fn main() -> std::fmt::Result {
    let data = [
        [1850.0f64, 10.0],
        [1940.0, 12.0],
        [1945.0, 12.2],
        [1989.0, 16.0],
        [2001.0, 20.0],
    ];

    let mut s = poloto::plot("simple", "x", "y");

    s.line_fill("", &data);

    s.render_io(std::io::stdout())
}
