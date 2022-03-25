use poloto::prelude::*;
fn main() {
    let data = [
        (20, "potato"),
        (14, "broccoli"),
        (53, "pizza"),
        (30, "avocado"),
    ];

    let plt = poloto::simple_bar!(
        data,
        [0],
        "Comparison of Food Tastiness",
        "Tastiness",
        "Foods"
    );

    print!("{}", poloto::disp(|w| plt.simple_theme(w)));
}
