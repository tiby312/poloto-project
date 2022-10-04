// PIPE me to a file!
fn main() {
    let data = vec![[0, 0], [1, 2], [2, 3]];

    let a = poloto::build::plot("label").line().cloned(data.iter());

    poloto::data(a)
        .build_and_label(("hello world", "x", "y"))
        .append_to(poloto::simple_light())
        .render_stdout();
}
