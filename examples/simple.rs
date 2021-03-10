use poloto::prelude::*;


//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot(
        "Demo: Some Trigonometry Plots",
        wr2!("This is the {} label", 'x'),
        "This is the y label"
    );

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    //Call twice_iter to allow the iterator to be cloned and ran twice.
    s.line(
        "cos",
        x.clone().map(|x| [x, x.cos()]).twice_iter(),
    );

    //Call `buffer_iter` to communicate that iterator results
    //should be stored to a Vec buffer for the second iteration.
    s.scatter(
        "sin",
        x.clone().map(|x| [x, x.sin()]).buffer_iter(),
    );

    s.histogram(
        wr2!("sin-{}", 10),
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .buffer_iter(),
    );

    s.line_fill(
        wr2!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]).buffer_iter(),
    );


    s.with_text(wr2!(
        "<style>{}</style>",
        ".poloto_background{fill:rgba(200,255,200,0.8);}"
    ));

    s.render_io(
        std::io::stdout()
    )?;

    Ok(())
}
