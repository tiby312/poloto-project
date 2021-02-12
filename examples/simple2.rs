//PIPE me to a file!
fn main() {
    let mut s = poloto::plot(
        "Demo: Some Trigonometry Plots",
        "This is the x label",
        "This is the y label",
    );

    let k=[-0.0000005,200000.0];
    s.line("cos",k.iter().map(|x|[*x,*x]) );

    s.render(std::io::stdout()).unwrap();
}
