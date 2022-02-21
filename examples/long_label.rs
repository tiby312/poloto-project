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

    data.text("⚠️ Here is an example note using the text() function. ⚠️");
    data.line(
        poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", 1000),
        (0..).zip(collatz(1000)),
    );
    data.line(
        poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", 1001),
        (0..).zip(collatz(1001)),
    );
    data.text(" 🍆 Here is another note using the text() function.🍎");
    data.line(
        poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", 1002),
        (0..).zip(collatz(1002)),
    );

    data.ymarker(0);

    let mut plotter = data.build().plot("collatz", "x", "y");

    // Use a width of 1200 instead of 800
    println!(
        "{}<style>{}</style>{}{}",
        poloto::disp_const(|w| poloto::simple_theme::write_header(w, 1200.0, 500.0)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}
