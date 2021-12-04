// PIPE me to a file!
fn main() {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            //Base case
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut plotter = poloto::plot("collatz", "x", "y");

    plotter.ymarker(0);

    for i in 1000..1006 {
        plotter.line(poloto::formatm!("c({})", i), (0..).zip(collatz(i)));
    }

    dashed_print(poloto::upgrade_write(std::io::stdout()), plotter);
}

fn dashed_print<W: std::fmt::Write, X: poloto::PlotNum, Y: poloto::PlotNum>(
    mut w: W,
    mut a: poloto::Plotter<X, Y>,
) {
    write!(
        &mut w,
        "{}<style>{}{}</style>",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto{stroke-dasharray:2;stroke-width:1;}"
    )
    .unwrap();
    a.render(&mut w);
    write!(&mut w, "{}", poloto::SVG_END).unwrap();
}
