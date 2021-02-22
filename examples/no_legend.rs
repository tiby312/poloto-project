fn main() -> core::fmt::Result {
    let mut s = poloto::plot("Cows Per Year", "Year", "Cow");
    let data = [
    //    [1979.0, 10000.0],
    //    [1989.0, 10000.0001],
        [1979.0, 0.0],
        [1989.0, 0.0001],
    
    ];

    s.line_fill("", data.iter().map(|x| *x));

    poloto::render_svg_io(std::io::stdout(), s)?;
    Ok(())
}
