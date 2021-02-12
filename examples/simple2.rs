//PIPE me to a file!
fn main() {
    let mut s = poloto::plot(
        "Demo: Some Trigonometry Plots",
        "This is the x label",
        "This is the y label",
    );

    let k=[-0.05,0.0002];
    s.line("cos",k.iter().map(|x|[*x,*x]) );


    //Make the first line a dashed line.
    s.append(svg::node::Text::new(
        "<style>.poloto0stroke{stroke-dasharray:10}</style>",
    ));

    s.render(std::io::stdout()).unwrap();
}
