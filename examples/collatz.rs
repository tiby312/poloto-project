// PIPE me to a file!
fn main() {
    let collatz = |mut a: usize| {
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

    let mut p = poloto::plot("collatz", "x", "y");

    for i in 1000..1006 {
        p.line(poloto::formatm!("c({})", i), collatz(i).enumerate().map(|(x,y)|(x as i128,y as i128)));
    }

    p.ymarker(0).simple_with_element(
        poloto::upgrade_write(std::io::stdout()),
        format_args!(
            "<style>{}{}</style>",
            poloto::STYLE_CONFIG_DARK_DEFAULT,
            ".poloto{stroke-dasharray:2;stroke-width:1;}"
        ),
    );
}
