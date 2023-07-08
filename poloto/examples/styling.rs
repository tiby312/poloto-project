use tagu::prelude::*;
use poloto::{build, prelude::OutputZip};
fn main() {
    let theme = poloto::render::Theme::light();

    // Style the first plot and its legend image if it is a histogram.
    let theme =
        theme.append(".poloto0.poloto_histo.poloto_imgs{fill:red;stroke:black;stroke-width:2px}");

    // Some attributes have to accessed directly , so use >* to select the rects directly.
    let theme = theme.append(".poloto0.poloto_histo.poloto_imgs>*{rx:20px;ry:20px}");

    // Style the text of the first legend
    let theme = theme.append(".poloto0.poloto_legend.poloto_text{fill:blue;}");

    // Style all line plots but not legend img.
    let theme = theme.append(".poloto_line.poloto_imgs.poloto_plot{stroke:purple;stroke-width:20px;stroke-dasharray:40px}");

    // Style all line plot legend imgs.
    let theme = theme.append(".poloto_line.poloto_imgs.poloto_legend{stroke:purple;stroke-width:10px;stroke-dasharray:10px}");

    // Style the scatter plots but not legend img
    let theme = theme.append(".poloto_scatter.poloto_plot{fill:purple;stroke-width:20px;}");

    // Style the scatter plots but not legend img
    let theme = theme.append(".poloto_scatter.poloto_plot{fill:purple;stroke-width:20px;}");

    // Style the xaxis name
    let theme = theme.append(
        ".poloto_name.poloto_x{fill:orange;stroke-width:20px;font-size:30px;font-style: italic;}",
    );

    // Style the background
    let theme = theme.append(".poloto_background{fill:darkslategray;}");

    // Style the text
    let theme = theme.append(".poloto_text{fill: peru;}");

    // Style the ticks
    let theme = theme.append(".poloto_imgs.poloto_ticks{stroke:springgreen;}");

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let data = poloto::plots!(
        build::plot("sin-10").histogram(x.clone().step_by(3).zip_output(|x| x.sin() - 10.)),
        build::plot("cos").line(x.clone().zip_output(|x| x.cos())),
        build::plot("sin-5").scatter(x.clone().step_by(3).zip_output(|x| x.sin() - 5.))
    );

    poloto::frame_build()
        .data(data)
        .build_and_label((
            "Demo: you can change the style of the svg file itself!",
            "x axis",
            "y axis",
        ))
        .append_to(poloto::header().append(theme))
        .render_stdout();
}
