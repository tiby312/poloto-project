fn main() {
    let data = [
        [0.000001f64, 0.000001],
        [0.000001000000001, 0.000001000000001],
    ];

    let s = poloto::plot("cows per year", "year", "cow")
        .scatter("", &data)
        .render(poloto::theme_dark());
    println!("{}", s)
}
