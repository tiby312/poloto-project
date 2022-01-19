use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd(2020, 8, 1), 8144000),
        (UnixTime::from_ymd(2020, 9, 30), 3144000),
        (UnixTime::from_ymd(2020, 10, 4), 3518000),
        (UnixTime::from_ymd(2020, 11, 1), 3835000),
        (UnixTime::from_ymd_hms((2020, 11, 1), 12, 59, 59), 2133000),
        (UnixTime::from_ymd(2021, 1, 2), 4133000),
        (UnixTime::from_ymd(2021, 2, 3), 4413000),
        (UnixTime::from_ymd(2021, 3, 4), 4682000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles")
        .with_ycontext(i128::ctx().no_dash().marker(0))
        .line("", &data)
        .move_into();

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
