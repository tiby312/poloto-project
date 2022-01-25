use poloto::num::timestamp::{UnixTime, UnixTimeContext};
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let day1 = (2020, 1, 30);
    let day2 = (2020, 1, 31);
    let time_zone = &chrono::Local;

    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd_hms(time_zone, day1, 23, 30, 59), 3144000),
        (UnixTime::from_ymd_hms(time_zone, day2, 1, 2, 0), 3518000),
        (UnixTime::from_ymd_hms(time_zone, day2, 1, 5, 1), 3835000),
        (UnixTime::from_ymd_hms(time_zone, day2, 1, 30, 59), 2133000),
        (UnixTime::from_ymd_hms(time_zone, day2, 1, 50, 1), 4133000),
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
