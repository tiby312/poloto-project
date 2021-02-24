fn main() -> core::fmt::Result {
    let mut s = poloto::plot("Cows Per Year", "Year", "Cow");

    // TEST 3
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    s.scatter("", data.iter().map(|x| *x));

    poloto::render_svg_io(std::io::stdout(), s)?;
    Ok(())
}
