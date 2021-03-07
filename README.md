
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).


A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

### Features

In order to calculate the right size view to view all the plots, poloto has to iterate over all the plot
points twice. Once to find the min and max bounds, and once to scale all the points by the scale determined
by the first iteration. There are two options here, either use the same iterator twice, or run an iterator
once and stores the results to be iterated a second time. Which method to use depends a lot of how slow
the iterator function is. If the user wants to do something expensive to calculate the next plot, then
you might want to stores the results. In contrast, if you iterating over values already calculated
then you might have no problem using the iterator twice. Poloto forces the user to choose which method to use
by either calling `twice_iter` or `buffer_iter` on an iterator.


### Example 

```rust
use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot_io(std::io::stdout());

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    //Call twice_iter to specify that you want to use the iterator twice to find
    //the right plot scale.
    s.line(wr!("{}os", 'c'), x.clone().map(|x| [x, x.cos()]).twice_iter());
    
    //Call `buffer_iter` to specify that you want poloto to save the iterator results
    //to a Vec buffer to find the right plot scale.
    s.scatter(wr!("s{}n", "i"), x.clone().map(|x| [x, x.sin()]).buffer_iter());

    s.histogram(
        wr!("sin-{}", 10),
        x.clone().step_by(3).map(|x| [x, x.sin() - 10.]).buffer_iter(),
    );

    s.line_fill(wr!("sin-{}", 20), x.clone().map(|x| [x, x.sin() - 20.]).buffer_iter());

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
