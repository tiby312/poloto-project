
fn main() {
    let data = [
        (20, "potato"),
        (14, "broccoli"),
        (53, "pizza"),
        (30, "avocado"),
    ];
    
    poloto::build::bar::gen_simple("", data, [0])
        .label(("Comparison of Food Tastiness", "Tastiness", "Foods"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
