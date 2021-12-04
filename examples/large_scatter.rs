use poloto::*;
fn main() {
    let x: Vec<_> = (0..30).map(|x| (x as f64 / 30.0) * 10.0).collect();

    let mut plotter = plot("cows per year", "year", "cows");

    plotter.scatter("a", x.iter().map(|&x| (x, x.cos())));

    plotter.line("b", x.iter().map(|&x| (x, x.sin())));

    large_dot_print(poloto::upgrade_write(std::io::stdout()), plotter);
}

fn large_dot_print<W: std::fmt::Write, X: poloto::PlotNum, Y: poloto::PlotNum>(
    mut w: W,
    mut a: poloto::Plotter<X, Y>,
) {
    write!(
        &mut w,
        "{}<style>{}{}</style>",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_scatter{stroke-width:33;}.poloto_scatter.poloto_legend_icon{stroke-width:10}"
    )
    .unwrap();
    a.render(&mut w);
    write!(&mut w, "{}", poloto::SVG_END).unwrap();
}
