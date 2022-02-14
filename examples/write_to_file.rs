use poloto::prelude::*;
fn main() -> std::io::Result<()> {
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let mut s = poloto::data();
    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.scatter("sin", x.clone().map(|x| [x, x.sin()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() - 20.]));

    let mut s = s.build().plot("test", "x", "y");

    //Write the graph to a file
    let mut file = std::fs::File::create("target/assets/write_to_file.svg")?;
    use std::io::Write;
    write!(file, "{}", poloto::disp_mut(|a| s.simple_theme(a)))
}
