use chrono::TimeZone;
use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let timezone = &chrono::Utc;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (timezone.ymd(2020, 1, 30).into(), 3144000),
        (timezone.ymd(2020, 1, 31).into(), 3518000),
        (timezone.ymd(2020, 2, 1).into(), 3835000),
        (timezone.ymd(2020, 2, 1).and_hms(12, 59, 59).into(), 2133000),
        (timezone.ymd(2020, 2, 2).into(), 4133000),
        (timezone.ymd(2020, 2, 3).into(), 4413000),
        (timezone.ymd(2020, 2, 4).into(), 4682000),
    ];

    let mut s = poloto::plot(
        "Number of Wikipedia Articles",
        "Year",
        "Number of Articles",
        UnixTime::default_ctx(),
        i128::default_ctx().with_marker(0),
    );
    s.line("", &data);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
