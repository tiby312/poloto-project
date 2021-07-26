
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
Documentation at [docs.rs](https://docs.rs/poloto)

A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this. The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

## Simple Example

```rust
// PIPE me to a file!
fn main() {
    let data = [
        [1850.0f64, 10.0],
        [1940.0, 12.0],
        [1945.0, 12.2],
        [1989.0, 16.0],
        [2001.0, 20.0],
    ];

    let mut s = poloto::plot("simple", "x", "y");

    s.line_fill("", &data);

    println!("{}", s.render());
}

```

## Output

<img src="./assets/simple.svg" alt="demo">


## Another Example 

```rust
use tagger::prelude::*;

// PIPE me to a file!
fn main() {
    let x = (0..50).map(|x| (x as f32 / 50.0) * 10.0);

    // Collect the iterator before passing it to a plot function
    // if you are using an expensive iterator.
    // The buffer has to live longer than the plotter, so we collect it here.
    let buffer = x.clone().map(|x| [x, x.sin()]).collect::<Vec<_>>();

    let mut plotter = poloto::Plotter::new(
        poloto::default_svg().appendm(single!(poloto::HTML_CONFIG_DARK_DEFAULT)),
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    // The iterator will be cloned and ran twice.
    plotter.line("cos", x.clone().map(|x| [x, x.cos()]));

    // When passing the buffer, make sure you pass it as a reference.
    // If you don't do this, then the buffer will be duplicated in memory as
    // the plotter will call `.clone()` on the iterator.
    plotter.scatter("sin", &buffer);

    plotter.histogram(
        formatm!("sin-{}", 10),
        x.clone().step_by(3).map(|x| [x, (x.sin() - 10.).round()]),
    );

    plotter.line_fill(
        formatm!("sin-{}", 20),
        x.clone().map(|x| [x, x.sin() - 20.]),
    );

    println!("{}", plotter.render());
}

```

## Output

<img src="./assets/trig.svg" alt="demo">

## Another example

```
fn gaussian(a:f64,b:f64,c:f64)->impl Fn(f64)->f64{
    move |x|{
        a*( -((x-b)*(x-b)) /(2.0*c*c) ).exp()
    }
}

// PIPE me to a file!
fn main() {
    
    let a=gaussian(10.0,0.0,0.3);
    let b=gaussian(05.0,0.0,0.2);
    let c=gaussian(15.0,0.0,0.1);

    let range=(0..10000).map(|x|x as f64/10000.0).map(|x|x*2.0-1.0);

    let mut s = poloto::plot("gaussian", "x", "y");
    
    s.line("",range.clone().map(|x|[x,a(x)]));
    s.line("",range.clone().map(|x|[x,c(x)]));
    s.line("",range.clone().map(|x|[x,b(x)]));
    
    println!("{}", s.render());
}
```
## Output

<img src="./assets/gaussian.svg" alt="demo">

## CSS Usage Example

See the graphs in this report: [broccoli_book](https://tiby312.github.io/broccoli_report/)


## Iterating plots twice

In order to calculate the right size view to scale all the plots, poloto has to iterate over all the plot
points twice. Once to find the min and max bounds, and once to scale all the points by the scale determined
by the first iteration. 

If you are using an iterator where each iteration is expensive, consider running the iterator just once,
collecting the results in a Vec. Then pass that Vec to the plotting functions. 
Beware of passing the buffer directly to the plotter! If you do this, you'll use a lot of memory since 
the plotter will clone the whole buffer. Instead pass a reference to the buffer. See the second example below.


## Can I change the styling of the plots?

Yes! You can harness the power of CSS both in the svg, or outside
in html with an embeded svg. Some things you can do:

 * Change the color scheme to fit your html theme.
 * Highlight one plot, make it dashed, or add hover effect
 * Animate things using @keyframes

The `Plotter` struct documents which css classes you can modify for the graph as a whole.
Each plot function documents which css classes you can modify to change that specific plot.

Scatter plots are done using SVG paths made up of lines of zero length. This allows you to change
the radius of the scatter dots by changing the stroke width.


## Formatting Tick Intervals

Poloto will first print intervals in normal decimal at the precision required to capture the differences
in the step size between the intervals. If the magnitude of a number is detected to be too big or small, it
may switch to scientific notation, still at the required precision. It will only switch if the scientific
notation version is actually less characters than the normal decimal format which is not always the case
when you consider the precision that might be required to capture the step size.

Even with the above system, there are cases where the numbers all have a really big magnitude, but
are all really close together (small step size). In this case, there isn't really a good way to format it.
In these cases, poloto will fall back to making the number relative to the first number.