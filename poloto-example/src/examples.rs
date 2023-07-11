use super::*;
use tagu::{
    stack::{ElemStackEscapable, Sentinel},
};

pub fn foo(
    w: ElemStackEscapable<'_, Sentinel>,
) -> Result<ElemStackEscapable<'_, Sentinel>, fmt::Error> {
    let mut document = support::Doc::new(w, file!())?;

    document.add(line!()).add(source!(|| {
        use poloto::build;
        use tagu::prelude::*;
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

        let a = (1000..1006).map(|i| build::plot(format!("c({})", i)).line((0..).zip(collatz(i))));

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


    Ok(document.into_stack())
}
