use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_with_html(
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y",
        style_string(),
    );

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]).twice_iter());
    s.histogram(
        "sin-10",
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .twice_iter(),
    );

    s.render_io(std::io::stdout())?;

    Ok(())
}

fn style_string() -> String {
    let mut html: String = poloto::HTML_CONFIG_LIGHT_DEFAULT.into();
    html.push_str(
        r###"
    <defs>
        <pattern id="pattern" patternUnits="userSpaceOnUse" width="50" height="50">
            <circle cx="25" cy="25" r="25" fill="black" fill-opacity="0.2"/>
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
    "###,
    );
    html
}
