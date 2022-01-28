use poloto::num::timestamp::UnixTimeContext;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let time_zone = &chrono::Local;
    let day1 = time_zone.ymd(2020, 1, 30);
    let day2 = time_zone.ymd(2020, 1, 31);
    use chrono::TimeZone;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (day1.and_hms(23, 30, 59).into(), 3144000),
        (day2.and_hms(1, 2, 0).into(), 3518000),
        (day2.and_hms(1, 5, 1).into(), 3835000),
        (day2.and_hms(1, 30, 59).into(), 2133000),
        (day2.and_hms(1, 50, 1).into(), 4133000),
    ];

    let mut s = poloto::plot(
        "Number of Wikipedia Articles",
        "Year",
        "Number of Articles",
        UnixTimeContext::new(time_zone),
        i128::default_ctx().with_marker(0),
    );
    s.line("", &data);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
