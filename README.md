
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).


A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

### Example 

```rust
use tagger::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io(std::io::stdout());

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    s.line(wr!("{}os", 'c'), x.clone().map(|x| [x, x.cos()]));

    s.scatter(wr!("s{}n", "i"), x.clone().map(|x| [x, x.sin()]));

    s.histogram(
        wr!("sin-{}", 10),
        x.clone().step_by(3).map(|x| [x, x.sin() - 10.]),
    );

    s.line_fill(wr!("sin-{}", 20), x.clone().map(|x| [x, x.sin() - 20.]));

    s.render(
        wr!("Demo: Some Trigonometry Plots {}", 5),
        wr!("This is the {} label", 'x'),
        wr!("This is the {} label", 'y'),
    )?;

    Ok(())
}

```

### Output


<img src="./assets/simple.svg" alt="demo">
