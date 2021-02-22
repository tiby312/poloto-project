//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot(
        "Demo: Some Trigonometry Plots",
        "This is the x label",
        "This is the y label",
    );

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));

    s.scatter("sin", x.clone().map(|x| [x, x.sin()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() - 20.]));

    poloto::render_svg_io(std::io::stdout(), s)?;

    Ok(())
}
