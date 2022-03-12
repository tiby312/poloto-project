use poloto::prelude::*;
fn main() {
    

    let data = poloto::data()
        .scatter("", [[5,0],[4,1],[6,2],[10,3]])
        .xmarker(0)
        .build();


    let (xtick,xtick_fmt)=poloto::ticks_from_default(data.boundx());

    let (ytick,ytick_fmt)=poloto::bar::gen_bar(data.boundy(),&[
        "potato",
        "chicken",
        "pizza",
        "popo"
    ]);


    let mut pp = data.plot_with(
        xtick,
        ytick,
        poloto::plot_fmt(
            "Number of rides at theme park hourly",
            "Hour",
            "Number of rides",
            xtick_fmt,
            ytick_fmt,
        ),
    );

    print!("{}", poloto::disp(|w| pp.simple_theme(w)));
}
