use poloto::num::integer::Defaulti128Context;
use poloto::num::unix_timestamp::{DefaultUnixTimeContext, UnixTime};
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (2010, 3144000),
        (2011, 3518000),
        (2012, 3835000),
        (2013, 4133000),
        (2014, 4413000),
        (2015, 4682000),
        (2016, 5045000),
        (2017, 5321200),
        (2018, 5541900),
        (2019, 5773600),
        (2020, 5989400),
        (2021, 6219700),
        (2022, 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles")
        .with_xcontext(DefaultUnixTimeContext.marker(UnixTime::from_year(2025)))
        .with_ycontext(Defaulti128Context.no_dash().marker(0));

    let data = data.into_iter().map(|(a, b)| {
        let a = UnixTime::from_year(a);
        (a, b)
    });

    //UnixTime::parse_from_str(&format!("{}/1/1 00:00:00", a), "%Y/%m/%d %H:%M:%S").unwrap();

    s.histogram("", data);

    let mut st = String::new();
    use std::fmt::Write;
    write!(&mut st, "{}", poloto::disp(|a| poloto::simple_theme(a, s))).unwrap();
    println!("{}", st);
}
