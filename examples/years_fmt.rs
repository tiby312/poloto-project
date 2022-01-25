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

    let xname = poloto::fmt::name_ext(|w, ([min, max], xs), _| {
        //Use dynamic or static formatting
        let srt = UnixTime::dynamic_format(&min, xs);
        let end = UnixTime::format(&max, "%Y");
        write!(w, "Entries from {} to {} in {}", srt, end, xs)
    });

    let mut plotter = poloto::plot(
        "title",
        xname,
        "yname",
        UnixTime::ctx().with_tick_fmt(|w, v, _b, s| write!(w, "{} yr", v.dynamic_format(s))),
        i128::ctx().with_no_dash().with_marker(0).with_no_dash(),
    );

    plotter.line("foo", &data);

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}
