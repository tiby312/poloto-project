fn main() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];
    
    println!(
        "{}",
        poloto::disp(|a| {
            poloto::simple_theme(
                a,
                poloto::plot("cows per year", "year", "cow")
                    .scatter("", &data)
                    .move_into(),
            )
        })
    );
}
