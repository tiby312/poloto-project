use std::convert::TryInto;

// PIPE me to a file!
fn main() {
    let data = [
        ("Jan", 3144000i128),
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
        ("Mar", 3518000),
    ];

    let mut s = poloto::plot("Number of Foos in 2021", "Months of 2021", "Foos");

    //Map the strings to indexes
    s.histogram(
        "",
        data.iter().enumerate().map(|c| {
            let k: i128 = c.0.try_into().unwrap();
            (k, c.1 .1)
        }),
    );

    s.ymarker(0);

    //Lookup the strings with the index
    s.xinterval_fmt(|fmt, val, _| {
        let d: usize = val.try_into().unwrap();
        write!(fmt, "{}", data[d].0)
    });

    s.simple_theme_dark(poloto::upgrade_write(std::io::stdout()));
}
