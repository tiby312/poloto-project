use poloto::prelude::*;
fn main() {
    let points = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let data = poloto::data();

    data.scatter("", points);

    let mut plotter = data.plot("cows per year", "year", "cow");

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_axis_lines{stroke:green}.poloto_tick_labels{fill:red}.poloto_labels{fill:blue}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
