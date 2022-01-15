mod tick_finder;
mod unixtime;

use super::*;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;
use unixtime::UnixTime;

impl PlotNum for UnixTime {
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        assert!(range[0] <= range[1]);

        let mut t = tick_finder::BestTickFinder::new(range, ideal_num_steps);

        t.consider_yr(&[1, 2, 5, 100, 200, 500, 1000, 2000, 5000]);
        t.consider_mo(&[1, 2, 6, 12, 24, 48]);
        t.consider_dy(&[1, 2, 5, 7, 10, 30, 60, 100, 365]);
        t.consider_hr(&[1, 2, 5, 10]);
        t.consider_mi(&[1, 2, 5, 10]);
        t.consider_se(&[1, 2, 5, 10]);

        let ticks:Vec<_>=t.into_best().into_iter().map(|x|Tick{position:x,value:x}).collect();

        assert!(ticks.len()>=2);
        
        TickInfo{
            ticks,
            display_relative:None
        }
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        unimplemented!();
    }

    fn unit_range(offset: Option<Self>) -> [Self; 2] {
        if let Some(o) = offset {
            [o, UnixTime(o.0 + 1)]
        } else {
            [UnixTime(0), UnixTime(1)]
        }
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let [val1, val2] = val;
        let [val1, val2] = [val1.0, val2.0];
        assert!(val1 <= val2);
        let diff = (val2 - val1) as f64;
        let scale = max / diff;
        self.0 as f64 * scale
    }

    fn dash_size(
        ideal_dash_size: f64,
        tick_info: &TickInfo<Self>,
        range: [Self; 2],
        max: f64,
    ) -> Option<f64> {
        None
        //compute_dash_size(ideal_dash_size, tick_info, range, max)
    }
}
