use super::*;

use chrono::TimeZone;
use poloto::num::timestamp::UnixTime;

#[test]
fn days() -> fmt::Result {
    let timezone = &chrono::Utc;

    let data: &[(UnixTime, _)] = &[
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

    let p = poloto::plots!(
        poloto::build::cloned_plot(data.iter()).line(""),
        poloto::build::markers(None, Some(0))
    );

    let w = util::create_test_file("days.svg");
    poloto::data(p)
        .build_and_label(("Number of Wikipedia Articles", "Day", "Number of Articles"))
        .append_to(poloto::simple_light())
        .render_fmt_write(w)
}

#[test]
fn minutes_local_time() -> fmt::Result {
    let time_zone = &chrono::FixedOffset::east(-3600 * 5);

    let day1 = time_zone.ymd(2020, 1, 30);
    let day2 = time_zone.ymd(2020, 1, 31);
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data: &[(UnixTime, _)] = &[
        (day1.and_hms(23, 30, 59).into(), 3144000),
        (day2.and_hms(01, 02, 00).into(), 3518000),
        (day2.and_hms(01, 05, 01).into(), 3835000),
        (day2.and_hms(01, 30, 59).into(), 2133000),
        (day2.and_hms(01, 50, 01).into(), 4133000),
    ];

    let s = poloto::data(plots!(
        poloto::build::cloned_plot(data.iter()).line(""),
        poloto::build::markers(None, Some(0))
    ));

    use poloto::num::timestamp::UnixTimeTickFmt;
    let s = s.with_xticks(UnixTimeTickFmt::with_timezone(time_zone.clone()));

    let w = util::create_test_file("minutes_local_time.svg");

    s.build_and_label(("Number of Wikipedia Articles", "time", "Number of Articles"))
        .append_to(poloto::simple_dark())
        .render_fmt_write(w)
}

#[test]
fn months() -> fmt::Result {
    let timezone = &chrono::Utc;
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data: &[(UnixTime, _)] = &[
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

    let plots = poloto::plots!(
        poloto::build::cloned_plot(data.iter()).line(""),
        poloto::build::markers([], [0])
    );

    let w = util::create_test_file("months.svg");

    poloto::data(plots)
        .build_and_label((
            "Number of Wikipedia Articles",
            "duration",
            "Number of Articles",
        ))
        .append_to(poloto::simple_dark())
        .render_fmt_write(w)
}

#[cfg(feature = "timestamp_full")]
#[test]
fn seconds() -> fmt::Result {
    use chrono::TimeZone;
    use poloto::ticks::GenTickDist;
    let timezone = &chrono::Utc;

    let date = timezone.ymd(2020, 1, 30);

    let data: &[(UnixTime, _)] = &[
        (date.and_hms(1, 1, 59).into(), 3144000),
        (date.and_hms(1, 2, 00).into(), 3518000),
        (date.and_hms(1, 2, 30).into(), 3835000),
        (date.and_hms(1, 2, 40).into(), 2133000),
        (date.and_hms(1, 3, 00).into(), 4133000),
    ];

    let data = poloto::data(plots!(
        poloto::build::cloned_plot(data.iter()).line(""),
        poloto::build::markers(None, Some(0))
    ));

    let xticks = poloto::ticks::from_closure(|data, opt, req| {
        use poloto::ticks::GenTickDist;
        let k = poloto::ticks::custom_gen(poloto::default_ticks::<UnixTime>(), data, opt, req);
        let step = *k.fmt.step();

        poloto::ticks::TickDistRes::new(k.it)
            .with_ticks(|w, v| write!(w, "{}", v.datetime(timezone).format("%H:%M:%S")))
            .with_data(step)
    });

    let data = data.with_xticks(xticks);

    let data = data.build();

    let bounds = *data.boundx();
    //let j = data.xticks().fmt.data;
    let data = data.label((
        "Number of Wikipedia Articles",
        hypermelon::format_move!(
            "{} to {}",
            bounds.min.datetime(timezone).format("%H:%M:%S"),
            bounds.max.datetime(timezone).format("%H:%M:%S"),
        ),
        "Number of Articles",
    ));

    let w = util::create_test_file("seconds.svg");

    data.append_to(poloto::simple_light()).render_fmt_write(w)
}
