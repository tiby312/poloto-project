fn main() {
    let mut s = poloto::plot(
        "Cows Per Year",
        "Year",
        "Cow",
    );
    let data=[
        [1979.0,10.0],
        [1989.0,12.0],
        [2001.0,13.0],
        [2010.0,4.0],
        [2014.0,3.0],
        [2020.0,6.0]
    ];
    
    s.line_fill("cows", data.iter().map(|x|*x));
    
    s.render_io(std::io::stdout()).unwrap();
}