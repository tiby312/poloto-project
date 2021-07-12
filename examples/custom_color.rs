use poloto::prelude::*;

fn main() -> std::fmt::Result {
    let data = [[1.0f64, 4.0], [2.0, 5.0], [3.0, 6.0]];

    let mut plotter = poloto::plot_with_html("cows per year", "year", "cows", MY_STYLE);

    plotter.line("cow", data.iter().map(|&x| x).twice_iter());

    plotter.render_io(std::io::stdout())?;

    Ok(())
}

const MY_STYLE: &str = r###"<style>
.poloto {
    font-family: "Arial";
    stroke-width:2;
    }
    .poloto_text{fill: black;  }
    .poloto_axis_lines{stroke: black;stoke-width:3;fill:none}
    .poloto_background{fill: aliceblue; }
    .poloto0stroke{stroke:  purple; }
    .poloto1stroke{stroke:  purple; }
    .poloto2stroke{stroke:  purple; }
    .poloto3stroke{stroke:  purple; }
    .poloto4stroke{stroke:  purple; }
    .poloto5stroke{stroke:  purple; }
    .poloto6stroke{stroke:  purple; }
    .poloto7stroke{stroke:  purple; }
    .poloto0fill{fill:purple;}
    .poloto1fill{fill:purple;}
    .poloto2fill{fill:purple;}
    .poloto3fill{fill:purple;}
    .poloto4fill{fill:purple;}
    .poloto5fill{fill:purple;}
    .poloto6fill{fill:purple;}
    .poloto7fill{fill:purple;}
</style>"###;
