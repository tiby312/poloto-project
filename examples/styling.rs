use hypermelon::prelude::*;
use poloto::build::plot;
use poloto::prelude::*;
fn main() {
    // .poloto_plot vs .poloto_legend
    // .poloto_x vs .poloto_y
    // .poloto_imgs vs .poloto_text
    // .poloto_histo vs .poloto_line vs .poloto
    // .poloto_where
    // .poloto_names

    let theme = poloto::render::Theme::light();

    // Say you want to style the first plot and its legend image if it is a histogram.
    let theme =
        theme.append(".poloto0.poloto_histo.poloto_imgs{fill:red;stroke:black;stroke-width:2px}");
    let theme = theme.append(".poloto0.poloto_histo.poloto_imgs>*{rx:20px;ry:20px}");

    // Say you want to style the text of the first legend
    let theme = theme.append(".poloto0.poloto_legend.poloto_text{fill:blue;}");

    // Say you want to style all line plots but not legend img.
    let theme=theme.append(".poloto_line.poloto_imgs.poloto_plot{stroke:purple;stroke-width:20px;stroke-dasharray:40px}");
    let theme=theme.append(".poloto_line.poloto_imgs.poloto_legend{stroke:purple;stroke-width:10px;stroke-dasharray:10px}");

    // Say you want to style the scatter plots butn ot legend img
    let theme = theme.append(".poloto_scatter.poloto_plot{fill:purple;stroke-width:20px;}");

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let data = poloto::plots!(
        plot("sin-10")
            .histogram()
            .buffered(x.clone().step_by(3).zip_output(|x| x.sin() - 10.)),
        plot("cos").line().buffered(x.zip_output(f64::cos)),
        plot("sin-5")
            .scatter()
            .buffered(x.clone().step_by(3).zip_output(|x| x.sin() - 5.))
    );

    poloto::data(data)
        .build_and_label((
            "Demo: you can change the style of the svg file itself!",
            "x",
            "y",
        ))
        .append_to(poloto::header().append(theme))
        .render_stdout();
}
