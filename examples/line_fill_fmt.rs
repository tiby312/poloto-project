use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let s = poloto::data()
        .line_fill(
            "tan(x)",
            x.clone()
                .map(|x| [x, x.tan()])
                .crop_above(10.0)
                .crop_below(0.0)
                .crop_left(2.0),
        )
        .build();

    let boundx = s.boundx.clone();

    let mut plotter = s.plot(
        formatm!("from {} to {}", boundx.min, boundx.max),
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
