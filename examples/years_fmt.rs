use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_year(2010), 3144000),
        (UnixTime::from_year(2011), 3518000),
        (UnixTime::from_year(2012), 3835000),
        (UnixTime::from_year(2013), 4133000),
        (UnixTime::from_year(2014), 4413000),
        (UnixTime::from_year(2015), 4682000),
        (UnixTime::from_year(2016), 5045000),
        (UnixTime::from_year(2017), 5321200),
        (UnixTime::from_year(2018), 5541900),
        (UnixTime::from_year(2019), 5773600),
        (UnixTime::from_year(2020), 5989400),
        (UnixTime::from_year(2021), 6219700),
        (UnixTime::from_year(2022), 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    let mut s = poloto::Plotter::new(
        "Number of Wikipedia Articles",
        "Year",
        "Number of Articles",
        UnixTime::ctx()
            .marker(UnixTime::from_year(2025))
            .with_fmt(|mut w, v, tt| {
                poloto::num::timestamp::write_fmt(&mut w, v, tt)?;
                write!(w, " yr")
            }),
        i128::ctx().no_dash().marker(0),
    );

    s.histogram("", &data);

    let mut st = String::new();
    use std::fmt::Write;
    write!(&mut st, "{}", poloto::disp(|a| poloto::simple_theme(a, s))).unwrap();
    println!("{}", st);
}
