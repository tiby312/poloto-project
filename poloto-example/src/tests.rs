use super::*;
use tagu::{
    prelude::*,
    stack::{ElemStackEscapable, Sentinel},
};

pub fn foo(
    w: ElemStackEscapable<'_, Sentinel>,
) -> Result<ElemStackEscapable<'_, Sentinel>, fmt::Error> {
    let mut document = support::Doc::new(w, file!())?;

    document.add(line!()).add(source!(|| {
        use poloto::build::plot;
        use poloto::prelude::*;
        use poloto::render::Theme;
        use tagu::prelude::*;
        let x: Vec<_> = (0..30).map(|x| (x as f64 / 30.0) * 10.0).collect();

        let plots = poloto::plots!(
            plot("a").scatter(x.iter().copied().zip_output(f64::cos)),
            plot("b").line(x.iter().copied().zip_output(f64::sin))
        );

        let data =
            poloto::frame_build()
                .data(plots)
                .build_and_label(("cows per year", "year", "cows"));

        let header = poloto::header().append(Theme::dark().append(tagu::build::raw(
            ".poloto_scatter.poloto_plot{stroke-width:33;}",
        )));

        data.append_to(header).render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build::plot;
        use poloto::prelude::*;
        let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

        let s = plot("tan(x)").line_fill(
            x.zip_output(f64::tan)
                .crop_above(10.0)
                .crop_below(0.0)
                .crop_left(2.0),
        );

        let data = poloto::frame_build().data(s).build_map(|data| {
            let boundx = *data.boundx();
            data.label((
                format_move!("from {} to {}", boundx.min, boundx.max),
                format_move!("This is the {} label", 'x'),
                "This is the y label",
            ))
        });

        let data = data.append_to(poloto::header().light_theme());

        data.render_string()
    }))?;


    document.add(line!()).add(source!(|| {
        use poloto::build::plot;
        use poloto::prelude::*;
        use tagu::prelude::*;
        let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

        let s = plot("tan(x)").line_fill(
            x.zip_output(f64::tan)
                .crop_above(10.0)
                .crop_below(0.0)
                .crop_left(2.0),
        );

        let data = poloto::frame_build().data(s).build_map(|data| {
            let boundx = *data.boundx();
            data.label((
                format_move!("from {} to {}", boundx.min, boundx.max),
                format_move!("This is the {} label", 'x'),
                "This is the y label",
            ))
        });

        let data = data.append_to(poloto::header().light_theme());

        data.render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build::plot;
        use poloto::render::Theme;
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
            .line((0..).zip(collatz(1000))),
            plot(format_move!(
                "c({}) The quick brown fox jumps over the lazy dog",
                1001
            ))
            .line((0..).zip(collatz(1001))),
            poloto::build::markers([], [0]),
            plot(" 🍆 Here is a note using the text() function.🍎",).text(),
            plot(format_move!(
                "c({}) The quick brown fox jumps over the lazy dog",
                1002
            ))
            .line((0..).zip(collatz(1002)))
        );

        let data = poloto::frame_build()
            .data(plots)
            .build_and_label(("collatz", "x", "y"));

        let a = [1200.0, 500.0];
        let header = poloto::header()
            .with_dim(a)
            .with_viewbox(a)
            .append(Theme::dark());

        data.append_to(header).render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build::plot;

        // Magnitude
        let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

        let d = poloto::frame_build()
            .data(plot("").scatter(data))
            .build_and_label(("cows per year", "year", "cow"))
            .append_to(poloto::header().light_theme());

        d.render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        use poloto::render::Theme;
        let points = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];

        let d = poloto::frame_build()
            .data(build::plot("").line(build::cloned(points.iter())))
            .build_and_label(("cows per year", "year", "cow"));

        let header = poloto::header().append(Theme::dark().append(
            tagu::build::raw(".poloto_axis_lines{stroke:green}.poloto_tick_labels{fill:red}.poloto_labels{fill:blue}")
        ));

        d.append_to(header).render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        use poloto::render::Theme;
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

        let v =
            (1000..1006).map(|i| build::plot(format_move!("c({})", i)).line((0..).zip(collatz(i))));

        let ddd = [2000.0, 1000.0];

        let header = poloto::header().with_dim(ddd).with_viewbox(ddd);

        let canvas = poloto::frame()
            .with_viewbox(header.get_viewbox())
            .with_tick_lines([true, true])
            .build();

        let data = canvas
            .data(poloto::plots!(
                poloto::build::markers([], [0]),
                Vec::from_iter(v)
            ))
            .build_and_label(("collatz", "x", "y"));

        let header = header.append(Theme::dark().append(tagu::build::raw(
            ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        )));

        data.append_to(header).render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        use poloto::prelude::*;
        let x: Vec<_> = (0..500).map(|x| (x as f64 / 500.0) * 10.0).collect();

        let data = poloto::plots!(
            build::plot(format_move!("test {}", 1)).line(x.iter().copied().zip_output(f64::cos)),
            build::plot(format_move!("test {}", 2)).line(x.iter().copied().zip_output(f64::sin))
        );

        poloto::frame_build()
            .data(data)
            .build_and_label(("cos per year", "year", "cows"))
            .append_to(poloto::header().dark_theme())
            .render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        use poloto::prelude::*;
        use poloto::render::Theme;
        let x: Vec<_> = (0..50).map(|x| (x as f64 / 50.0) * 10.0).collect();

        let data = poloto::plots!(
            build::plot("cos").line(x.iter().copied().zip_output(f64::cos)),
            build::plot("sin-10")
                .histogram(x.iter().copied().step_by(3).zip_output(|x| x.sin() - 10.))
        );

        let data = poloto::frame_build().data(data).build_and_label((
            "Demo: you can change the style of the svg file itself!",
            "x",
            "y",
        ));

        let header = poloto::header().append(Theme::dark().chain(tagu::build::raw_escapable(
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

        let k = header.append(data);

        let mut s = String::new();
        tagu::render_escapable(k, &mut s)?;
        Ok(s)
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        use poloto::prelude::*;
        let x: Vec<_> = (0..500).map(|x| (x as f64 / 500.0) * 10.0).collect();

        let data = poloto::frame_build()
            .data(poloto::plots!(
                build::plot("tan(x)").line(
                    x.iter()
                        .copied()
                        .zip_output(f64::tan)
                        .crop_above(10.0)
                        .crop_below(-10.0)
                        .crop_left(2.0)
                ),
                build::plot("2*cos(x)").line(
                    x.iter()
                        .copied()
                        .zip_output(|x| 2.0 * x.cos())
                        .crop_above(1.4)
                )
            ))
            .build_and_label((
                "Some Trigonometry Plots 🥳",
                format_move!("This is the {} label", 'x'),
                "This is the y label",
            ));

        data.append_to(poloto::header().light_theme())
            .render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build::PlotRes;
        use poloto::build::PlotTag;
        let v: Vec<PlotRes<std::iter::Empty<PlotTag<(i128, i128), &'static str>>, (i128, i128)>> =
            vec![];

        let data = poloto::frame_build().data(v).build_and_label((
            "Some Trigonometry Plots 🥳",
            format_move!("This is the {} label", 'x'),
            "This is the y label",
        ));

        data.append_to(poloto::header().light_theme())
            .render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build::PlotRes;
        use poloto::build::PlotTag;

        let v: Vec<PlotRes<std::iter::Empty<PlotTag<(i128, i128), &'static str>>, (i128, i128)>> =
            vec![];

        let data = poloto::frame_build()
            .data(poloto::plots!(v, poloto::build::markers([], [5])))
            .build_and_label((
                "Some Trigonometry Plots 🥳",
                format_move!("This is the {} label", 'x'),
                "This is the y label",
            ));

        data.append_to(poloto::header().light_theme())
            .render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        let p = poloto::frame_build()
            .data(poloto::plots!(
                build::plot("hay").scatter(build::cloned(std::iter::empty::<(i128, i128)>())),
                poloto::build::markers([], [5])
            ))
            .build_and_label((
                "Some Trigonometry Plots 🥳",
                format_move!("This is the {} label", 'x'),
                "This is the y label",
            ));

        p.append_to(poloto::header().light_theme()).render_string()
    }))?;

    document.add(line!()).add(source!(|| {
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

        let data = poloto::frame_build().data(poloto::plots!(
            poloto::build::plot("foo").histogram(data),
            poloto::build::markers(None, Some(0))
        ));

        let xtick_fmt = poloto::ticks::TickDistribution::new((2010..).step_by(2));

        data.map_xticks(|_| xtick_fmt)
            .build_and_label(("title", "xname", "yname"))
            .append_to(poloto::header().light_theme())
            .render_string()
    }))?;

    document.add(line!()).add(source!(|| {
        use poloto::build;

        let hr = 1000 * 60 * 60;

        //heart rate recorded in milliseconds
        let heart_rate = [
            [hr * 0, 80],
            [hr * 1, 80],
            [hr * 2, 80],
            [hr * 3 + 100, 90],
            [hr * 3 + 1000, 30],
        ];

        // Have there be a tick every hour

        let p = poloto::plots!(
            poloto::build::plot("hay").line(build::cloned(heart_rate.iter())),
            poloto::build::markers(None, Some(0))
        );

        let xticks =
            poloto::ticks::TickDistribution::new(std::iter::successors(Some(0), |w| Some(w + hr)))
                .with_tick_fmt(|&v| format_move!("{} hr", v / hr));

        let data = poloto::frame_build().data(p).map_xticks(|_| xticks);

        data.build_and_label(("collatz", "x", "y"))
            .append_to(poloto::header().dark_theme())
            .render_string()
    }))?;
    Ok(document.into_stack())
}
