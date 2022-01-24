use poloto::prelude::*;

//PIPE me to a file!
fn main() {
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let mut s = poloto::plot(
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y",
        f64::ctx(),
        f64::ctx(),
    );

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    println!(
        "{}<style>{}</style>{}{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        r###"
    <defs>
        <pattern id="pattern2" patternUnits="userSpaceOnUse" width="10" height="10">
            <line x1="0" y1="5" x2="10" y2="5" stroke="red" stroke-width="5"/>
        </pattern> 
    </defs>
    <style>
    .poloto0stroke.poloto0stroke{
        stroke-dasharray:10 2 2;
    }
    .poloto1fill.poloto1fill{
        fill: url(#pattern2);
    }
    </style>"###,
        poloto::disp(|a| s.render(a)),
        poloto::simple_theme::SVG_END
    );
}
