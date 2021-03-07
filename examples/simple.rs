use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io(std::io::stdout());

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line(wr!("{}os", 'c'), x.clone().map(|x| [x, x.cos()]).twice_iter());
    
    s.scatter(wr!("s{}n", "i"), x.clone().map(|x| [x, x.sin()]).twice_iter());

    s.histogram(
        wr!("sin-{}", 10),
        x.clone().step_by(3).map(|x| [x, x.sin() - 10.]).buffer_iter(),
    );

    s.line_fill(wr!("sin-{}", 20), x.clone().map(|x| [x, x.sin() - 20.]).buffer_iter());

    s.render(
        wr!("Demo: Some Trigonometry Plots {}", 5),
        wr!("This is the {} label", 'x'),
        wr!("This is the {} label", 'y'),
    )?;
    

    Ok(())
}
