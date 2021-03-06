
fn main()->std::fmt::Result{
    use tagger::prelude::*;
    let data=[
        [1.0f64,4.0],
        [2.0,5.0],
        [3.0,6.0]
    ];

    let mut buffer=String::new();

    poloto::default_svg(&mut buffer,|svg|{
        svg.elem_no_attr("style",|w|{
            write_ret!(w,"{}","<style>.poloto{--poloto_color0:purple;}</style>")
        })?;

        let mut plotter = poloto::Plotter::new(svg);
        plotter.line(wr!("cow"),data.iter().map(|&x|x));
        plotter.render_no_default_tags(
            wr!("cows per year"),
            wr!("year"),
            wr!("cows")
        )
    })?;
    
    println!("{}",buffer);
    Ok(())
}