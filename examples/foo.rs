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

    let plots = poloto::plots!(
        poloto::build::plots_dyn((1000..1006).map(|i| {
            let name = hypermelon::format_move!("c({})", i);
            let it = (0..).zip(collatz(i));
            poloto::build::buffered_plot(it).line(name)
        }),),
        poloto::build::origin()
    );

    let steps = poloto::ticks::from_closure(|_, _, _| {
        poloto::ticks::TickBuilder::new((0..).step_by(6)).build()
    });

    poloto::data(plots)
        .with_xticks(steps)
        .build("title", "x", "y")
        .append_to(poloto::simple_light())
        .render_stdout();
}
