use poloto::prelude::*;
fn main() {
    let data = poloto::data()
        .bars("", [[5, 0], [4, 1], [6, 2], [10, 3]])
        .xmarker(0)
        .ymarker(-1)
        .ymarker(4)
        .xtick_lines()
        .build();

    let (xtick, xtick_fmt) = poloto::ticks_from_default(data.boundx());

    let (ytick, ytick_fmt) =
        poloto::bar::gen_bar(data.boundy(), &["potato", "chicken", "pizza", "avocado"]);

    let mut pp = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt("Stuff", "Hour", "Number of rides", xtick_fmt, ytick_fmt),
    );

    print!("{}", poloto::disp(|w| pp.simple_theme(w)));
}
