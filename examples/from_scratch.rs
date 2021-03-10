use poloto::prelude::*;
fn main() -> std::fmt::Result {
    


    let mut buffer = String::new();

    let mut root=tagger::Element::new(&mut buffer);
    root.elem("svg", |writer| {
        let mut svg = writer.write(|w| {
            poloto::default_tags::default_svg_attrs()(w)?;

            Ok(w)
        })?;

        poloto::default_tags::default_styling(&mut svg,"white","black",[
            "red";poloto::default_tags::NUM_COLORS
        ])?;

        let mut plotter = poloto::Plotter::with_no_svg_style_tags(svg);

        let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

        plotter.line(wr!("test1"), x.clone().map(|x| [x,x.cos()]).twice_iter());
    
        plotter.line(wr!("test1"), x.clone().map(|x| [x,x.sin()]).twice_iter());
    
        
        plotter.render(wr!("cows per year"), wr!("year"), wr!("cows"))

    })?;
    
    println!("{}", buffer);
    Ok(())
}
