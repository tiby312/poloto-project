use poloto::prelude::*;
fn main() {
    let (bars, ytick, ytick_fmt) = poloto::bar::gen_bar(
        "",
        [
            (20, "potato"),
            (14, "broccoli"),
            (53, "pizza"),
            (30, "avocado"),
        ],
    );

    let data = bars.xmarker(0).collect();
    
    let canvas = poloto::canvas().xtick_lines().build();

    let (xtick, xtick_fmt) = poloto::ticks_from_default(data.boundx(&canvas));

    let mut plt = data.stage_with(&canvas).plot_with(
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
