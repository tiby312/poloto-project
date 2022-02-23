use super::*;

use chrono::TimeZone;
use poloto::num::timestamp::{unixtime_ticks, UnixTime};

#[test]
fn days() -> fmt::Result {
    let timezone = &chrono::Utc;

    let data = [
        (timezone.ymd(2020, 1, 30).into(), 3144000),
        (timezone.ymd(2020, 1, 31).into(), 3518000),
        (timezone.ymd(2020, 2, 01).into(), 3835000),
        (
            timezone.ymd(2020, 2, 01).and_hms(12, 59, 59).into(),
            2133000,
        ),
        (timezone.ymd(2020, 2, 02).into(), 4133000),
        (timezone.ymd(2020, 2, 03).into(), 4413000),
        (timezone.ymd(2020, 2, 04).into(), 4682000),
    ];

    let s = poloto::data::<UnixTime, _>()
        .line("", data)
        .ymarker(0)
        .build();

    let mut s = s.plot("Number of Wikipedia Articles", "Day", "Number of Articles");

    let mut w = util::create_test_file("days.svg");

    write!(w, "{}", poloto::disp(|a| s.simple_theme(a)))
}

#[test]
fn minutes_local_time() -> fmt::Result {
    let time_zone = &chrono::FixedOffset::east(-3600 * 5);

    let day1 = time_zone.ymd(2020, 1, 30);
    let day2 = time_zone.ymd(2020, 1, 31);
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (day1.and_hms(23, 30, 59).into(), 3144000),
        (day2.and_hms(01, 02, 00).into(), 3518000),
        (day2.and_hms(01, 05, 01).into(), 3835000),
        (day2.and_hms(01, 30, 59).into(), 2133000),
        (day2.and_hms(01, 50, 01).into(), 4133000),
    ];

    let s = poloto::data::<UnixTime, _>()
        .line("", &data)
        .ymarker(0)
        .build();

    let (xtick, xtick_fmt) = unixtime_ticks(s.boundx(), time_zone);
    let (ytick, ytick_fmt) = poloto::ticks_from_default(s.boundy());

    let mut s = s.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "Number of Wikipedia Articles",
            "time",
            "Number of Articles",
            xtick_fmt,
            ytick_fmt,
        ),
    );

    let w = util::create_test_file("minutes_local_time.svg");

    s.simple_theme(w)
}

#[test]
fn months() -> fmt::Result {
    let timezone = &chrono::Utc;
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (timezone.ymd(2020, 08, 01).into(), 8144000),
        (timezone.ymd(2020, 09, 30).into(), 3144000),
        (timezone.ymd(2020, 10, 04).into(), 3518000),
        (timezone.ymd(2020, 11, 01).into(), 3835000),
        (
            timezone.ymd(2020, 11, 01).and_hms(12, 59, 59).into(),
            2133000,
        ),
        (timezone.ymd(2021, 01, 02).into(), 4133000),
        (timezone.ymd(2021, 02, 03).into(), 4413000),
        (timezone.ymd(2021, 03, 04).into(), 4682000),
    ];

    let s = poloto::data::<UnixTime, _>()
        .line("", &data)
        .ymarker(0)
        .build();

    let mut s = s.plot(
        "Number of Wikipedia Articles",
        "duration",
        "Number of Articles",
    );

    let w = util::create_test_file("months.svg");

    s.simple_theme(w)
}

#[test]
fn seconds() -> fmt::Result {
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

    let w = util::create_test_file("seconds.svg");

    plotter.simple_theme(w)
}
