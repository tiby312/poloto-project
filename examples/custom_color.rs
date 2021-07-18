fn main() -> std::fmt::Result {
    let data = [(1.0f32, 4.5), (2.0, 5.5), (3.0, 6.5)];

    let data_int = [[1usize, 4], [2, 5], [3, 6]];

    let mut plotter = poloto::plot_with_html("cows per year", "year", "cows", MY_STYLE);

    plotter.scatter("ints", &data_int);

    plotter.line("floats", &data);

    plotter.render_io(std::io::stdout())
}

const MY_STYLE: &str = "<style>\
.poloto { \
    stroke-linecap:round;\
    font-family: sans-serif;\
    stroke-width:2;\
    }\
    .scatter{stroke-width:33}\
    .poloto_text{fill: black;  }\
    .poloto_axis_lines{stroke: black;stoke-width:3;fill:none}\
    .poloto_background{fill: aliceblue; }\
    .poloto0stroke{stroke:  purple; }\
    .poloto1stroke{stroke:  green; }\
    .poloto2stroke{stroke:  purple; }\
    .poloto3stroke{stroke:  purple; }\
    .poloto4stroke{stroke:  purple; }\
    .poloto5stroke{stroke:  purple; }\
    .poloto6stroke{stroke:  purple; }\
    .poloto7stroke{stroke:  purple; }\
    .poloto0fill{fill:purple;}\
    .poloto1fill{fill:green;}\
    .poloto2fill{fill:purple;}\
    .poloto3fill{fill:purple;}\
    .poloto4fill{fill:purple;}\
    .poloto5fill{fill:purple;}\
    .poloto6fill{fill:purple;}\
    .poloto7fill{fill:purple;}\
</style>";
