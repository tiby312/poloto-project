use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    let timezone = &chrono::Utc;
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (timezone.ymd(2020, 8, 1).into(), 8144000),
        (timezone.ymd(2020, 9, 30).into(), 3144000),
        (timezone.ymd(2020, 10, 4).into(), 3518000),
        (timezone.ymd(2020, 11, 1).into(), 3835000),
        (
            timezone.ymd(2020, 11, 1).and_hms(12, 59, 59).into(),
            2133000,
        ),
        (timezone.ymd(2021, 1, 2).into(), 4133000),
        (timezone.ymd(2021, 2, 3).into(), 4413000),
        (timezone.ymd(2021, 3, 4).into(), 4682000),
    ];

    let mut s = poloto::plot(
        "Number of Wikipedia Articles",
        "duration",
        "Number of Articles",
        UnixTime::default_ctx(),
        i128::default_ctx().with_marker(0),
    );
    s.line("", &data);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
