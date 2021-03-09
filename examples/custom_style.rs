use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut buffer = tagger::upgrade(std::io::stdout());

    let mut s = poloto::plot(&mut buffer);

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line(wr!("cos"), x.clone().map(|x| [x, x.cos()]).twice_iter());
    s.histogram(
        wr!("sin-10"),
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .twice_iter(),
    );

    s.with_raw_text(wr!(
        "{}",
        r###"
    <defs>
        <pattern id="pattern" patternUnits="userSpaceOnUse" width="10" height="10">
            <circle cx="5" cy="5" r="5" fill="black" fill-opacity="0.2"/>
        </pattern>
        <pattern id="pattern2" patternUnits="userSpaceOnUse" width="10" height="10">
            <line x1="0" y1="5" x2="10" y2="5" stroke="red" stroke-width="5"/>
            </pattern> 
    </defs>
    <style>
    .poloto_background.poloto_background{
        fill: url(#pattern);
    }
    .poloto0stroke.poloto0stroke{
        stroke-dasharray:10 2 2;
    }
    .poloto1fill.poloto1fill{
        fill: url(#pattern2);
    }
    </style>
    "###
    ));

    s.render(
        wr!("Demo: you can change the style of the svg file itself!"),
        wr!("x"),
        wr!("y"),
    )?;

    Ok(())
}
