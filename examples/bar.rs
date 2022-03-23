use poloto::prelude::*;
fn main() {
    let (bar, ybound, ytick_fmt) = poloto::build::bar::gen_bar(
        "",
        [
            (20, "potato"),
            (14, "broccoli"),
            (53, "pizza"),
            (30, "avocado"),
        ],
    );

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, false])
        .build();

    let data = opt.build(bar.markers([0], ybound));

    let xtick_fmt = poloto::ticks::from_default(data.bounds().0);

    let plt = data.plot_with(poloto::plot_fmt(
        "Comparison of Food Tastiness",
        "Tastiness",
        "Foods",
        xtick_fmt,
        ytick_fmt,
    ));

    print!("{}", poloto::disp(|w| plt.simple_theme(w)));
}
