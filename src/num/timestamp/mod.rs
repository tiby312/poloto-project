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

impl std::fmt::Display for TimestampType {
    fn fmt(&self, a: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(a, "{:?}", self)
    }
}

impl PlotNum for UnixTime {
    type StepInfo = TimestampType;

    fn scale(&self, range: [UnixTime; 2], max: f64) -> f64 {
        let val = *self;
        let [val1, val2] = range;
        let [val1, val2] = [val1.0, val2.0];
        assert!(val1 <= val2);
        let diff = (val2 - val1) as f64;
        let scale = max / diff;
        val.0 as f64 * scale
    }

    fn tick_fmt(
        &mut self,
        writer: &mut dyn fmt::Write,
        _bound:[Self;2],
        info: &mut TimestampType,
    ) -> std::fmt::Result {
        write!(writer,"{}",self.dynamic_format(info))
    }

    fn compute_ticks(
        ideal_num_steps: u32,
        range: [UnixTime; 2],
        _info: DashInfo,
    ) -> TickInfo<UnixTime> {
        assert!(range[0] <= range[1]);

        let mut t = tick_finder::BestTickFinder::new(range, ideal_num_steps);

        let steps_yr = &[1, 2, 5, 100, 200, 500, 1000, 2000, 5000];
        let steps_mo = &[1, 2, 3, 6];
        let steps_dy = &[1, 2, 4, 5, 7];
        let steps_hr = &[1, 2, 4, 6];
        let steps_mi = &[1, 2, 10, 15, 30];
        let steps_se = &[1, 2, 5, 10, 15, 30];

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
            ticks,
            dash_size,
            display_relative: None, //Never want to do this for unix time.
        }
    }

    fn unit_range(offset: Option<UnixTime>) -> [UnixTime; 2] {
        if let Some(o) = offset {
            [o, UnixTime(o.0 + 1)]
        } else {
            [UnixTime(0), UnixTime(1)]
        }
    }
}
