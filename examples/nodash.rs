use poloto::util::no_dash_tuple;

// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (2010, -3144000),
        (2011, -3518000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");

    s.line("", data.map(no_dash_tuple));

    println!("{}", poloto::disp(|a| poloto::simple_theme_dark(a, s)));
}
