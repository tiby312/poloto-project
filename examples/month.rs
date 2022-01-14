use std::convert::TryFrom;

// PIPE me to a file!
fn main() {
    use poloto::util::integer::i128::MonthIndex;

    let data = [
        ("Jan", 3144000),
        ("Feb", 3518000),
        ("Mar", 3835000),
        ("Apr", 4133000),
        ("May", 4413000),
        ("Jun", 4682000),
        ("Jul", 5045000),
        ("Aug", 5321200),
        ("Sep", 5541900),
        ("Oct", 5773600),
        ("Nov", 5989400),
        ("Dec", 6219700),
        ("Jan", 3518000),
        ("Feb", 3518000),
    ];

    let mut s = poloto::plot("Number of Foos in 2021", "Months of 2021", "Foos");

    //Map the strings to indexes
    s.histogram("", (0..).map(MonthIndex).zip(data.iter().map(|x| x.1)));

    s.ymarker(0);

    //Lookup the strings with the index
    s.xinterval_fmt(|fmt, val, _| write!(fmt, "{}", data[usize::try_from(val.0).unwrap()].0));

    println!("{}", poloto::disp(|a| poloto::simple_theme_dark(a, s)));
}
