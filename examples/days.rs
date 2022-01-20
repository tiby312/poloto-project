use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd(2020, 1, 30), 3144000),
        (UnixTime::from_ymd(2020, 1, 31), 3518000),
        (UnixTime::from_ymd(2020, 2, 1), 3835000),
        (UnixTime::from_ymd_hms((2020, 2, 1), 12, 59, 59), 2133000),
        (UnixTime::from_ymd(2020, 2, 2), 4133000),
        (UnixTime::from_ymd(2020, 2, 3), 4413000),
        (UnixTime::from_ymd(2020, 2, 4), 4682000),
    ];

    let mut s = poloto::Plotter::new(
        "Number of Wikipedia Articles",
        UnixTime::ctx("Year"),
        i128::ctx("Number of Articles").no_dash().marker(0),
    )
    .line("", &data)
    .move_into();

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}