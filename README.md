
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).


A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. 

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

Here is a simple demo:

### Example 

```rust
//PIPE me to a file!
fn main() {
    let mut s = poloto::plot(
        "Demo: Some Trigonometry Plots",
        "This is the x label",
        "This is the y label",
    );

    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    s.line("cos", x.clone().map(|x| [x, x.cos()]));
    s.scatter("sin", x.clone().map(|x| [x, x.sin()]));
    s.histogram("sin-10", x.clone().step_by(3).map(|x| [x, x.sin() - 10.]));
    s.line_fill("sin-20", x.clone().map(|x| [x, x.sin() - 20.]));

    //Make the first line a dashed line.
    s.append(svg::node::Text::new(
        "<style>.poloto0stroke{stroke-dasharray:10}</style>",
    ));

    s.render(std::io::stdout()).unwrap();
}
```

### Output


<img src="./assets/simple.svg" alt="demo">
