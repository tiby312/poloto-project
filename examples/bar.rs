use poloto::prelude::*;
fn main() {
    let mut data = poloto::data();

    let (ytick, ytick_fmt) = poloto::bar::gen_bar(
        &mut data,
        [
            (20, "potato"),
            (10, "broccoli"),
            (53, "pizza"),
            (30, "avocado"),
        ],
    );

    let data = data.xmarker(0).build();

    let canvas = poloto::canvas().xtick_lines().build();

    let (xtick, xtick_fmt) = data.default_ticks_x(&canvas);

    let mut plt = data.plot_with_ticks_and_canvas(
        canvas,
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
