fn main() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let mut plotter = poloto::plot(
        poloto::ctx::f64,
        poloto::ctx::f64,
        "cows per year",
        "year",
        "cow",
    );
    plotter.scatter("", &data);

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_axis_lines{stroke:green}.poloto_tick_labels{fill:red}.poloto_labels{fill:blue}",
        poloto::disp(|a| plotter.render(a)),
        poloto::SVG_END
    )
}
