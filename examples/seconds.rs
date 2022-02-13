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

    let (xtick, xtick_fmt) = data.boundx().default_tick_generate();

    let xtick_step = xtick_fmt.step();
    // Assume the steps are in seconds given the data we provided.
    assert_eq!(xtick_step, poloto::num::timestamp::StepUnit::SE);

    let xtick_fmt =
        xtick_fmt.with_tick_fmt(|w, v| write!(w, "{}", v.datetime(timezone).format("%H:%M:%S")));

    let boundx = data.boundx();

    let y = data.boundy().default_tick_generate();

    let mut plotter = data.plot_with(
        "Number of Wikipedia Articles",
        formatm!(
            "{} to {} in {}",
            UnixTime::datetime(&boundx.min, timezone).format("%H:%M:%S"),
            UnixTime::datetime(&boundx.max, timezone).format("%H:%M:%S"),
            xtick_step
        ),
        "Number of Articles",
        (xtick, xtick_fmt),
        y,
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
