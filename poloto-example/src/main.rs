use poloto::build;
use poloto::build::plot;
use poloto::prelude::*;
use poloto::render::Theme;
use shower::source;
use std::fmt;
use tagu::elem::Elem;
use tagu::elem::Locked;
use tagu::stack::ElemStackEscapable;
use tagu::stack::Sentinel;

use fmt::Write;
fn rust_to_html(source: &str) -> impl Elem {
    use synoptic::languages::rust;
    use synoptic::tokens::Token;

    let k = rust();

    // Run highlighter
    let result = k.run(source);

    //let div = hbuild::elem("div").with(("style", "overflow:auto;width:auto;padding:.2em .6em;"));
    let pre = hbuild::elem("pre").with((
        "style",
        "color:white;width:100%;overflow:scroll;margin:0;line-height:125%",
    ));

    let code = tagu::build::from_stack_escapable(move |mut stack| {
        // For each row
        for (_, row) in result.iter().enumerate() {
            for tok in row {
                // Handle the tokens
                match tok {
                    // Handle the start token (start foreground colour)
                    Token::Start(kind) => {
                        let color = match kind.as_str() {
                            "keyword" => "#ba8baf",
                            "struct" => "#f7ca88",
                            "boolean" | "number" | "global" => "#dc9656",
                            "operator" => "#d8d8d8",
                            "comment" | "reference" => "#585858",
                            "string" | "character" => "#a1b56c",
                            "function" | "macro" => "#7cafc2",
                            "regex" | "symbol" => "#86c1b9",
                            "namespace" => "#f78c6c",
                            _ => "#ffffff",
                        };

                        write!(stack.writer_escapable(), "<span style=\"color:{}\">", color)?;
                    }
                    // Handle a text token (print out the contents)
                    Token::Text(txt) => stack.writer_escapable().write_str(txt)?,
                    // Handle an end token (reset foreground colour)
                    Token::End(_) => {
                        stack.writer_escapable().write_str("</span>")?;
                    }
                }
            }

            stack.writer_escapable().write_str("\n")?;
        }

        Ok(stack)
    });

    pre.append(code).inline().with_tab("")
}

pub struct Doc<'a> {
    stack: ElemStackEscapable<'a, Sentinel>,
    file: &'static str,
}

pub struct Adder<'a, 'b> {
    doc: &'a mut Doc<'b>,
    line: u32,
}
impl<'a, 'b> Adder<'a, 'b> {
    fn add(
        self,
        (program, source): (impl FnOnce() -> Result<String, fmt::Error>, &str),
    ) -> fmt::Result {
        let file = self.doc.file;
        let line = self.line;

        let ret = poloto_evcxr::encode_string_as_img(program()?);

        let ret = hbuild::elem("div")
            .with(("style", ("overflow:scroll")))
            .append(ret);

        let line = hbuild::raw(format_move!("{}:{}", file, line)).inline();

        let line = {
            let pre = hbuild::elem("pre").with(("style", "color:white;margin:0;line-height:125%"));
            pre.append(line).with_tab("")
        };

        let s = rust_to_html(&source);

        let k2 = hbuild::elem("text")
            .with(("style", "text-indent: 0px;"))
            .append(s);

        let div =
            hbuild::elem("div").with(("style", "margin-bottom:50px;margin-left: auto;margin-right: auto;max-width:800px;width:100%;padding:10px;background:black;border-radius:15px"));

        let all = div.append(line).append(k2).append(ret);

        self.doc.stack.put(all)?;
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

use tagu::build as hbuild;
use tagu::prelude::*;

fn main() -> fmt::Result {
    let k = hbuild::from_stack_escapable(|w| {
        let mut document = Doc::new(w, file!());

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
        Ok(document.stack)
    });

    let head = hbuild::elem("head");
    //let style = hbuild::elem("style").append(include_str!("markdown.css"));

    let html = hbuild::elem("html").with(("style", "background: #2b303b;"));
    let html = html.append(head.chain(hbuild::elem("body").append(k)));
    tagu::render_escapable(html, tagu::stdout_fmt())

    //https://docs.rs/syntect/latest/syntect/html/index.html
}
