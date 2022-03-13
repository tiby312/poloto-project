use poloto::prelude::*;
fn main() {
    let mut data = poloto::data();

    let (ytick, ytick_fmt) = poloto::bar::gen_bar(
        &mut data,
        [
            (05, "potato"),
            (03, "chicken"),
            (23, "pizza"),
            (53, "avocado"),
        ],
    );

    let data = data.xmarker(0).build();

    let canvas = poloto::gen_canvas().build();
    let (xtick, xtick_fmt) = poloto::ticks_from_default(data.boundx(), canvas.boundx());

    let mut pp = data.plot_with_canvas_ext(
        canvas,
        xtick,
        ytick,
        poloto::plot_fmt("Stuff", "Hour", "Number of rides", xtick_fmt, ytick_fmt),
    );

    print!("{}", poloto::disp(|w| pp.simple_theme(w)));
}
