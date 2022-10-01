fn main() {
    let data = [
        (20, "potato"),
        (14, "broccoli"),
        (53, "pizza"),
        (30, "avocado"),
    ];

    poloto::build::bar::gen_simple("", data, [0])
        .labels("Comparison of Food Tastiness", "Tastiness", "Foods")
        .simple_theme_dark()
        .render_stdout();
}
