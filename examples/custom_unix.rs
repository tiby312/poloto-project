
use poloto::num::timestamp::UnixTime;
use poloto::num::timestamp::TimestampType;
#[repr(transparent)]
#[derive(Copy,Clone,PartialEq,PartialOrd)]
pub struct DayTimestamp(UnixTime);

impl poloto::plotnum::PlotNum for DayTimestamp {
    type StepInfo = TimestampType;

    fn scale(&self, [a,b]: [DayTimestamp; 2], max: f64) -> f64 {
        self.0.scale([a.0,b.0],max)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        _range:[Self;2],
        _info: &mut TimestampType,
    ) -> std::fmt::Result {
        write!(writer,"{}",self.0.format("%D"))
    }

    fn where_fmt(&mut self, writer:&mut dyn std::fmt::Write, [min,max]:[Self;2]) ->std::fmt::Result {
        self.0.where_fmt(writer,[min.0,max.0])
    }

    fn compute_ticks(
        ideal_num_steps: u32,
        [min,max]: [DayTimestamp; 2],
        info: poloto::plotnum::DashInfo,
    ) -> poloto::plotnum::TickInfo<DayTimestamp> {
        UnixTime::compute_ticks(ideal_num_steps,[min.0,max.0],info).map(|x|DayTimestamp(x))
    }

    fn unit_range(offset: Option<DayTimestamp>) -> [DayTimestamp; 2] {
        if let Some(o) = offset {
            [o, DayTimestamp(UnixTime(o.0.0 + 1))]
        } else {
            [DayTimestamp(UnixTime(0)), DayTimestamp(UnixTime(1))]
        }
    }
}



use poloto::prelude::*;
// PIPE me to a file!
fn main() {
    let day1 = (2020, 1, 30);
    let day2 = (2020, 1, 31);
    //Source https://en.wikipedia.org/wiki/Wikipedia:Size_of_Wikipedia
    let data = [
        (DayTimestamp(UnixTime::from_ymd_hms(day1, 23, 30, 59)), 3144000),
        (DayTimestamp(UnixTime::from_ymd_hms(day2, 1, 2, 0)), 3518000),
        (DayTimestamp(UnixTime::from_ymd_hms(day2, 1, 5, 1)), 3835000),
        (DayTimestamp(UnixTime::from_ymd_hms(day2, 1, 30, 59)), 2133000),
        (DayTimestamp(UnixTime::from_ymd_hms(day2, 1, 50, 1)), 4133000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");
    s.line("", &data);
    s.yaxis().marker(0);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
