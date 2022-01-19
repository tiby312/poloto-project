use poloto::num::unix_timestamp::UnixTime;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let day1 = (2020, 1, 30);
    let day2 = (2020, 1, 31);
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd_hms(day1, 23, 30, 59), 3144000),
        (UnixTime::from_ymd_hms(day2, 1, 2, 0), 3518000),
        (UnixTime::from_ymd_hms(day2, 1, 5, 1), 3835000),
        (UnixTime::from_ymd_hms(day2, 1, 30, 59), 2133000),
        (UnixTime::from_ymd_hms(day2, 1, 50, 1), 4133000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles")
        .with_ycontext(poloto::ctx::<i128>().no_dash().marker(0));

    s.line("", data);

    let mut st = String::new();
    use std::fmt::Write;
    write!(&mut st, "{}", poloto::disp(|a| poloto::simple_theme(a, s))).unwrap();
    println!("{}", st);
}
