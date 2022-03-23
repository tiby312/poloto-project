use super::*;

#[test]
fn marathon() -> fmt::Result {
    let hr = 1000 * 60 * 60;

    //heart rate recorded in milliseconds
    let heart_rate = [
        [hr * 0, 80],
        [hr * 1, 80],
        [hr * 2, 80],
        [hr * 3 + 100, 90],
        [hr * 3 + 1000, 30],
    ];

    // Have there be a tick every hour

    let opt = poloto::render::render_opt();

    let data = poloto::data(
        poloto::build::line("hay", &heart_rate).markers(None, Some(0)),
        opt,
    );

    let xtick_fmt = poloto::ticks::from_iter(std::iter::successors(Some(0), |w| Some(w + hr)));

    let ytick_fmt = poloto::ticks::from_default(data.bounds().1);

    let plotter = data.plot_with(poloto::plot_fmt(
        "collatz",
        "x",
        "y",
        xtick_fmt.with_tick_fmt(|w, v| write!(w, "{} hr", v / hr)),
        ytick_fmt,
    ));

    let mut w = util::create_test_file("marathon.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn years() -> fmt::Result {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (2010, 3144000),
        (2011, 3518000),
        (2012, 3835000),
        (2013, 4133000),
        (2014, 4413000),
        (2015, 4682000),
        (2016, 5045000),
        (2017, 5321200),
        (2018, 5541900),
        (2019, 5773600),
        (2020, 5989400),
        (2021, 6219700),
        (2022, 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    let opt = poloto::render::render_opt();

    let data = poloto::data(
        poloto::build::histogram("foo", data).markers(None, Some(0)),
        opt,
    );

    let xtick_fmt = poloto::ticks::from_iter((2010..).step_by(2));

    let ytick_fmt = poloto::ticks::from_default(data.bounds().1);

    let plotter = data.plot_with(poloto::plot_fmt(
        "title", "xname", "yname", xtick_fmt, ytick_fmt,
    ));

    let mut w = util::create_test_file("years.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}
