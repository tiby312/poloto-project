fn main() {
    let mut s = splot::plot("Testing testing one two three", "this is x", "this is y");

    let x = (0..10).map(|x| (x as f32 / 10.0) * std::f32::consts::TAU);

    s.lines("sin", x.clone().map(|x| [x, x.sin()]));

    s.scatter("cos", x.clone().map(|x| [x, x.cos()]));

    s.histogram("tan", x.clone().map(|x| [x, x.sin()]));
    /*
        s.line_fill("-tan", x.clone().map(|x| [x, -x.tan()]));
    */
    //PIPE me to a file!
    s.render(std::io::stdout()).unwrap();
    //OR use this
    //s.render_to_file("demo.svg").unwrap();
}
