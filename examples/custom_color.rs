use poloto::*;
fn main() {
    let data = [(1.0, 4.5), (2.0, 5.5), (3.0, 6.5)];

    let data_int = [[1.0, 4.0], [2.0, 5.0], [3.0, 6.0]];

    let mut plotter = plot("cows per year", "year", "cows");

    plotter.scatter("ints", &data_int);

    plotter.line("floats", &data);

    println!("{}", CUSTOM_SVG);
    plotter.render(poloto::upgrade_write(std::io::stdout()));
    println!("{}", poloto::SVG_END);
}

const CUSTOM_SVG: &str = r###"
<svg class="poloto_background poloto" width="800" height="500" viewBox="0 0 800 500" xmlns="http://www.w3.org/2000/svg">
<style>
    .poloto { 
    stroke-linecap:round;
    stroke-linejoin:round;
    font-family: 'Tahoma', sans-serif;
    stroke-width:2;
    }
    .scatter{stroke-width:33}
    .poloto_text{fill: black;  }
    .poloto_axis_lines{stroke: black;stroke-width:3;fill:none}
    .poloto_background{background-color: rgba(100,0,0,0.5) }
    .poloto0stroke{stroke:  purple; }
    .poloto1stroke{stroke:  green; }
    .poloto2stroke{stroke:  purple; }
    .poloto3stroke{stroke:  purple; }
    .poloto4stroke{stroke:  purple; }
    .poloto5stroke{stroke:  purple; }
    .poloto6stroke{stroke:  purple; }
    .poloto7stroke{stroke:  purple; }
    .poloto0fill{fill:purple;}
    .poloto1fill{fill:green;}
    .poloto2fill{fill:purple;}
    .poloto3fill{fill:purple;}
    .poloto4fill{fill:purple;}
    .poloto5fill{fill:purple;}
    .poloto6fill{fill:purple;}
    .poloto7fill{fill:purple;}
</style>"###;
