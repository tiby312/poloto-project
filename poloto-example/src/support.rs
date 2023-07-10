use super::*;

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
    pub fn add(
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
            hbuild::elem("div").with(("style", "margin-bottom:50px;margin-left: auto;margin-right: auto;min-width:400px;max-width:800px;padding-top:15px;padding-bottom:15px;background:black;border-radius:15px"));

        let all = div.append(line).append(k2).append(ret);

        self.doc.stack.put(all)?;
        Ok(())
    }
}

impl<'a> Doc<'a> {
    pub fn new(mut stack: ElemStackEscapable<'a, Sentinel>, file: &'static str) -> Result<Doc<'a>,fmt::Error> {

        stack.put(
            hbuild::single("meta")
                .with(attrs!(
                    ("name", "viewport"),
                    ("content", "width=device-width, initial-scale=1.0")
                ))
                .with_ending(""),
        )?;


        Ok(Doc { stack, file })
    }
    pub fn add<'b>(&'b mut self, line: u32) -> Adder<'b, 'a> {
        Adder { doc: self, line }
    }
    pub fn into_stack(self)->ElemStackEscapable<'a,Sentinel>{
        self.stack
    }
}
