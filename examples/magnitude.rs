fn main() {
    //let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];
    //let data = [[-1000000000000, 0]];
    //let data = [[1000000000000, 0]];
    let data = [[32, 32]];

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
