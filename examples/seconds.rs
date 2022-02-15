use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    use chrono::TimeZone;

    let timezone = &chrono::Utc;

    let date = timezone.ymd(2020, 1, 30);

    let data = [
        (date.and_hms(1, 1, 59).into(), 3144000),
        (date.and_hms(1, 2, 00).into(), 3518000),
        (date.and_hms(1, 2, 30).into(), 3835000),
        (date.and_hms(1, 2, 40).into(), 2133000),
        (date.and_hms(1, 3, 00).into(), 4133000),
    ];

    let data = poloto::data::<UnixTime, _>().line("", &data).build();

    let boundx = data.boundx.clone();

    let xtick = poloto::ticks_from_default(data.boundx);

    let xtick_step = xtick.fmt.step();

    // Assume the steps are in seconds given the data we provided.
    assert_eq!(xtick_step, poloto::num::timestamp::StepUnit::SE);

    let mut plotter = data.inner.plot_with(
        "Number of Wikipedia Articles",
        formatm!(
            "{} to {} in {}",
            UnixTime::datetime(&boundx.min, timezone).format("%H:%M:%S"),
            UnixTime::datetime(&boundx.max, timezone).format("%H:%M:%S"),
            xtick_step
        ),
        "Number of Articles",
        xtick.with_tick_fmt(|w, v| write!(w, "{}", v.datetime(timezone).format("%H:%M:%S"))),
        poloto::ticks_from_default(data.boundy),
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
