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

///
/// Returns a 3 letter string for a month. input must be in the range `[1,12]` or it will panic.
///
pub fn month_str(month: u32) -> &'static str {
    match month {
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
    }
}

pub struct UnixTimeTickFmt<T: TimeZone> {
    step: StepUnit,
    timezone: T,
}
impl<T: TimeZone> UnixTimeTickFmt<T> {
    pub fn step(&self) -> StepUnit {
        self.step
    }
}
impl<T: TimeZone> TickFormat for UnixTimeTickFmt<T> {
    type Num = UnixTime;

    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        write!(writer, "{}", val.dynamic_format(&self.timezone, &self.step))
    }
}

///
/// Conveys what unit is being used for step sizes.
///
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StepUnit {
    YR,
    MO,
    DY,
    HR,
    MI,
    SE,
}

impl StepUnit {
    pub fn is_years(&self) -> bool {
        *self == StepUnit::YR
    }
    pub fn is_months(&self) -> bool {
        *self == StepUnit::MO
    }
    pub fn is_days(&self) -> bool {
        *self == StepUnit::DY
    }
    pub fn is_hours(&self) -> bool {
        *self == StepUnit::HR
    }

    pub fn is_minutes(&self) -> bool {
        *self == StepUnit::MI
    }

    pub fn is_seconds(&self) -> bool {
        *self == StepUnit::SE
    }
}
impl std::fmt::Display for StepUnit {
    fn fmt(&self, a: &mut std::fmt::Formatter) -> std::fmt::Result {
        use StepUnit::*;
        let val = match &self {
            YR => "Years",
            MO => "Months",
            DY => "Days",
            HR => "Hours",
            MI => "Minutes",
            SE => "Seconds",
        };
        write!(a, "{}", val)
    }
}

impl Default for UnixTimeTickGen<Utc> {
    fn default() -> Self {
        UnixTimeTickGen { timezone: Utc }
    }
}

pub fn unixtime_ticks<T: TimeZone>(
    bound: crate::Bound<UnixTime>,
    timezone: &T,
) -> TickDist<UnixTimeTickFmt<T>> {
    UnixTimeTickGen::new(timezone).generate(bound)
}

pub struct UnixTimeTickGen<T: TimeZone> {
    timezone: T,
}

impl<T: TimeZone> UnixTimeTickGen<T> {
    pub fn new(timezone: &T) -> Self {
        UnixTimeTickGen {
            timezone: timezone.clone(),
        }
    }
}

impl<T: TimeZone> TickGenerator for UnixTimeTickGen<T> {
    type Num = UnixTime;
    type Fmt = UnixTimeTickFmt<T>;
    fn generate(self, bound: crate::Bound<Self::Num>) -> TickDist<Self::Fmt> {
        let range = [bound.min, bound.max];

        assert!(range[0] <= range[1]);
        let ideal_num_steps = bound.ideal_num_steps;

        let ideal_num_steps = ideal_num_steps.max(2);

        let [start, end] = range;
        let mut t = tick_finder::BestTickFinder::new(end, ideal_num_steps);

        let steps_yr = &[1, 2, 5, 100, 200, 500, 1000, 2000, 5000];
        let steps_mo = &[1, 2, 3, 6];
        let steps_dy = &[1, 2, 4, 5, 7];
        let steps_hr = &[1, 2, 4, 6];
        let steps_mi = &[1, 2, 10, 15, 30];
        let steps_se = &[1, 2, 5, 10, 15, 30];

        let d = start.datetime(&self.timezone);
        use StepUnit::*;
        t.consider_meta(YR, UnixYearGenerator { date: d.clone() }, steps_yr);
        t.consider_meta(MO, UnixMonthGenerator { date: d.clone() }, steps_mo);
        t.consider_meta(DY, UnixDayGenerator { date: d.clone() }, steps_dy);
        t.consider_meta(HR, UnixHourGenerator { date: d.clone() }, steps_hr);
        t.consider_meta(MI, UnixMinuteGenerator { date: d.clone() }, steps_mi);
        t.consider_meta(SE, UnixSecondGenerator { date: d }, steps_se);

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

        TickDist {
            ticks: TickInfo {
                bound,
                ticks,
                dash_size: None,
                display_relative: None, //Never want to do this for unix time.
            },
            fmt: UnixTimeTickFmt {
                timezone: self.timezone,
                step: ret.unit_data,
            },
        }
    }
}

impl PlotNum for UnixTime {
    type DefaultTickGenerator = UnixTimeTickGen<Utc>;
    fn scale(&self, range: [UnixTime; 2], max: f64) -> f64 {
        let val = *self;
        let [val1, val2] = range;
        let [val1, val2] = [val1.0, val2.0];
        assert!(val1 <= val2);
        let diff = (val2 - val1) as f64;
        let scale = max / diff;
        val.0 as f64 * scale
    }
    fn unit_range(offset: Option<UnixTime>) -> [UnixTime; 2] {
        if let Some(o) = offset {
            [o, UnixTime(o.0 + 1)]
        } else {
            [UnixTime(0), UnixTime(1)]
        }
    }
}
