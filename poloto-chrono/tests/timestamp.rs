use chrono::{NaiveDate, TimeZone};
use poloto::build;
use poloto::prelude::*;
use poloto_chrono::UnixTime;
use std::fmt;
#[test]
fn days() -> fmt::Result {
    let timezone = &chrono::Utc;

    let data: &[(UnixTime, _)] = &[
        (
            timezone
                .with_ymd_and_hms(2020, 1, 30, 0, 0, 0)
                .unwrap()
                .into(),
            3144000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 1, 31, 0, 0, 0)
                .unwrap()
                .into(),
            3518000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 2, 01, 0, 0, 0)
                .unwrap()
                .into(),
            3835000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 2, 01, 12, 59, 59)
                .unwrap()
                .into(),
            2133000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 2, 02, 0, 0, 0)
                .unwrap()
                .into(),
            4133000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 2, 03, 0, 0, 0)
                .unwrap()
                .into(),
            4413000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 2, 04, 0, 0, 0)
                .unwrap()
                .into(),
            4682000,
        ),
    ];

    let p = poloto::plots!(
        poloto::build::plot("").line(build::cloned(data.iter())),
        poloto::build::markers(None, Some(0))
    );

    let w = create_test_file("days.svg");
    poloto::data(p)
        .build_and_label(("Number of Wikipedia Articles", "Day", "Number of Articles"))
        .append_to(poloto::header().light_theme())
        .render_fmt_write(w)
}

#[test]
fn minutes_local_time() -> fmt::Result {
    let time_zone = &chrono::FixedOffset::east_opt(-3600 * 5).unwrap();

    let day1 = NaiveDate::from_ymd_opt(2020, 1, 30).unwrap();
    let day2 = NaiveDate::from_ymd_opt(2020, 1, 31).unwrap();

    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data: &[(UnixTime, _)] = &[
        (
            time_zone
                .from_local_datetime(&day1.and_hms_opt(23, 30, 59).unwrap())
                .latest()
                .unwrap()
                .into(),
            3144000,
        ),
        (
            time_zone
                .from_local_datetime(&day2.and_hms_opt(01, 02, 00).unwrap())
                .latest()
                .unwrap()
                .into(),
            3518000,
        ),
        (
            time_zone
                .from_local_datetime(&day2.and_hms_opt(01, 05, 01).unwrap())
                .latest()
                .unwrap()
                .into(),
            3835000,
        ),
        (
            time_zone
                .from_local_datetime(&day2.and_hms_opt(01, 30, 59).unwrap())
                .latest()
                .unwrap()
                .into(),
            2133000,
        ),
        (
            time_zone
                .from_local_datetime(&day2.and_hms_opt(01, 50, 01).unwrap())
                .latest()
                .unwrap()
                .into(),
            4133000,
        ),
    ];

    let s = poloto::data(plots!(
        poloto::build::plot("").line(data),
        poloto::build::markers(None, Some(0))
    ));

    use poloto_chrono::UnixTimeTickFmt;
    let s = s.map_xticks(|_| UnixTimeTickFmt::with_timezone(time_zone.clone()));

    let w = create_test_file("minutes_local_time.svg");

    s.build_and_label(("Number of Wikipedia Articles", "time", "Number of Articles"))
        .append_to(poloto::header().dark_theme())
        .render_fmt_write(w)
}

#[test]
fn months() -> fmt::Result {
    let timezone = &chrono::Utc;
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data: &[(UnixTime, _)] = &[
        (
            timezone
                .with_ymd_and_hms(2020, 08, 01, 0, 0, 0)
                .unwrap()
                .into(),
            8144000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 09, 30, 0, 0, 0)
                .unwrap()
                .into(),
            3144000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 10, 04, 0, 0, 0)
                .unwrap()
                .into(),
            3518000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 11, 01, 0, 0, 0)
                .unwrap()
                .into(),
            3835000,
        ),
        (
            timezone
                .with_ymd_and_hms(2020, 11, 01, 12, 59, 59)
                .unwrap()
                .into(),
            2133000,
        ),
        (
            timezone
                .with_ymd_and_hms(2021, 01, 02, 0, 0, 0)
                .unwrap()
                .into(),
            4133000,
        ),
        (
            timezone
                .with_ymd_and_hms(2021, 02, 03, 0, 0, 0)
                .unwrap()
                .into(),
            4413000,
        ),
        (
            timezone
                .with_ymd_and_hms(2021, 03, 04, 0, 0, 0)
                .unwrap()
                .into(),
            4682000,
        ),
    ];

    let plots = poloto::plots!(
        poloto::build::plot("").line(data),
        poloto::build::markers([], [0])
    );

    let w = create_test_file("months.svg");

    poloto::data(plots)
        .build_and_label((
            "Number of Wikipedia Articles",
            "duration",
            "Number of Articles",
        ))
        .append_to(poloto::header().dark_theme())
        .render_fmt_write(w)
}

#[test]
fn seconds() -> fmt::Result {
    use hypermelon::format_move;

    use chrono::TimeZone;
    let timezone = &chrono::Utc;

    let date = NaiveDate::from_ymd_opt(2020, 1, 30).unwrap();

    let data = [
        (1, 1, 59, 3144000),
        (1, 2, 00, 3518000),
        (1, 2, 30, 3835000),
        (1, 2, 40, 2133000),
        (1, 3, 00, 4133000),
    ];

    let data: &[(UnixTime, _)] = &data.map(|(a, b, c, d)| {
        (
            timezone
                .from_local_datetime(&date.and_hms_opt(a, b, c).unwrap())
                .latest()
                .unwrap()
                .into(),
            d,
        )
    });

    let data = poloto::data(plots!(
        poloto::build::plot("").line(data),
        poloto::build::markers(None, Some(0))
    ));

    let data = data.map_xticks(|g| {
        poloto::ticks::from_closure(|data, canvas, opt| {
            let k = poloto::ticks::gen_ticks(g, data, canvas, opt);
            let step = *k.fmt.step();
            poloto::ticks::from_iter(k.iter)
                .with_tick_fmt(|&v| format_move!("{}", v.datetime(timezone).format("%H:%M:%S")))
                .with_data(step)
        })
    });

    let data = data.build_map(|data| {
        let bounds = *data.boundx();
        let j = data.xticks().fmt.data;
        data.label((
            "Number of Wikipedia Articles",
            hypermelon::format_move!(
                "{} to {} with {}",
                bounds.min.datetime(timezone).format("%H:%M:%S"),
                bounds.max.datetime(timezone).format("%H:%M:%S"),
                j
            ),
            "Number of Articles",
        ))
    });

    let w = create_test_file("seconds.svg");

    data.append_to(poloto::header().light_theme())
        .render_fmt_write(w)
}

pub fn create_test_file(filename: &str) -> hypermelon::tools::Adaptor<std::fs::File> {
    std::fs::create_dir_all("../target/assets/test_timestamp").unwrap();
    let file =
        std::fs::File::create(format!("../target/assets/test_timestamp/{}", filename)).unwrap();
    hypermelon::tools::upgrade_write(file)
}
