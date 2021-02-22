fn main() -> core::fmt::Result {
    let mut s = poloto::plot("Cows Per Year", "Year", "Cow");
    let data = [
        [1979.0, 0.0001],
        [1989.0, 0.00008],
        [2001.0, 0.00005],
        [2010.0, 0.00001],
        [2014.0, 0.00005],
        [2020.0, 0.00003],
    ];

    s.line_fill("", data.iter().map(|x| *x));

    poloto::render_svg_io(std::io::stdout(), s)?;
    Ok(())
}
