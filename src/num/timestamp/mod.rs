//! Plot unix timestamps.
//!
//! Does not implement dashes/grid lines because due to leap days, the distance
//! between the dashes can't be constant.
//!
//!
//!
//!  
mod tick_finder;
mod unixtime;

use super::*;
use chrono::prelude::*;
use chrono::DateTime;
pub use unixtime::*;

#[derive(Copy, Clone, Debug)]
pub enum TimestampType {
    YR,
    MO,
    DY,
    HR,
    MI,
    SE,
}

pub fn write_fmt<T: fmt::Write>(
    mut formatter: T,
    val: UnixTime,
    step: TimestampType,
    fmt: FmtFull,
) -> fmt::Result {
    use TimestampType::*;

    let m = match val.month() {
        1 => "Jan",
        2 => "Feb",
        3 => "Mar",
        4 => "Apr",
        5 => "May",
        6 => "Jun",
        7 => "Jul",
        8 => "Aug",
        9 => "Sep",
        10 => "Oct",
        11 => "Nov",
        12 => "Dec",
        _ => unreachable!(),
    };

    match fmt {
        FmtFull::Full => {
            write!(formatter, "{}", val)
        }
        FmtFull::Tick => match step {
            YR => {
                write!(formatter, "{}", val.year())
            }
            MO => {
                write!(formatter, "{} {}", val.year(), m)
            }
            DY => {
                write!(formatter, "{} {}", m, val.day())
            }
            HR => {
                write!(formatter, "{}:{}", val.weekday(), val.hour())
            }
            MI => {
                write!(formatter, "{}:{}", val.hour(), val.minute())
            }
            SE => {
                write!(formatter, "{}:{}", val.minute(), val.second())
            }
        },
    }
}
#[derive(Default)]
pub struct DefaultUnixTimeContext;

impl PlotNumContext for DefaultUnixTimeContext {
    type Num = UnixTime;
    type UnitData = TimestampType;
    type TickIter = std::vec::IntoIter<Tick<UnixTime>>;

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [UnixTime; 2],
        _dash: DashInfo,
    ) -> TickInfo<UnixTime, TimestampType, Self::TickIter> {
        assert!(range[0] <= range[1]);

        let mut t = tick_finder::BestTickFinder::new(range, ideal_num_steps);

        let steps_yr = &[1, 2, 5, 100, 200, 500, 1000, 2000, 5000];
        let steps_mo = &[1, 2, 3, 6, 12];
        let steps_dy = &[1, 2, 5, 10];
        let steps_hr = &[1, 2, 6, 12];
        let steps_mi = &[1, 2, 10, 15];
        let steps_se = &[1, 2, 5, 10];

        t.consider_yr(steps_yr, steps_mo);
        t.consider_mo(steps_mo, steps_dy);
        t.consider_dy(steps_dy, steps_hr);
        t.consider_hr(steps_hr, steps_mi);
        t.consider_mi(steps_mi, steps_se);
        t.consider_se(steps_se, &[1, 2, 5, 10]);

        //TODO handle dashes???

        let ret = t.into_best().unwrap();

        let ticks: Vec<_> = ret
            .ticks
            .into_iter()
            .map(|x| Tick {
                position: x,
                value: x,
            })
            .collect();

        assert!(ticks.len() >= 2);

        let dash_size = None;

        /*
        let dash_size = {
            /*
            let dash_multiple = good_normalized_step;
            let max = dash.max;
            let ideal_dash_size = dash.ideal_dash_size;
            let one_step = step.scale(range, max);

            assert!(dash_multiple > 0);

            if dash_multiple == 1 || dash_multiple == 10 {
                let a = test_multiple(ideal_dash_size, one_step, 2, range, max).unwrap();
                let b = test_multiple(ideal_dash_size, one_step, 5, range, max).unwrap();
                if (a - ideal_dash_size).abs() < (b - ideal_dash_size).abs() {
                    Some(a)
                } else {
                    Some(b)
                }
            } else {
                Some(test_multiple(ideal_dash_size, one_step, dash_multiple, range, max).unwrap())
            }
            */
        };
        */

        TickInfo {
            unit_data: ret.unit_data,
            ticks: ticks.into_iter(),
            dash_size,
            display_relative: None, //Never want to do this for unix time.
        }
    }

    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: UnixTime,
        step: TimestampType,
        fmt: FmtFull,
    ) -> std::fmt::Result {
        self::write_fmt(formatter, val, step, fmt)
    }

    fn unit_range(&mut self, offset: Option<UnixTime>) -> [UnixTime; 2] {
        if let Some(o) = offset {
            [o, UnixTime(o.0 + 1)]
        } else {
            [UnixTime(0), UnixTime(1)]
        }
    }

    fn scale(&mut self, val: UnixTime, range: [UnixTime; 2], max: f64) -> f64 {
        let [val1, val2] = range;
        let [val1, val2] = [val1.0, val2.0];
        assert!(val1 <= val2);
        let diff = (val2 - val1) as f64;
        let scale = max / diff;
        val.0 as f64 * scale
    }
}

impl PlotNum for UnixTime {
    type DefaultContext = DefaultUnixTimeContext;
}
