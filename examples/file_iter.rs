use poloto::iter::*;
//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot("Demo: Some Trigonometry Plots", "x", "y");

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let temp_file = "temp.txt";

    //Use a temporary file to store the plots
    s.line(
        "cos",
        file_buffer(x.clone().map(|x| [x, x.cos()]), temp_file),
    );

    s.render_io(std::io::stdout())?;

    std::fs::remove_file(temp_file).unwrap();

    Ok(())
}
