use poloto::prelude::*;
fn main() -> core::fmt::Result {
    let mut s = poloto::plot("test", "x", "y");

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]).twice_iter());

    s.scatter("sin", x.clone().map(|x| [x, x.sin()]).twice_iter());
    s.histogram(
        "sin-10",
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .twice_iter(),
    );
    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() - 20.]).twice_iter());

    //Write the graph to a file
    let file = std::fs::File::create("assets/write_to_file.svg").unwrap();

    s.render_io(file)?;

    Ok(())
}
