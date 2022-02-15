// PIPE me to a file!
fn main() {
    let hr = 1000 * 60 * 60;

    //heart rate recorded in milliseconds
    let heart_rate = [
        [hr * 0, 80],
        [hr * 1, 80],
        [hr * 2, 80],
        [hr * 3 + 100, 90],
        [hr * 3 + 1000, 30],
    ];

    // Have there be a tick every hour

    let data = poloto::data().line("hay", &heart_rate).ymarker(0).build();

    let xx = std::iter::successors(Some(0), |w| Some(w + hr));

    let x = poloto::num::steps(data.boundx, xx, |w, v| write!(w, "{} hr", v / hr));
    let y = poloto::ticks_from_default(data.boundy);

    let mut plotter = data.inner.plot_with("collatz", "x", "y", x, y);

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
