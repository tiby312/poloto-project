
use tagger::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {

    let mut buffer=String::new();

    let mut s = poloto::plot(&mut buffer);
    
    let x = (0..1).map(|x| (x as f64 / 50.0) * 10.0);
    
    let chi=5;
    {
        s.line( |w|write!(w,"chicken {}",chi), x.clone().map(|x| [x, x.cos()]));
    }
    
    s.scatter(wr!("sin"), x.clone().map(|x| [x, x.sin()]));

    s.histogram(wr!("sin-10"), x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));

    s.line_fill(wr!("sin-20"), x.clone().map(|x| [x, x.sin() - 20.]));
    
    s.render(
        wr!("Demo: Some Trigonometry Plots {}",5),
        wr!("This is the x label"),
        wr!("This is the y label"),
    )?;
    

    println!("{}",buffer);

    Ok(())
}
