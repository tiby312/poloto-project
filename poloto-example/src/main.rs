use hypermelon::elem::Elem;
use hypermelon::elem::Locked;
use hypermelon::stack::ElemStack;
use hypermelon::stack::ElemStackEscapable;
use hypermelon::stack::Sentinel;
use poloto::build;
use poloto::build::plot;
use poloto::prelude::*;
use poloto::render::Theme;
use std::fmt;
pub struct Doc<'a> {
    stack: ElemStackEscapable<'a, Sentinel>,
    file: &'static str,
}

pub struct Adder<'a, 'b> {
    doc: &'a mut Doc<'b>,
    line: u32,
}
impl<'a, 'b> Adder<'a, 'b> {
    fn add<K: Elem + Locked>(self, (program, source): (impl FnOnce() -> K, &str)) -> fmt::Result {
        let file = self.doc.file;
        let line = self.line;
        let ret = program();
        let k1 =
            hbuild::elem("text").append(hbuild::raw(format_move!("{}:{}", file, line)).inline());

        let ss = format!("```\n{}\n```", source);
        let parser = pulldown_cmark::Parser::new(&ss);
        let mut s = String::new();
        pulldown_cmark::html::push_html(&mut s, parser);

        let k2 = hbuild::elem("text")
            .with(("class", "markdown-body"))
            .append(hbuild::raw_escapable(s));
        self.doc.stack.put(k1)?;
        self.doc.stack.put(k2)?;

        self.doc.stack.put(ret)?;
        Ok(())
    }
}

impl<'a> Doc<'a> {
    fn new(stack: ElemStackEscapable<'a, Sentinel>, file: &'static str) -> Doc<'a> {
        Doc { stack, file }
    }
    fn add<'b>(&'b mut self, line: u32) -> Adder<'b, 'a> {
        Adder { doc: self, line }
    }
}

use hypermelon::build as hbuild;
use hypermelon::prelude::*;

fn main() -> fmt::Result {
    let k = hbuild::from_stack_escpable(|w| {
        let mut document = Doc::new(w, file!());

        document.add(line!()).add(shower::source!(|| {
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

            let style = poloto::render::Theme::dark()
                .append(".poloto_line{stroke-dasharray:2;stroke-width:2;}");

            let a =
                (1000..1006).map(|i| build::plot(format!("c({})", i)).line((0..).zip(collatz(i))));

            poloto::frame()
                .with_tick_lines([true, true])
                .with_viewbox(svg.get_viewbox())
                .build()
                .data(poloto::plots!(poloto::build::origin(), a))
                .build_and_label(("collatz", "x", "y"))
                .append_to(svg.append(style))
        }))?;

        document.add(line!()).add(shower::source!(|| {
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

            let header = poloto::header()
                .append(Theme::dark().append(".poloto_scatter.poloto_plot{stroke-width:33;}"));

            data.append_to(header)
        }))?;

        document.add(line!()).add(shower::source!(|| {
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

            data
        }))?;
        Ok(document.stack)
    });

    let head = hbuild::elem("head");
    let style = hbuild::elem("style").append(include_str!("markdown.css"));

    let html = hbuild::elem("html").with(("style", "background: black;"));
    let html = html.append(head.append(style).chain(hbuild::elem("body").append(k)));
    hypermelon::render_escapable(html, hypermelon::stdout_fmt())
}
