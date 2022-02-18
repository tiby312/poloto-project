// PIPE me to a file!
fn main() {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut data = poloto::data();
    for i in 1000..1006 {
        data.line(poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", i), (0..).zip(collatz(i)));
    }
    data.ymarker(0);

    let mut plotter = data.build().plot("collatz", "x", "y");

    // Use a width of 1200 instead of 800
    println!(
        "{}<style>{}</style>{}{}",
        r##"<svg class="poloto" width="1200" height="500" viewBox="0 0 1200 500" xmlns="http://www.w3.org/2000/svg">"##,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
