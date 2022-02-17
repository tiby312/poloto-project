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

    let data = poloto::data::<UnixTime, _>()
        .line("", &data)
        .ymarker(0)
        .build();

    let (xmin, xmax) = (data.boundx().min, data.boundx().max);

    let (xtick, xtick_fmt) = poloto::ticks_from_default(data.boundx());

    let xtick_step = xtick_fmt.step();

    // Assume the steps are in seconds given the data we provided.
    // We are going to use a custom time format that won't work
    // if the steps were years, for example.
    assert!(xtick_step.is_seconds());

    let (ytick, ytick_fmt) = poloto::ticks_from_default(data.boundy());

    let mut plotter = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "Number of Wikipedia Articles",
            formatm!(
                "{} to {} in {}",
                xmin.datetime(timezone).format("%H:%M:%S"),
                xmax.datetime(timezone).format("%H:%M:%S"),
                xtick_step
            ),
            "Number of Articles",
            xtick_fmt
                .with_tick_fmt(|w, v| write!(w, "{}", v.datetime(timezone).format("%H:%M:%S"))),
            ytick_fmt,
        ),
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
