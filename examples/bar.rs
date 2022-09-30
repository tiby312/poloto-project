fn main() {
    let data = [
        (20, "potato"),
        (14, "broccoli"),
        (53, "pizza"),
        (30, "avocado"),
    ];

    let (plots, ytick_fmt) = poloto::build::bar::gen_bar("", data, [0]);

    let opt = poloto::render::render_opt_builder()
        .with_tick_lines([true, false])
        .build();

    poloto::data(plots)
        .with_yticks(ytick_fmt)
        .with_opt(opt)
        .labels("Comparison of Food Tastiness", "Tastiness", "Foods")
        .simple_theme()
        .render_stdout()
}
