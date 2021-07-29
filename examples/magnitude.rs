fn main() {
    let data = [
        [0.000001f64, 0.000001],
        [0.000001000000001, 0.000001000000001],
    ];

    let s =
        poloto::simple_render_dark(poloto::plot("cows per year", "year", "cow").scatter("", &data));
    println!("{}", s)
}
