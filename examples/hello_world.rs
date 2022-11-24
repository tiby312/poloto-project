use poloto::prelude::*;

// PIPE me to a file!
fn main() {
    let data = vec![[0, 0], [1, 2], [2, 3]];

    let a = data.iter().cloned_plot().line("label");

    poloto::data(a)
        .build_and_label(("hello world", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
