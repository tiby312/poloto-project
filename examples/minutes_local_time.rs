use poloto::num::timestamp::{unixtime_ticks, UnixTime};
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
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

    let mut s = s.inner.plot_with(
        "Number of Wikipedia Articles",
        "Year",
        "Number of Articles",
        unixtime_ticks(s.boundx, time_zone),
        poloto::ticks_from_default(s.boundy),
    );

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
