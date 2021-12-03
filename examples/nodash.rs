use poloto::util::NoDash;

// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (NoDash(2010), NoDash(-3144000)),
        (NoDash(2011), NoDash(-3518000)),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");

    s.line("", &data);

    s.simple_theme_dark(poloto::upgrade_write(std::io::stdout()));
}
