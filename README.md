
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).


A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

## Iterating plots twice

In order to calculate the right size view to scale all the plots, poloto has to iterate over all the plot
points twice. Once to find the min and max bounds, and once to scale all the points by the scale determined
by the first iteration. There are two options here. Either we use the same iterator twice, or we run an iterator
once and store the results to be iterated a second time. Which method to use depends a lot of how slow
the iterator function is. If the user wants to do something expensive to calculate the next plot, then
you might want to store the results. In contrast, if you are iterating over values already calculated
then you might have no problem using the iterator twice. Poloto forces the user to choose which method to use
by either calling `twice_iter` or `buffer_iter` on an iterator. A third `file_buffer` is also provided that
uses a temporary file to store the iterator results.

## Formatting Tick Intervals

Poloto will first print intervals in normal decimal at the precision required to capture the differences
in the step size between the intervals. If the magnitude of a number is detected to be too big or small, it
may switch to scientific notation, still at the required precision. It will only switch if the scientific
notation version is actually less characters than the normal decimal format which is not always the case
when you consider the precision that might be required to capture the step size.

Even with the above system, there are cases where the numbers all have a really big magnitude, but
are all really close together (small step size). In this case, there isn't really a good way to format it.
In these cases, poloto will fall back to making the number relative to the first number.


## Can I change the styling of the plots?

Yes! You can harness the power of CSS both in the svg, or outside
in html with an embeded svg. Some things you can do:

 * Change the color scheme to fit your html theme.
 * Highlight one plot, make it dashed, or add hover effect
 * Animate things using @keyframes

Depending on whether you are adding a new style attribute or overriding
an existing one, you might have to increase the specificty of your css clause to make sure it overrides
the svg css clause.

## Example 

```rust
use poloto::prelude::*;

//PIPE me to a file!
fn main() -> core::fmt::Result {
    let mut s = poloto::plot(
        "Demo: Some Trigonometry Plots",
        move_format!("This is the {} label", 'x'),
        "This is the y label",
    );

    let x = (0..50).map(|x| (x as f64 / 50.0) * 10.0);

    //Call twice_iter to allow the iterator to be cloned and ran twice.
    s.line("cos", x.clone().map(|x| [x, x.cos()]).twice_iter());

    //Call `buffer_iter` to communicate that iterator results
    //should be stored to a Vec buffer for the second iteration.
    s.scatter("sin", x.clone().map(|x| [x, x.sin()]).buffer_iter());

    s.histogram(
        move_format!("sin-{}", 10),
        x.clone()
            .step_by(3)
            .map(|x| [x, x.sin() - 10.])
            .buffer_iter(),
    );

    s.line_fill(
        move_format!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]).buffer_iter(),
    );

    s.render_io(std::io::stdout())?;

    Ok(())
}


```

## Output

<img src="./assets/simple.svg" alt="demo">

## CSS Usage Example

See the graphs in this report: [broccoli_book](https://tiby312.github.io/broccoli_report/)

## Why not scale the intervals to end nicely with the ends of the axis lines?

Doing this you would have to either have more dead space, or exclude
plots that the user would expect to get plotted. Neither of these sounded
better than the option of just having the intervals stop not necessarily
at the end of the axis lines.

