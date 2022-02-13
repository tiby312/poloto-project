use poloto::prelude::*;
fn main() {
    let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);

    let mut s = poloto::data();

    s.scatter("a", x.clone().map(|x| (x, x.cos())));

    s.line("b", x.clone().map(|x| (x, x.sin())));

    let mut plotter = s.build().plot("cows per year", "year", "cows");

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_scatter{stroke-width:33;}.poloto_scatter.poloto_legend_icon{stroke-width:10}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    );
}
