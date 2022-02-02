use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    use chrono::TimeZone;

    let timezone = &chrono::Utc;

    let date = timezone.ymd(2020, 1, 30);

    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (date.and_hms(1, 1, 59).into(), 3144000),
        (date.and_hms(1, 2, 00).into(), 3518000),
        (date.and_hms(1, 2, 30).into(), 3835000),
        (date.and_hms(1, 2, 40).into(), 2133000),
        (date.and_hms(1, 3, 00).into(), 4133000),
    ];

    let xname = poloto::fmt::name_ext(|w, ([min, max], step), _| {
        write!(
            w,
            "{} to {} in {}",
            UnixTime::datetime(&min, timezone).format("%H:%M:%S"),
            UnixTime::datetime(&max, timezone).format("%H:%M:%S"),
            step
        )
    });

    let mut s = poloto::plot(
        "Number of Wikipedia Articles",
        xname,
        "Number of Articles",
        UnixTime::default_ctx()
            .with_tick_fmt(|w, v, _, _| write!(w, "{}", v.datetime(timezone).format("%H:%M:%S"))),
        i128::default_ctx().with_marker(0),
    );
    s.line("", &data);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
