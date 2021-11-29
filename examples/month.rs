

const MONTHS:&[&'static str]=
&[
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec",
];

#[derive(Copy,Clone,PartialOrd,PartialEq)]
struct MonthNum(i128);

impl std::fmt::Display for MonthNum{
    fn fmt(&self,w:&mut std::fmt::Formatter)->std::fmt::Result{
        write!(w,"{}",MONTHS[(self.0 as usize) % 12])
    }
}

impl poloto::util::PlotNumber for MonthNum{

    fn compute_ticks(ideal_num_steps:usize,range:[Self;2])->poloto::util::TickInfo<Self>{
        poloto::util::compute_ticks_i128(ideal_num_steps,[range[0].0,range[1].0]).map(|v|MonthNum(v))
    }


    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        _step: Option<Self>,
    ) -> std::fmt::Result{
        write!(formatter, "{}", self)
    }
    

    fn unit_range()->[Self;2]{
        [MonthNum(0),MonthNum(1)]
    }


    fn scale(&self,val:[Self;2],max:f64)->f64{
        let diff=(val[1].0-val[0].0) as f64;

        let scale=max/diff;

        (self).0 as f64*scale
    }
}


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
    s.histogram("", data.iter().enumerate().map(|c| (MonthNum(c.0 as i128), c.1 .1)));

    s.ymarker(0);
    //Lookup the strings with the index
    //s.xinterval_fmt(|fmt, val, _| write!(fmt, "{}", data[unsafe{val.to_int_unchecked::<usize>()}].0));

    s.simple_theme_dark(poloto::upgrade_write(std::io::stdout()));
}
