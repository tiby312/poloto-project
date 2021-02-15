//PIPE me to a file!
fn main() {


    let mut s = poloto::plot_io(std::io::stdout())
    .finish(
        "Demo: Some Trigonometry Plots",
        "This is the x label",
        "This is the y label"
    );


    /*
    let s=poloto::default_svg();
    s.write_str("<style>.poloto0stroke{stroke-dasharray:10}</style>\n");
    let mut s = poloto::from_element(s).finish(
        "Demo: Some Trigonometry Plots",
        "This is the x label",
        "This is the y label"
    );
    */

    use std::fmt::Write;
    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    s.line(
        "cos",
        x.clone().map(|x| [x, x.cos()])
    );
    s.render();
    /*
    s.scatter("sin", x.clone().map(|x| [x, x.sin()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() - 20.]));
    */
    //s.render();
    
}
