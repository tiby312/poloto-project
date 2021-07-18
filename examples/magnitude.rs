fn main() -> core::fmt::Result {
    let data = [
        [0.000001f64, 0.000001],
        [0.000001000000001, 0.000001000000001],
    ];

    let mut s = poloto::plot("cows per year", "year", "cow");
    s.scatter("", &data);
    s.render_io(std::io::stdout())
}
