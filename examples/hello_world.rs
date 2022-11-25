use poloto::build;
// PIPE me to a file!
fn main() {
    let data = vec![[0, 0], [1, 2], [2, 3]];

    let a = build::plot("label").line2(data);

    poloto::data(a)
        .build_and_label(("hello world", "x", "y"))
        .append_to(poloto::header().light_theme())
        .render_stdout();
}
