use chrono::TimeZone;
use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
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

    let mut s = s.plot("Number of Wikipedia Articles", "Year", "Number of Articles");

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
