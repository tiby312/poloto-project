use poloto::prelude::*;
fn main() -> core::fmt::Result {
    let file = std::fs::File::create("assets/write_to_file.svg").unwrap();

    let mut s = poloto::plot_io(file);

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line(wr!("cos"), x.clone().map(|x| [x, x.cos()]).twice_iter());

    s.scatter(wr!("sin"), x.clone().map(|x| [x, x.sin()]).twice_iter());
    s.histogram(
        wr!("sin-10"),
        x.clone().step_by(3).map(|x| [x, x.sin() - 10.]).twice_iter(),
    );
    s.line_fill(wr!("sin-20"), x.clone().map(|x| [x, x.sin() - 20.]).twice_iter());

    s.render(
        wr!("Demo: Some Trigonometry Plots"),
        wr!("This is the x label"),
        wr!("This is the y label"),
    )
    .map(|_| ())
}
