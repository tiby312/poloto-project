use hypermelon::elem::Elem;
use poloto::build;
use std::fmt::Display;
pub struct Doc {
    file: &'static str,
}

pub struct Adder {
    file: &'static str,
    line: u32,
}
impl Adder {
    fn add<K: Display>(self, (program, source): (impl FnOnce() -> K, &str)) {
        let file = self.file;
        let line = self.line;
        println!("file={}:{}", file, line);
        println!("source:\n{}", source);
        let ret = program();
        println!("return:{}", ret);
    }
}

impl Doc {
    fn new(file: &'static str) -> Doc {
        Doc { file }
    }
    fn add(&mut self, line: u32) -> Adder {
        Adder {
            file: self.file,
            line,
        }
    }
}

fn main() {
    let document = &mut Doc::new(file!());

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

        let a = (1000..1006).map(|i| build::plot(format!("c({})", i)).line((0..).zip(collatz(i))));

        poloto::frame()
            .with_tick_lines([true, true])
            .with_viewbox(svg.get_viewbox())
            .build()
            .data(poloto::plots!(poloto::build::origin(), a))
            .build_and_label(("collatz", "x", "y"))
            .append_to(svg.append(style))
            .render_string()
            .unwrap()
    }));
}
