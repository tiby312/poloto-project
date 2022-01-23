use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    let day1 = (2020, 1, 30);
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd_hms(day1, 1, 1, 59), 3144000),
        (UnixTime::from_ymd_hms(day1, 1, 2, 0), 3518000),
        (UnixTime::from_ymd_hms(day1, 1, 2, 30), 3835000),
        (UnixTime::from_ymd_hms(day1, 1, 2, 40), 2133000),
        (UnixTime::from_ymd_hms(day1, 1, 3, 0), 4133000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");
    s.line("", &data);
    s.yaxis().marker(0);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
