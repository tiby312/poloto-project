fn main() {
    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    let mut s = poloto::plot("test", "x", "y");

    s.line("cos", x.clone().map(|x| [x, x.cos()]));

    s.scatter("sin", x.clone().map(|x| [x, x.sin()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() - 20.]));

    //Write the graph to a file
    let mut file = std::fs::File::create("assets/write_to_file.svg").unwrap();

    use std::io::Write;
    write!(file, "{}", s.render().unwrap()).unwrap();
}
