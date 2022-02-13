use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    use chrono::TimeZone;

    let timezone = &chrono::Utc;

    let date = timezone.ymd(2020, 1, 30);

    let data = [
        (date.and_hms(1, 1, 59).into(), 3144000),
        (date.and_hms(1, 2, 00).into(), 3518000),
        (date.and_hms(1, 2, 30).into(), 3835000),
        (date.and_hms(1, 2, 40).into(), 2133000),
        (date.and_hms(1, 3, 00).into(), 4133000),
    ];

    let data=poloto::data().line("",&data).build();

    let xtick=data.boundx().default_tick();
    let xtick_step=xtick.step;
    let xtick=xtick.with_tick_fmt(|w, v, _, &s| {
        // Assume the steps are in seconds given the data we provided.
        assert_eq!(s, poloto::num::timestamp::StepUnit::SE);

        write!(w, "{}", v.datetime(timezone).format("%H:%M:%S"))
    });

    let mut plotter=data.plot(
        "Number of Wikipedia Articles",
        formatm!("{} to {} in {}",UnixTime::datetime(&min, timezone).format("%H:%M:%S"),UnixTime::datetime(&max, timezone).format("%H:%M:%S"),xtick_step),
        "Number of Articles"
    ).with_xtick(xtick);


    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}
