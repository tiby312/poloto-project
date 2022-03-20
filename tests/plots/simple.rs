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

    let canvas = poloto::render::canvas_builder().preserve_aspect().build();

    let plotter = poloto::simple_fmt!(
        canvas,
        poloto::build::line_fill_raw("", range.map(heart)).markers([-20.0, 20.0], [-20.0, 20.0]),
        "Heart Graph",
        "x",
        "y"
    );

    let w = util::create_test_file("heart.svg");

    plotter.simple_theme_dark(w)
}

#[test]
fn large_scatter() -> fmt::Result {
    let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);

    let plots = plots!(
        poloto::build::scatter("a", x.clone().map(|x| (x, x.cos()))),
        poloto::build::line("b", x.clone().map(|x| (x, x.sin())))
    );

    let plotter = poloto::simple_fmt!(plots, "cows per year", "year", "cows");
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

    let canvas = poloto::render::canvas();
    let s = canvas.build(poloto::build::line_fill(
        "tan(x)",
        x.clone()
            .map(|x| [x, x.tan()])
            .crop_above(10.0)
            .crop_below(0.0)
            .crop_left(2.0),
    ));

    let boundx = s.bounds().0.data.clone();

    let fmt = poloto::plot_fmt(
        formatm!("from {} to {}", boundx.min, boundx.max),
        formatm!("This is the {} label", 'x'),
        "This is the y label",
        poloto::ticks::from_default(s.bounds().0),
        poloto::ticks::from_default(s.bounds().1),
    );

    let plotter = s.plot_with(fmt);

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

    let plots = plots!(
        poloto::build::text("Some notes here"),
        poloto::build::line(
            formatm!("c({}) The quick brown fox jumps over the lazy dog", 1000),
            (0..).zip(collatz(1000)),
        ),
        poloto::build::line(
            formatm!("c({}) The quick brown fox jumps over the lazy dog", 1001),
            (0..).zip(collatz(1001)),
        ),
        poloto::build::text(" 🍆 Here is a note using the text() function.🍎",),
        poloto::build::line(
            formatm!("c({}) The quick brown fox jumps over the lazy dog", 1002),
            (0..).zip(collatz(1002)),
        )
    );

    let plotter = poloto::simple_fmt!(plots.markers([], [0]), "collatz", "x", "y");

    let mut w = util::create_test_file("long_label.svg");

    // Use a width of 1200 instead of 800
    write!(
        w,
        "{}<style>{}</style>{}{}",
        poloto::disp_const(|w| poloto::simple_theme::write_header(
            w,
            [1200.0, 500.0],
            [1200.0, 500.0]
        )),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn magnitude() -> fmt::Result {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let p = poloto::simple_fmt!(
        poloto::build::scatter("", &data),
        "cows per year",
        "year",
        "cow"
    );

    let w = util::create_test_file("magnitude.svg");

    p.simple_theme(w)
}

#[test]
fn base_color() -> fmt::Result {
    let points = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let plotter = poloto::simple_fmt!(
        poloto::build::scatter("", points),
        "cows per year",
        "year",
        "cow"
    );

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

    let mut v = vec![];
    for i in 1000..1006 {
        let l = poloto::build::line(formatm!("c({})", i), (0..).zip(collatz(i)));
        v.push(l);
    }

    let canvas = poloto::render::canvas_builder()
        .with_dim([2000.0, 1000.0])
        .with_tick_lines([true, true])
        .build();

    let plotter = poloto::simple_fmt!(
        canvas,
        poloto::build::plots_dyn(v).markers([], [0]),
        "collatz",
        "x",
        "y"
    );

    let mut w = util::create_test_file("custom_dim.svg");

    write!(
        w,
        "{}<style>{}{}</style>{}{}",
        poloto::disp_const(|w| poloto::simple_theme::write_header(
            w,
            [2000.0, 1000.0],
            [2000.0, 1000.0]
        )),
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

#[test]
fn dark() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let plots = plots!(
        poloto::build::line(formatm!("test {}", 1), x.clone().map(|x| [x, x.cos()])),
        poloto::build::line(formatm!("test {}", 2), x.clone().map(|x| [x, x.sin()]),)
    );

    let plotter = poloto::simple_fmt!(plots, "cos per year", "year", "cows");

    let w = util::create_test_file("dark.svg");

    plotter.simple_theme_dark(w)
}

#[test]
fn custom_style() -> fmt::Result {
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let plots = plots!(
        poloto::build::line("cos", x.clone().map(|x| [x, x.cos()])),
        poloto::build::histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]))
    );

    let p = poloto::simple_fmt!(
        plots,
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y"
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

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    let data = plots!(
        poloto::build::line(
            "tan(x)",
            poloto::build::buffered_iter::buffered(
                x.clone()
                    .map(|x| [x, x.tan()])
                    .crop_above(10.0)
                    .crop_below(-10.0)
                    .crop_left(2.0),
            ),
        ),
        poloto::build::line(
            "sin(2x)",
            poloto::build::bounded_iter::from_rect(
                [0.0, 10.0],
                [0.0, 10.0],
                x.clone().map(|x| [x, (2.0 * x).sin()]),
            ),
        ),
        poloto::build::line(
            "2*cos(x)",
            poloto::build::buffered_iter::buffered(
                x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
            ),
        )
    );

    let plotter = poloto::simple_fmt!(
        data,
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label"
    );

    let w = util::create_test_file("trig.svg");
    plotter.simple_theme(w)
}

#[test]
fn no_plots() -> fmt::Result {
    let v: Vec<poloto::build::SinglePlot<std::iter::Empty<(i128, i128)>, &'static str>> = vec![];

    let plotter = poloto::simple_fmt!(
        poloto::build::plots_dyn(v),
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label"
    );

    let w = util::create_test_file("no_plots.svg");
    plotter.simple_theme(w)
}

#[test]
fn no_plots_only_marker() -> fmt::Result {
    let v: Vec<poloto::build::SinglePlot<std::iter::Empty<(i128, i128)>, &'static str>> = vec![];

    let plotter = poloto::simple_fmt!(
        poloto::build::plots_dyn(v).markers([], [5]),
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label"
    );

    let w = util::create_test_file("no_plots_only_makrer.svg");
    plotter.simple_theme(w)
}

#[test]
fn one_empty_plot() -> fmt::Result {
    let l = poloto::build::scatter("hay", std::iter::empty::<(i128, i128)>());

    let plotter = poloto::simple_fmt!(
        l.markers([], [5]),
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label"
    );

    let w = util::create_test_file("one_empty_plot.svg");
    plotter.simple_theme(w)
}
