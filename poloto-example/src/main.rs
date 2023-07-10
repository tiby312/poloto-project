use poloto::build;
use poloto::build::plot;
use poloto::prelude::*;
use poloto::render::Theme;
use shower::source;
use std::fmt;
use tagu::elem::Elem;
use tagu::stack::ElemStackEscapable;
use tagu::stack::Sentinel;

use fmt::Write;

use tagu::build as hbuild;
use tagu::prelude::*;

mod support;
use support::Doc;



fn main() -> fmt::Result {
    let k = hbuild::from_stack_escapable(|w| {
        let mut document = Doc::new(w, file!())?;

        document.add(line!()).add(source!(|| {
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

            let svg = poloto::header().with_viewbox_width(1200.0);

            let style = poloto::render::Theme::dark().append(tagu::build::raw(
                ".poloto_line{stroke-dasharray:2;stroke-width:2;}",
            ));

            let a =
                (1000..1006).map(|i| build::plot(format!("c({})", i)).line((0..).zip(collatz(i))));

            poloto::frame()
                .with_tick_lines([true, true])
                .with_viewbox(svg.get_viewbox())
                .build()
                .data(poloto::plots!(poloto::build::origin(), a))
                .build_and_label(("collatz", "x", "y"))
                .append_to(svg.append(style))
                .render_string()
        }))?;

        document.add(line!()).add(source!(|| {
            let x: Vec<_> = (0..30).map(|x| (x as f64 / 30.0) * 10.0).collect();

            let plots = poloto::plots!(
                plot("a").scatter(x.iter().copied().zip_output(f64::cos)),
                plot("b").line(x.iter().copied().zip_output(f64::sin))
            );

            let data = poloto::frame_build().data(plots).build_and_label((
                "cows per year",
                "year",
                "cows",
            ));

            let header = poloto::header().append(Theme::dark().append(tagu::build::raw(
                ".poloto_scatter.poloto_plot{stroke-width:33;}",
            )));

            data.append_to(header).render_string()
        }))?;

        document.add(line!()).add(source!(|| {
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
        Ok(document.into_stack())
    });

    let head = hbuild::elem("head");
    //let style = hbuild::elem("style").append(include_str!("markdown.css"));

    let html = hbuild::elem("html").with(("style", "background: #2b303b;"));
    let html = html.append(head.chain(hbuild::elem("body").with(("style","margin:0px;padding:0px;")).append(k)));
    tagu::render_escapable(html, tagu::stdout_fmt())

    //https://docs.rs/syntect/latest/syntect/html/index.html
}
