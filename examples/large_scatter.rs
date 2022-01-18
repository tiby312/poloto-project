use poloto::*;
fn main() {
    let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);

    let mut plotter = plot(
        poloto::ctx::f64,
        poloto::ctx::f64,
        "cows per year",
        "year",
        "cows",
    );

    plotter.scatter("a", x.clone().map(|x| (x, x.cos())));

    plotter.line("b", x.clone().map(|x| (x, x.sin())));

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_scatter{stroke-width:33;}.poloto_scatter.poloto_legend_icon{stroke-width:10}",
        poloto::disp(|a| plotter.render(a)),
        poloto::SVG_END
    );
}
