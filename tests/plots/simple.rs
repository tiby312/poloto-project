use hypermelon::{elem::Elem, format_move};

use super::*;
use poloto::build::plot;
use poloto::render::Header;
use poloto::render::Theme;
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

    let canvas = poloto::render::render_opt().preserve_aspect().move_into();

    let plots = poloto::plots!(
        plot("").line_fill_raw().buffered(range.map(heart)),
        poloto::build::markers([-20.0, 20.0], [-20.0, 20.0])
    );

    let w = util::create_test_file("heart.svg");

    poloto::data(plots)
        .map_opt(|_| canvas)
        .build_and_label(("Heart Graph", "x", "y"))
        .append_to(poloto::header().dark_theme())
        .render_fmt_write(w)
}

#[test]
fn large_scatter() -> fmt::Result {
    let x = (0..30).map(|x| (x as f64 / 30.0) * 10.0);

    let plots = poloto::plots!(
        plot("a").scatter().buffered(x.zip_output(f64::cos)),
        plot("b").line().buffered(x.zip_output(f64::sin))
    );

    let data = poloto::data(plots).build_and_label(("cows per year", "year", "cows"));

    let header = Header::new().append(Theme::dark().append(
".poloto_scatter{stroke-width:33;}.poloto_legend.poloto_imgs.poloto_scatter{stroke-width:10}",
    ));

    let w = util::create_test_file("large_scatter.svg");

    data.append_to(header).render_fmt_write(w)
}

#[test]
fn line_fill_fmt() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let s = plot("tan(x)").line_fill().buffered(
        x.zip_output(f64::tan)
            .crop_above(10.0)
            .crop_below(0.0)
            .crop_left(2.0),
    );

    let data = poloto::data(s).build_map(|data| {
        let boundx = *data.boundx();
        data.label((
            format_move!("from {} to {}", boundx.min, boundx.max),
            format_move!("This is the {} label", 'x'),
            "This is the y label",
        ))
    });

    let data = data.append_to(poloto::header().light_theme());

    let w = util::create_test_file("line_fill_fmt.svg");

    data.render_fmt_write(w)
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

    let plots = poloto::plots!(
        plot("Some notes here").text(),
        plot(format_move!(
            "c({}) The quick brown fox jumps over the lazy dog",
            1000
        ))
        .line()
        .buffered((0..).zip(collatz(1000))),
        plot(format_move!(
            "c({}) The quick brown fox jumps over the lazy dog",
            1001
        ))
        .line()
        .buffered((0..).zip(collatz(1001))),
        poloto::build::markers([], [0]),
        plot(" 🍆 Here is a note using the text() function.🍎",).text(),
        plot(format_move!(
            "c({}) The quick brown fox jumps over the lazy dog",
            1002
        ))
        .line()
        .buffered((0..).zip(collatz(1002)))
    );

    let data = poloto::data(plots).build_and_label(("collatz", "x", "y"));

    let a = [1200.0, 500.0];
    let header = Header::new()
        .with_dim(a)
        .with_viewbox(a)
        .append(Theme::dark());

    let w = util::create_test_file("long_label.svg");

    data.append_to(header).render_fmt_write(w)
}

#[test]
fn magnitude() -> fmt::Result {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let d = poloto::data(plot("").scatter().cloned(data.iter()))
        .build_and_label(("cows per year", "year", "cow"))
        .append_to(poloto::header().light_theme());

    let w = util::create_test_file("magnitude.svg");

    d.render_fmt_write(w)
}

#[test]
fn base_color() -> fmt::Result {
    let points = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let d = poloto::data(plot("").line().cloned(points.iter())).build_and_label((
        "cows per year",
        "year",
        "cow",
    ));

    let header = Header::new().append(Theme::dark().append(
        ".poloto_axis_lines{stroke:green}.poloto_tick_labels{fill:red}.poloto_labels{fill:blue}",
    ));

    let w = util::create_test_file("base_color.svg");

    d.append_to(header).render_fmt_write(w)
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
        let l = plot(format_move!("c({})", i))
            .line()
            .buffered((0..).zip(collatz(i)));
        v.push(l);
    }

    let ddd = [2000.0, 1000.0];

    let header = Header::new().with_dim(ddd).with_viewbox(ddd);

    let canvas = poloto::render::render_opt()
        .with_viewbox(header.get_viewbox())
        .with_tick_lines([true, true])
        .move_into();

    let data = poloto::data(poloto::plots!(
        poloto::build::markers([], [0]),
        poloto::build::plots_dyn(v)
    ))
    .map_opt(|_| canvas)
    .build_and_label(("collatz", "x", "y"));

    let w = util::create_test_file("custom_dim.svg");

    let header =
        header.append(Theme::dark().append(".poloto_line{stroke-dasharray:2;stroke-width:1;}"));

    data.append_to(header).render_fmt_write(w)
}

#[test]
fn dark() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let data = poloto::plots!(
        plot(format_move!("test {}", 1))
            .line()
            .buffered(x.zip_output(f64::cos)),
        plot(format_move!("test {}", 2))
            .line()
            .buffered(x.zip_output(f64::sin))
    );

    let w = util::create_test_file("dark.svg");
    poloto::data(data)
        .build_and_label(("cos per year", "year", "cows"))
        .append_to(poloto::header().dark_theme())
        .render_fmt_write(w)
}

#[test]
fn custom_style() -> fmt::Result {
    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let data = poloto::plots!(
        plot("cos").line().buffered(x.zip_output(f64::cos)),
        plot("sin-10")
            .histogram()
            .buffered(x.clone().step_by(3).zip_output(|x| x.sin() - 10.))
    );

    let data = poloto::data(data).build_and_label((
        "Demo: you can change the style of the svg file itself!",
        "x",
        "y",
    ));

    let header = Header::new().append(Theme::dark().chain(hypermelon::build::raw_escapable(
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
    )));

    let w = util::create_test_file("custom_style.svg");

    let k = header.append(data);

    hypermelon::render_escapable(k, w)
    //data.append_to(header).render_fmt_write(w)
}

#[test]
fn trig() -> fmt::Result {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let data = poloto::data(poloto::plots!(
        plot("tan(x)").line().buffered(
            x.zip_output(f64::tan)
                .crop_above(10.0)
                .crop_below(-10.0)
                .crop_left(2.0)
        ),
        plot("2*cos(x)")
            .line()
            .buffered(x.zip_output(|x| 2.0 * x.cos()).crop_above(1.4))
    ))
    .build_and_label((
        "Some Trigonometry Plots 🥳",
        format_move!("This is the {} label", 'x'),
        "This is the y label",
    ));

    let w = util::create_test_file("trig.svg");

    data.append_to(poloto::header().light_theme())
        .render_fmt_write(w)
}

#[test]
fn no_plots() -> fmt::Result {
    let v: Vec<
        poloto::build::plot_iter_impl::SinglePlot<
            i128,
            i128,
            std::iter::Empty<(i128, i128)>,
            &'static str,
        >,
    > = vec![];

    let data = poloto::data(poloto::build::plots_dyn(v)).build_and_label((
        "Some Trigonometry Plots 🥳",
        format_move!("This is the {} label", 'x'),
        "This is the y label",
    ));

    let w = util::create_test_file("no_plots.svg");

    data.append_to(poloto::header().light_theme())
        .render_fmt_write(w)
}

#[test]
fn no_plots_only_marker() -> fmt::Result {
    let v: Vec<
        poloto::build::plot_iter_impl::SinglePlot<
            i128,
            i128,
            std::iter::Empty<(i128, i128)>,
            &'static str,
        >,
    > = vec![];

    let data = poloto::data(poloto::plots!(
        poloto::build::plots_dyn(v),
        poloto::build::markers([], [5])
    ))
    .build_and_label((
        "Some Trigonometry Plots 🥳",
        format_move!("This is the {} label", 'x'),
        "This is the y label",
    ));

    let w = util::create_test_file("no_plots_only_makrer.svg");
    data.append_to(poloto::header().light_theme())
        .render_fmt_write(w)
}

#[test]
fn one_empty_plot() -> fmt::Result {
    let p = poloto::data(poloto::plots!(
        plot("hay")
            .scatter()
            .cloned(std::iter::empty::<(i128, i128)>()),
        poloto::build::markers([], [5])
    ))
    .build_and_label((
        "Some Trigonometry Plots 🥳",
        format_move!("This is the {} label", 'x'),
        "This is the y label",
    ));

    let w = util::create_test_file("one_empty_plot.svg");

    p.append_to(poloto::header().light_theme())
        .render_fmt_write(w)
}

#[test]
fn test_cloned_cloneable() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let l1 = plot("").scatter().cloned(data.iter());
    let l2 = plot("").scatter().buffered(data.iter());
    let l = plots!(l1, l2);

    let p1 = poloto::data(l.clone()).build_and_label(("title", "x", "y"));
    let p2 = poloto::data(l).build_and_label(("title", "x", "y"));

    let mut s1 = String::new();
    let mut s2 = String::new();

    p1.headless().render_fmt_write(&mut s1).unwrap();
    p2.headless().render_fmt_write(&mut s2).unwrap();

    assert_eq!(s1, s2);
}

#[test]
fn test_single_and_chain_and_dyn_cloneable() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

    let l1 = plot("").scatter().cloned(data.iter());
    let l2 = plot("").scatter().cloned(data.iter());
    let l = plots!(l1, l2);

    let p1 = poloto::data(l.clone()).build_and_label(("title", "x", "y"));
    let p2 = poloto::data(l.clone()).build_and_label(("title", "x", "y"));

    let mut s1 = String::new();
    let mut s2 = String::new();

    p1.headless().render_fmt_write(&mut s1).unwrap();
    p2.headless().render_fmt_write(&mut s2).unwrap();

    assert_eq!(s1, s2);

    let l3 = poloto::build::plots_dyn(vec![plot("").scatter().cloned(data.iter())]);

    let l = plots!(l, l3);

    let p1 = poloto::data(l.clone()).build_and_label(("title", "x", "y"));
    let p2 = poloto::data(l).build_and_label(("title", "x", "y"));

    let mut s1 = String::new();
    let mut s2 = String::new();

    p1.headless().render_fmt_write(&mut s1).unwrap();
    p2.headless().render_fmt_write(&mut s2).unwrap();

    assert_eq!(s1, s2);
}
