fn main() {
    let data = [
        [0.000001f64, 0.000001],
        [0.000001000000001, 0.000001000000001],
    ];

    poloto::plot("cows per year", "year", "cow")
        .scatter("", &data)
        .simple_theme(&mut tagger::from_io(std::io::stdout()));
}
