use super::*;

#[test]
fn heart() -> fmt::Result {
    // https://mathworld.wolfram.com/HeartCurve.html
    let heart = |t: f64| {
        [
            16.0 * t.sin().powi(3),
            13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
        ]
    };

    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    let mut data = poloto::data();
    data.line_fill_raw("heart", range.map(heart));
    data.xmarker(-20.0).xmarker(20.0);
    data.ymarker(-20.0).ymarker(20.0);
    data.preserve_aspect();

    let mut plotter = data.build().plot("Heart Graph", "x", "y");

    let w = util::create_test_file("heart.svg");

    plotter.simple_theme_dark(w)
}

#[test]
fn large_scatter() -> fmt::Result {
    let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);

    let mut s = poloto::data();

    s.scatter("a", x.clone().map(|x| (x, x.cos())));

    s.line("b", x.clone().map(|x| (x, x.sin())));

    let mut plotter = s.build().plot("cows per year", "year", "cows");

    let mut w = util::create_test_file("large_scatter.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_scatter{stroke-width:33;}.poloto_scatter.poloto_legend_icon{stroke-width:10}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn line_fill_fmt() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let s = poloto::data()
        .line_fill(
            "tan(x)",
            x.clone()
                .map(|x| [x, x.tan()])
                .crop_above(10.0)
                .crop_below(0.0)
                .crop_left(2.0),
        )
        .build();

    let boundx = s.boundx().clone();

    let mut plotter = s.plot(
        formatm!("from {} to {}", boundx.min, boundx.max),
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    let w = util::create_test_file("line_fill_fmt.svg");

    plotter.simple_theme(w)
}

#[test]
// PIPE me to a file!
fn long_label() -> fmt::Result {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut data = poloto::data();

    data.text("⚠️ Here is an example note using the text() function. ⚠️");
    data.line(
        poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", 1000),
        (0..).zip(collatz(1000)),
    );
    data.line(
        poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", 1001),
        (0..).zip(collatz(1001)),
    );
    data.text(" 🍆 Here is another note using the text() function.🍎");
    data.line(
        poloto::formatm!("c({}) The quick brown fox jumps over the lazy dog", 1002),
        (0..).zip(collatz(1002)),
    );

    data.ymarker(0);

    let mut plotter = data.build().plot("collatz", "x", "y");

    let mut w = util::create_test_file("long_label.svg");

    // Use a width of 1200 instead of 800
    write!(
        w,
        "{}<style>{}</style>{}{}",
        poloto::disp_const(|w| poloto::simple_theme::write_header(w, 1200.0, 500.0)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn magnitude() -> fmt::Result {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let p = poloto::data().scatter("", &data).build();

    let mut p = p.plot("cows per year", "year", "cow");

    let w = util::create_test_file("magnitude.svg");

    p.simple_theme(w)
}

#[test]
fn base_color() -> fmt::Result {
    let points = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let data = poloto::data().scatter("", points).build();

    let mut plotter = data.plot("cows per year", "year", "cow");

    let mut w = util::create_test_file("base_color.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_axis_lines{stroke:green}.poloto_tick_labels{fill:red}.poloto_labels{fill:blue}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn custom_dim() -> fmt::Result {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut data = poloto::data();
    for i in 1000..1006 {
        data.line(poloto::formatm!("c({})", i), (0..).zip(collatz(i)));
    }
    data.ymarker(0);
    data.with_dim(2000.0, 1000.0);
    data.ytick_lines();
    data.xtick_lines();

    let mut plotter = data.build().plot("collatz", "x", "y");

    let mut w = util::create_test_file("custom_dim.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::disp_const(|w| poloto::simple_theme::write_header(w, 2000.0, 1000.0)),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn dark() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut data = poloto::data();
    data.line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()]));
    data.line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]));

    let mut plotter = data.build().plot("cos per year", "year", "cows");

    let w = util::create_test_file("dark.svg");

    plotter.simple_theme_dark(w)
}

#[test]
fn custom_style() -> fmt::Result {
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let mut s = poloto::data();

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    let mut p = s.build().plot(
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y",
    );

    let mut w = util::create_test_file("custom_style.svg");

    write!(
        w,
        "{}<style>{}</style>{}{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        r###"
    <defs>
        <pattern id="pattern2" patternUnits="userSpaceOnUse" width="10" height="10">
            <line x1="0" y1="5" x2="10" y2="5" stroke="red" stroke-width="5"/>
        </pattern> 
    </defs>
    <style>
    .poloto0stroke.poloto0stroke{
        stroke-dasharray:10 2 2;
    }
    .poloto1fill.poloto1fill{
        fill: url(#pattern2);
    }
    </style>"###,
        poloto::disp(|a| p.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn trig() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut s = poloto::data();
    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    s.line(
        "tan(x)",
        poloto::buffered_iter::buffered(
            x.clone()
                .map(|x| [x, x.tan()])
                .crop_above(10.0)
                .crop_below(-10.0)
                .crop_left(2.0),
        ),
    );

    s.line(
        "sin(2x)",
        poloto::bounded_iter::from_rect(
            [0.0, 10.0],
            [0.0, 10.0],
            x.clone().map(|x| [x, (2.0 * x).sin()]),
        ),
    );

    s.line(
        "2*cos(x)",
        poloto::buffered_iter::buffered(x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4)),
    );

    let mut plotter = s.build().plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    let w = util::create_test_file("trig.svg");
    plotter.simple_theme(w)
}