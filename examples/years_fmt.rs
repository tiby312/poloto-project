use poloto::num::timestamp::UnixTime;

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

    let title = poloto::name_ext(|p: poloto::Data<UnixTime, i128>| {
        use poloto::plotnum::FmtFull;
        let srt = poloto::disp_const(|w| p.boundx[0].default_fmt(w, FmtFull::Short, p.stepx));
        let end = poloto::disp_const(|w| p.boundx[1].default_fmt(w, FmtFull::Short, p.stepx));
        write!(p.writer, "Entries from {} to {} in {}", srt, end, p.stepx)
    });

    //compute min and max
    let mut plotter = poloto::plot(title, "years", "entries");

    plotter.line("foo", &data);
    plotter.ymarker(0);
    plotter.no_dash_y();

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|w| plotter.render(w)),
        poloto::SVG_END
    )
}
