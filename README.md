
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
Documentation at [docs.rs](https://docs.rs/poloto)

A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embeded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this. The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)

## Gaussian Example

```rust
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gaussian = |sigma: f64, mu: f64| {
        move |x: f64| {
            (-0.5 * (x - mu).powi(2) / sigma.powi(2)).exp()
                * (sigma * std::f64::consts::TAU).sqrt().recip()
        }
    };

    let range: Vec<_> = (0..200)
        .map(|x| x as f64 / 200.0)
        .map(|x| x * 10.0 - 5.0)
        .collect();

    poloto::plot("gaussian", "x", "y")
        .line("σ = 1.0", range.iter().map(|&x| [x, gaussian(1.0, 0.0)(x)]))
        .line("σ = 0.5", range.iter().map(|&x| [x, gaussian(0.5, 0.0)(x)]))
        .line("σ = 0.3", range.iter().map(|&x| [x, gaussian(0.3, 0.0)(x)]))
        .ymarker(0.0)
        .simple_theme(poloto::upgrade_write(std::io::stdout()));
}

```
## Output

<img src="./assets/gaussian.svg" alt="demo">


## Data Example

```rust
// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        [2010, 3144000],
        [2011, 3518000],
        [2012, 3835000],
        [2013, 4133000],
        [2014, 4413000],
        [2015, 4682000],
        [2016, 5045000],
        [2017, 5321200],
        [2018, 5541900],
        [2019, 5773600],
        [2020, 5989400],
        [2021, 6219700],
        [2022, 0], //To complete our histogram, we manually specify when 2021 ends.
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");

    s.histogram("", &data);

    //Scale grpah to include up to the year 2025.
    //Also scale to include a value of 0 articles.
    s.xmarker(2025).ymarker(0);

    s.simple_theme_dark(poloto::upgrade_write(std::io::stdout()));
}

```

## Output

<img src="./assets/simple.svg" alt="demo">


## Collatz Example

```rust
use std::convert::TryFrom;

// PIPE me to a file!
fn main() {
    let collatz = |mut a: i128| {
        std::iter::from_fn(move || {
            //Base case
            if a == 1 {
                None
            } else {
                a = if a % 2 == 0 { a / 2 } else { 3 * a + 1 };
                Some(a)
            }
        })
        .fuse()
    };

    let mut p = poloto::plot("collatz", "x", "y");

    for i in 1000..1006 {
        p.line(
            poloto::formatm!("c({})", i),
            collatz(i)
                .enumerate()
                .map(|(x, y)| (i128::try_from(x).unwrap(), y)),
        );
    }

    p.ymarker(0).simple_with_element(
        poloto::upgrade_write(std::io::stdout()),
        format_args!(
            "<style>{}{}</style>",
            poloto::STYLE_CONFIG_DARK_DEFAULT,
            ".poloto{stroke-dasharray:2;stroke-width:1;}"
        ),
    );
}

```
## Output


<img src="./assets/collatz.svg" alt="demo">


## Parametric Example

```rust
// PIPE me to a file!
fn main() {
    // https://mathworld.wolfram.com/HeartCurve.html
    let heart = |t: f64| {
        [
            16.0 * t.sin().powi(3),
            13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
        ]
    };

    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    poloto::plot("Heart Graph", "x", "y")
        .line_fill("heart", range.map(|x| heart(x)))
        .preserve_aspect()
        .simple_theme(poloto::upgrade_write(std::io::stdout()));
}

```

## Output

<img src="./assets/heart.svg" alt="demo">


## Trig Example 

```rust
use poloto::formatm;

// PIPE me to a file!
fn main() {
    let x: Vec<_> = (0..500).map(|x| (x as f64 / 500.0) * 10.0).collect();

    let mut plotter = poloto::plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
    );

    use poloto::Croppable;
    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    plotter.line(
        "tan(x)",
        x.iter()
            .map(|&x| [x, x.tan()])
            .crop_above(10.0)
            .crop_below(-10.0)
            .crop_left(2.0),
    );

    plotter.line("sin(2x)", x.iter().map(|&x| [x, (2.0 * x).sin()]));

    plotter.line(
        "2*cos(x)",
        x.iter().map(|&x| [x, 2.0 * x.cos()]).crop_above(1.4),
    );

    plotter.simple_theme(poloto::upgrade_write(std::io::stdout()));
}

```

## Output

<img src="./assets/trig.svg" alt="demo">

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