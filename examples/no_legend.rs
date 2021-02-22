fn main() -> core::fmt::Result {
    let mut s = poloto::plot("Cows Per Year", "Year", "Cow");
    /*
    // TEST 1
    let data=[
        [1979.0, 1000000000.0]
    ];

    // TEST 2
    let data=[
        [1979.0, 10000000.0],
        [1979.0, -10000001.0]
    ];
    */

    // TEST 3
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    s.scatter("", data.iter().map(|x| *x));

    poloto::render_svg_io(std::io::stdout(), s)?;
    Ok(())
}
