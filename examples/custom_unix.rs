
use poloto::num::timestamp::UnixTime;
use poloto::num::timestamp::TimestampType;
#[repr(transparent)]
#[derive(Copy,Clone,Eq,PartialEq,PartialOrd)]
pub struct MyTime(UnixTime);

impl poloto::plotnum::PlotNum for MyTime {
    type StepInfo = TimestampType;

    fn scale(&self, range: [MyTime; 2], max: f64) -> f64 {
        let range=[range[0].0,range[1].0];
        self.0.scale(range,max)
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        range:[Self;2],
        info: &mut TimestampType,
    ) -> std::fmt::Result {
        //let range=[range[0].0,range[1].0];
        //self.0.tick_fmt(writer,range,info)
        write!(writer,"{}",self.0)
    }

    fn where_fmt(&mut self, writer:&mut dyn std::fmt::Write, range:[Self;2]) ->std::fmt::Result {
        let range=[range[0].0,range[1].0];
        self.0.where_fmt(writer,range)
    }

    fn compute_ticks(
        ideal_num_steps: u32,
        range: [MyTime; 2],
        info: poloto::plotnum::DashInfo,
    ) -> poloto::plotnum::TickInfo<MyTime> {
        let range=[range[0].0,range[1].0];
        UnixTime::compute_ticks(ideal_num_steps,range,info).map(|x|MyTime(x))
    }

    fn unit_range(offset: Option<MyTime>) -> [MyTime; 2] {
        if let Some(o) = offset {
            [o, MyTime(UnixTime(o.0.0 + 1))]
        } else {
            [MyTime(UnixTime(0)), MyTime(UnixTime(1))]
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
        (UnixTime::from_ymd_hms(day1, 23, 30, 59), 3144000),
        (UnixTime::from_ymd_hms(day2, 1, 2, 0), 3518000),
        (UnixTime::from_ymd_hms(day2, 1, 5, 1), 3835000),
        (UnixTime::from_ymd_hms(day2, 1, 30, 59), 2133000),
        (UnixTime::from_ymd_hms(day2, 1, 50, 1), 4133000),
    ];

    let mut s = poloto::plot("Number of Wikipedia Articles", "Year", "Number of Articles");
    s.line("", data.into_iter().map(|(a,b)|(MyTime(a),b)));
    s.yaxis().marker(0);

    println!("{}", poloto::disp(|a| s.simple_theme(a)));
}
