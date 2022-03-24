use poloto::prelude::*;
fn main() {
    let (bar, ytick_fmt) = poloto::build::bar::gen_bar(
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

    let data = poloto::data(bar.markers([0], []));

    let (bx, _) = poloto::ticks::bounds(&data, &opt);

    let xtick_fmt = poloto::ticks::from_default(bx);

    let plt = poloto::plot_with(
        data,
        opt,
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
