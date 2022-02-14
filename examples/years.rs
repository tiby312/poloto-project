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

    let data = poloto::data().histogram("foo", data).ymarker(0).build();

    let x = data
        .boundx().from_gen(poloto::steps((2010..).step_by(2), |w, v| write!(w, "{} yr", v)));

    let y = data.boundy().default_gen().with_no_dash();

    let mut plotter = data.plot_with("title", "xname", "yname", x, y);

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp_mut(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}
