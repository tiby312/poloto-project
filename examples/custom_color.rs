use poloto::*;
fn main() {
    let data = [(1.0f32, 4.5), (2.0, 5.5), (3.0, 6.5)];

    let data_int = [[1usize, 4], [2, 5], [3, 6]];

    let mut plotter = plot("cows per year", "year", "cows");

    plotter.scatter("ints", &data_int);

    plotter.line("floats", &data);

    let mut e = tagger::new(tagger::upgrade_write(std::io::stdout()));

    default_svg(&mut e, tagger::no_attr(), |d| {
        d.put_raw(poloto::minify(MY_STYLE));
        plotter.render(d.writer());
    });
}

const MY_STYLE: &str = r###"<style>
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
