use poloto::prelude::*;
fn main() {
    let (bar, ybound, ytick, ytick_fmt) = poloto::build::bar::gen_bar(
        "",
        [
            (20, "potato"),
            (14, "broccoli"),
            (53, "pizza"),
            (30, "avocado"),
        ],
    );

    let canvas = poloto::render::canvas()
        .with_tick_lines(true, false)
        .build();

    let data = bar.collect_with([0], ybound).stage_with(&canvas);

    let (xtick, xtick_fmt) = poloto::ticks::from_default(data.bounds().0);

    let mut plt = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "Comparison of Food Tastiness",
            "Tastiness",
            "Foods",
            xtick_fmt,
            ytick_fmt,
        ),
    );

    print!("{}", poloto::disp(|w| plt.simple_theme(w)));
}
