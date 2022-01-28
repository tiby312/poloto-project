use chrono::TimeZone;
use poloto::num::timestamp::UnixTimeContext;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    //fake hourly trend over one day.
    let trend: [i128; 24] = [
        0, 0, 0, 0, 0, 3, 5, 5, 10, 20, 50, 60, 70, 50, 40, 34, 34, 20, 10, 20, 10, 4, 2, 0,
    ];

    let timezone = &chrono::Local;

    let data = trend
        .into_iter()
        .zip(0..)
        .map(|(x, i)| (timezone.ymd(2020, 1, 30).and_hms(i, 0, 0).into(), x));

    let mut s = poloto::plot(
        "Number of rides at theme park hourly",
        "Hour",
        "Number of rides",
        UnixTimeContext::new(timezone).with_tick_fmt(|w, v, _, _| {
            use chrono::Timelike;
            write!(w, "{}", v.datetime(timezone).hour())
        }),
        i128::default_ctx().with_marker(0),
    );
    s.histogram("", data);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
