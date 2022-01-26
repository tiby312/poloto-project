use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    let timezone = &chrono::Utc;
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd(timezone, 2020, 8, 1), 8144000),
        (UnixTime::from_ymd(timezone, 2020, 9, 30), 3144000),
        (UnixTime::from_ymd(timezone, 2020, 10, 4), 3518000),
        (UnixTime::from_ymd(timezone, 2020, 11, 1), 3835000),
        (
            UnixTime::from_ymd_hms(timezone, (2020, 11, 1), 12, 59, 59),
            2133000,
        ),
        (UnixTime::from_ymd(timezone, 2021, 1, 2), 4133000),
        (UnixTime::from_ymd(timezone, 2021, 2, 3), 4413000),
        (UnixTime::from_ymd(timezone, 2021, 3, 4), 4682000),
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
