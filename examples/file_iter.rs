use poloto::prelude::*;

use poloto::iter::*;
//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io(std::io::stdout());

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    let temp_file="temp.txt";

    //Call twice_iter to specify that you want to use the iterator twice to find
    //the right plot scale.
    s.line(
        wr!("{}os", 'c'),
        file_buffer(x.clone().map(|x| [x, x.cos()]),temp_file),
    );
    
    s.render(
        wr!("Demo: Some Trigonometry Plots {}", 5),
        wr!("This is the {} label", 'x'),
        wr!("This is the {} label", 'y'),
    )?;
    
    std::fs::remove_file(temp_file).unwrap();

    Ok(())
}
