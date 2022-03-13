use poloto::prelude::*;
fn main() {
    let mut data = poloto::data();

    let (ytick, ytick_fmt) = poloto::bar::gen_bar(
        &mut data,
        [
            ("potato", 5),
            ("chicken", 3),
            ("pizza", 23),
            ("avocado", 53),
        ],
    );

    let data = data.xmarker(0).build();

    let (xtick, xtick_fmt) = poloto::ticks_from_default(data.boundx());

    let mut pp = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt("Stuff", "Hour", "Number of rides", xtick_fmt, ytick_fmt),
    );

    print!("{}", poloto::disp(|w| pp.simple_theme(w)));
}
