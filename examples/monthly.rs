use chrono::TimeZone;
use poloto::num::timestamp::{month_str, StepUnit, UnixTimeContext};
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // monthly trend over one day.
    let trend: [i128; 12] = [0, 3, 5, 10, 30, 40, 25, 23, 21, 5, 4, 2];

    let timezone = &chrono::FixedOffset::east(-3600 * 5);

    let data = trend
        .into_iter()
        .zip(0..)
        .map(|(x, i)| (timezone.ymd(2020, i + 1, 1).into(), x));

    let mut s = poloto::plot(
        "Some monthly data",
        "Month",
        "Things",
        UnixTimeContext::new(timezone).with_tick_fmt(|w, v, _, &s| {
            // Assume the steps are in months given the data we provided.
            assert_eq!(s, StepUnit::MO);

            // Custom formatting if month steps is chosen.
            use chrono::Datelike;
            write!(w, "{}", month_str(v.datetime(timezone).month()))
        }),
        i128::default_ctx().with_marker(0),
    );
    s.histogram("", data);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
