fn main() {
    let mut s = splot::plot("Testing testing one two three", "this is x", "this is y");

    //let x=(0..10).map(|x|x as f32);
    //s.lines("sin", x.clone().map(|x| [x, x]));

    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);
    //let x=vec!([0.001,2.0],[0.009,4.0]);

    s.line_dotted("sin+10", x.clone().map(|x| [x, x.sin()+10.]));
    
    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.scatter("sin", x.clone().map(|x| [x, x.sin()]));
    s.histogram("sin-10", x.clone().map(|x| [x, x.sin()-10.]));

    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() -20.]));

    /*
    //s.lines("sin", x.clone().map(|x|[x,x.sin()]));

    */

    /*
    s.lines("sin", x.clone().map(|x| [x, x.sin()]));


    */
    /*
        s.line_fill("-tan", x.clone().map(|x| [x, -x.tan()]));
    */
    //PIPE me to a file!
    //s.render(std::io::stdout()).unwrap();
    //OR use this
    s.render_to_file("demo.svg").unwrap();
}
