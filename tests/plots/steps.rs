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

    let p = plots!(
        poloto::build::label("hay").line().cloned(heart_rate.iter()),
        poloto::build::markers(None, Some(0))
    );

    let xticks =
        poloto::ticks::TickDistribution::new(std::iter::successors(Some(0), |w| Some(w + hr)))
            .with_ticks(|w, v| write!(w, "{} hr", v / hr));

    let data = poloto::data(p).map_xticks(|_| xticks);

    let w = util::create_test_file("marathon.svg");

    data.build_and_label(("collatz", "x", "y"))
        .append_to(poloto::simple_dark())
        .render_fmt_write(w)
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

    let data = poloto::data(plots!(
        poloto::build::label("foo").histogram().cloned(data.iter()),
        poloto::build::markers(None, Some(0))
    ));

    let xtick_fmt = poloto::ticks::TickDistribution::new((2010..).step_by(2));

    let w = util::create_test_file("years.svg");

    data.map_xticks(|_| xtick_fmt)
        .build_and_label(("title", "xname", "yname"))
        .append_to(poloto::simple_light())
        .render_fmt_write(w)
}
