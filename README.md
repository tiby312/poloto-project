
You can find poloto on [github](https://github.com/tiby312/poloto) and [crates.io](https://crates.io/crates/poloto).
Documentation at [docs.rs](https://docs.rs/poloto)

A simple 2D plotting library that outputs graphs to SVG that can be styled using CSS.

Poloto graphs can be stylized using css either directly in the SVG, or from inside of html with an embedded svg. The latter allows the user to dynamically match the svg to their website's theme. The user can take full advantage of CSS, adding highlight on hover, animation, shadows, strokes, etc. Check out the [github examples](https://github.com/tiby312/poloto/tree/master/examples) to see this. The latest graph outputs of the examples can be found in the [assets](https://github.com/tiby312/poloto/tree/master/assets) folder.

You can see it in action in this rust book [broccoli-book](https://tiby312.github.io/broccoli_report/)



## Gaussian Example

```rust
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    // See https://en.wikipedia.org/wiki/Gaussian_function
    let gaussian = |sigma: f64, mu: f64| {
        use std::f64::consts::TAU;
        let s = sigma.powi(2);
        let k = (sigma * TAU).sqrt().recip();
        move |x: f64| (-0.5 * (x - mu).powi(2) / s).exp() * k
    };

    let range = (0..200).map(|x| (x as f64 / 200.0) * 10.0 - 5.0);

    let g1 = gaussian(1.0, 0.0);
    let g2 = gaussian(0.5, 0.0);
    let g3 = gaussian(0.3, 0.0);

    let mut plotter = poloto::plot("gaussian", "x", "y", f64::ctx(), f64::ctx());

    plotter.line("σ = 1.0", range.clone().map(|x| [x, g1(x)]));
    plotter.line("σ = 0.5", range.clone().map(|x| [x, g2(x)]));
    plotter.line("σ = 0.3", range.clone().map(|x| [x, g3(x)]));

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}

```
## Output

<img src="./assets/gaussian.svg" alt="demo">


## Data Example

```rust
use poloto::num::timestamp::UnixTime;
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_year(2010), 3144000),
        (UnixTime::from_year(2011), 3518000),
        (UnixTime::from_year(2012), 3835000),
        (UnixTime::from_year(2013), 4133000),
        (UnixTime::from_year(2014), 4413000),
        (UnixTime::from_year(2015), 4682000),
        (UnixTime::from_year(2016), 5045000),
        (UnixTime::from_year(2017), 5321200),
        (UnixTime::from_year(2018), 5541900),
        (UnixTime::from_year(2019), 5773600),
        (UnixTime::from_year(2020), 5989400),
        (UnixTime::from_year(2021), 6219700),
        (UnixTime::from_year(2022), 0), //To complete our histogram, we manually specify when 2021 ends.
    ];

    let xname = poloto::fmt::name_ext(|w, ([min, max], xs), _| {
        let srt = UnixTime::dynamic_format(&min, xs);
        let end = UnixTime::dynamic_format(&max, xs);
        write!(w, "Entries from {} to {} in {}", srt, end, xs)
    });

    let mut plotter = poloto::plot(
        "title",
        xname,
        "yname",
        UnixTime::ctx().with_tick_fmt(|v, w, _b, _s| write!(w, "{}", v.format("%D"))),
        i128::ctx().with_no_dash().with_marker(0).with_no_dash(),
    );

    plotter.line("foo", &data);

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|w| plotter.render(w)),
        poloto::simple_theme::SVG_END
    )
}

```

## Output

<img src="./assets/years_fmt.svg" alt="demo">


## Collatz Example

```rust
use poloto::prelude::*;
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

    let mut plotter = poloto::plot("collatz", "x", "y", i128::ctx(), i128::ctx().with_marker(0));
    for i in 1000..1006 {
        plotter.line(poloto::formatm!("c({})", i), (0..).zip(collatz(i)));
    }

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::simple_theme::SVG_HEADER,
        poloto::simple_theme::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_line{stroke-dasharray:2;stroke-width:1;}",
        poloto::disp(|a| plotter.render(a)),
        poloto::simple_theme::SVG_END
    )
}

```
## Output


<img src="./assets/collatz.svg" alt="demo">


## Parametric Example

```rust
// PIPE me to a file!
use poloto::prelude::*;
fn main() {
    // https://mathworld.wolfram.com/HeartCurve.html
    let heart = |t: f64| {
        [
            16.0 * t.sin().powi(3),
            13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos(),
        ]
    };

    let range = (0..100).map(|x| x as f64 / 100.0).map(|x| x * 6.0 - 3.0);

    let mut plotter = poloto::plot(
        "Heart Graph",
        "x",
        "y",
        f64::ctx().with_marker(-20.0).with_marker(20.0),
        f64::ctx().with_marker(-20.0).with_marker(20.0),
    );

    plotter.line_fill_raw("heart", range.map(heart));
    plotter.preserve_aspect();

    println!("{}", poloto::disp(|a| plotter.simple_theme_dark(a)));
}

```

## Output

<img src="./assets/heart.svg" alt="demo">


## Trig Example 

```rust
use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let x = (0..500).map(|x| (x as f64 / 500.0) * 10.0);

    let mut plotter = poloto::plot(
        "Some Trigonometry Plots 🥳",
        formatm!("This is the {} label", 'x'),
        "This is the y label",
        f64::ctx(),
        f64::ctx(),
    );

    // Using poloto::Croppable, we can filter out plots and still have discontinuity.
    plotter.line(
        "tan(x)",
        x.clone()
            .map(|x| [x, x.tan()])
            .crop_above(10.0)
            .crop_below(-10.0)
            .crop_left(2.0),
    );

    plotter.line("sin(2x)", x.clone().map(|x| [x, (2.0 * x).sin()]));

    plotter.line(
        "2*cos(x)",
        x.clone().map(|x| [x, 2.0 * x.cos()]).crop_above(1.4),
    );

    println!("{}", poloto::disp(|a| plotter.simple_theme(a)));
}

```

## Output

<img src="./assets/trig.svg" alt="demo">



## CSS Usage Example

See the graphs in this report: [broccoli_book](https://tiby312.github.io/broccoli_report/)

## CSS classes

Below are the css classes that can be stylized. There are default styles settings
for these css classes in the static strings `STYLE_CONFIG_LIGHT_DEFAULT` and `STYLE_CONFIG_DARK_DEFAULT`.

These are the css classes added through `Plotter::render`

* `poloto_text` - all poloto text
* `poloto_axis_lines` - axis lines and ticks 
* `poloto_tick_labels` - x and y labels as well as `where` labels
* `poloto_labels` - title, x label, ylabel
* `poloto_title` - title
* `poloto_xname` - xlabel
* `poloto_yname` - ylabel
* `poloto_legend_text` - legend text
* `poloto_legend_icon` - legend icon
* `poloto_scatter` - scatter plots and legend icon
* `poloto_line` - line plots and legend icon
* `poloto_histo` - histogram and legend icon 
* `poloto_linefill` - line fill and legend icon
* `poloto_linefillraw` - line fill raw and legend icon

These are the css classes added through `poloto::SVG_HEADER` which is used by `simple_theme` and `simple_theme_dark`.

* `poloto` - default svg element

For plots:

* `poloto[n]fill` - If the n'th plot requires fill. (e.g. linefill or histogram)
* `poloto[n]stroke` - If the n'th plot requires stroke. (e.g. line or scatter)


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