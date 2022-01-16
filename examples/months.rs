use poloto::util::integer::UnixTime;
// PIPE me to a file!
fn main() {
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (UnixTime::from_ymd(2020, 8, 30), 8144000),
        (UnixTime::from_ymd(2020, 9, 30), 3144000),
        (UnixTime::from_ymd(2020, 10, 4), 3518000),
        (UnixTime::from_ymd(2020, 11, 1), 3835000),
        (UnixTime::from_ymd_hms(2020, 11, 1, 12, 59, 59), 2133000),
        (UnixTime::from_ymd(2021, 1, 2), 4133000),
        (UnixTime::from_ymd(2021, 2, 3), 4413000),
        (UnixTime::from_ymd(2021, 3, 4), 4682000),
    ];

    /*
    for a in data[0].0.months(1).take(10){
        println!("{}",a);
    } */

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");

    s.line("", data);

    s.ymarker(0);

    let mut st = String::new();
    use std::fmt::Write;
    write!(&mut st, "{}", poloto::disp(|a| poloto::simple_theme(a, s))).unwrap();
    println!("{}", st);
}
