
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).


A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

### Iterating plots twice

In order to calculate the right size view to scale all the plots, poloto has to iterate over all the plot
points twice. Once to find the min and max bounds, and once to scale all the points by the scale determined
by the first iteration. There are two options here. Either we use the same iterator twice, or we run an iterator
once and store the results to be iterated a second time. Which method to use depends a lot of how slow
the iterator function is. If the user wants to do something expensive to calculate the next plot, then
you might want to store the results. In contrast, if you iterating over values already calculated
then you might have no problem using the iterator twice. Poloto forces the user to choose which method to use
by either calling `twice_iter` or `buffer_iter` on an iterator. A third `file_buffer` is also provided that
uses a temporary file to store the iterator results.

### Passing closures instead of strings

Instead of passing strings, the user passes closures on how to make a string for things like the title, xaxis name
y axis name, and plot names. This allows us to inject formatted strings directly into the svg file as it is being
written on the fly. This allows us to avoid the dynamic allocation of calling `format!`.

### Example 

```rust
use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io(std::io::stdout());

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    //Call twice_iter to allow the iterator to be cloned and ran twice.
    s.line(
        wr!("{}os", 'c'),
        x.clone().map(|x| [x, x.cos()]).twice_iter(),
    );

    //Call `buffer_iter` to communicate that iterator results
    //should be stored to a Vec buffer for the second iteration.
    s.scatter(
        wr!("s{}n", "i"),
        x.clone().map(|x| [x, x.sin()]).buffer_iter(),
    );

    s.histogram(
        wr!("sin-{}", 10),
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .buffer_iter(),
    );

    s.line_fill(
        wr!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]).buffer_iter(),
    );

    s.with_back_color("rgba(255,255,255,0.8)");

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
