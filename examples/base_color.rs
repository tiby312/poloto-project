fn main() {
    let data = [[0.000001, 0.000001], [0.000001000000001, 0.000001000000001]];
    
    let mut plotter = 
        poloto::plot("cows per year", "year", "cow")
            .scatter("", &data)
            .move_into();

    println!(
        "{}<style>{}{}</style>{}{}",
        poloto::SVG_HEADER,
        poloto::STYLE_CONFIG_DARK_DEFAULT,
        ".poloto_axis_lines{stroke:green;}.poloto_base.poloto_text{fill:red}",
        poloto::disp(|a| plotter.render(a)),
        poloto::SVG_END
    )


}
