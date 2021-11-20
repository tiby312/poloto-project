fn main() {
    let data = [
        [0.000001f64, 0.000001],
        [0.000001000000001, 0.000001000000001],
    ];

    poloto::plot("cows per year", "year", "cow")
        .scatter("", &data)
        .yinterval_fmt(|fmt, val, step| {
            write!(fmt, "{} cows", poloto::default_val_formatter(val, step))
        })
        .xinterval_fmt(|fmt, val, step| {
            write!(fmt, "{} yr", poloto::default_val_formatter(val, step))
        })
        .simple_theme(poloto::upgrade_write(std::io::stdout()));
}
