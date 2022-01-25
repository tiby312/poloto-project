//!
//! Plot integers
//!
use super::*;

fn round_up_to_nearest_multiple(val: i128, multiple: i128) -> i128 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

///
/// Returns where the first tick should be, as well as how many ticks there are
///
fn get_range_info(step: i128, range_all: [i128; 2]) -> (i128, u32) {
    let start_step = round_up_to_nearest_multiple(range_all[0], step);

    let step_num = {
        let mut counter = start_step;
        let mut res = 0;
        for a in 0.. {
            if counter > range_all[1] {
                res = a;
                break;
            }

            assert!(step + counter > counter, "{:?}", (step, range_all));
            counter += step;
        }
        res
    };

    (start_step, step_num)
}

///
/// INPUT:
/// Ideal number of ticks
/// Allowed step ammounts
/// The range
///
/// OUTPUT:
/// The best step size
/// The step power (e.g. 1,2,5, or 10)
fn find_good_step(good_steps: &[u32], ideal_num_steps: u32, range_all: [i128; 2]) -> (i128, u32) {
    let range = range_all[1] - range_all[0];

    let rough_step = (range / (ideal_num_steps - 1) as i128).max(1);

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor()) as i128;

    let cc = good_steps.iter().map(|&x| {
        let num_steps = get_range_info(x as i128 * step_power, range_all).1;
        (x, (num_steps as i32 - ideal_num_steps as i32).abs())
    });

    let best = cc.min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    (best.0 as i128 * step_power, best.0)
}

///
/// Default integer context. It will attempt to find reasonable step sizes, and format them as regular integers.
///
#[derive(Default)]
pub struct DefaultIntegerContext;
impl PlotNumContext for DefaultIntegerContext {
    type StepInfo = i128;
    type Num = i128;

    fn tick_fmt(
        &mut self,
        writer: &mut dyn fmt::Write,
        val: i128,
        _bound: [i128; 2],
        info: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        util::write_interval_i128(writer, val, Some(*info))
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: i128,
        _bound: [i128; 2],
    ) -> std::fmt::Result {
        util::write_interval_i128(writer, val, None)
    }

    fn scale(&mut self, val: i128, range: [i128; 2], max: f64) -> f64 {
        let diff = (range[1] - range[0]) as f64;

        let scale = max / diff;

        val as f64 * scale
    }
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [i128; 2],
        dash: DashInfo,
    ) -> TickInfo<i128, i128> {
        let good_ticks = &[1, 2, 5, 10];

        let (step, good_normalized_step) = find_good_step(good_ticks, ideal_num_steps, range);
        let (start_step, step_num) = get_range_info(step, range);

        let display_relative = util::should_fmt_offset(
            start_step as f64,
            (start_step + ((step_num - 1) as i128) * step) as f64,
            step as f64,
        );

        let first_tick = if display_relative { 0 } else { start_step };

        let mut ticks = Vec::with_capacity(usize::try_from(step_num).unwrap());
        for a in 0..step_num {
            let position = start_step + step * (a as i128);
            let value = first_tick + step * (a as i128);

            ticks.push(Tick { position, value });
        }

        let dash_size = {
            let dash_multiple = good_normalized_step;
            let max = dash.max;
            let ideal_dash_size = dash.ideal_dash_size;
            let one_step = self.scale(step, range, max);

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
        };

        //let dash_size = None;

        TickInfo {
            unit_data: step,
            ticks,
            dash_size,
            //dash_size,
            display_relative: display_relative.then(|| start_step),
        }
    }

    fn unit_range(&mut self, offset: Option<i128>) -> [i128; 2] {
        if let Some(o) = offset {
            [o - 1, o + 1]
        } else {
            [-1, 1]
        }
    }
}

impl HasDefaultContext for i128 {
    type DefaultContext = DefaultIntegerContext;
}

impl PlotNum for i128 {}

/*
pub use month::MonthIndex;
mod month {
    use super::*;

    ///
    /// A wrapper type that displays ticks at intervals that make sense for indexing to months.
    /// Ticks will appear at 1,2,6,12 instead of 1,2,5,10.
    /// See the month example.
    ///
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    pub struct MonthIndex(pub i128);

    impl fmt::Display for MonthIndex {
        fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
            write!(a, "{}", self.0)
        }
    }
    impl PlotNum for MonthIndex {
        type UnitData = <i128 as PlotNum>::UnitData;
        fn compute_ticks(ideal_num_steps: u32, range: [Self; 2],dash:DashInfo) -> TickInfo<Self, Self::UnitData> {
            let cc = [1, 2, 6, 12].iter().map(|&step| {
                let (a, num_steps) = get_range_info(step, [range[0].0, range[1].0]);
                (
                    step,
                    a,
                    num_steps,
                    (ideal_num_steps as i32 - num_steps as i32).abs(),
                )
            });

            let (step, start_step, num_steps, _) = cc.min_by(|a, b| a.3.cmp(&b.3)).unwrap();

            let mut ticks = Vec::with_capacity(usize::try_from(num_steps).unwrap());
            for a in 0..num_steps {
                let position = MonthIndex(start_step + step * (a as i128));
                ticks.push(Tick {
                    position,
                    value: position,
                });
            }

            let dash_multiple = step as u32;

            let step = MonthIndex(step);
            let start_step = MonthIndex(start_step);

            assert!(ticks.len() >= 2);

            TickInfo {
                unit_data: StepAmount(step.0),
                ticks,
                //dash_multiple,
                display_relative: None,
            }
        }

        fn unit_range(offset: Option<Self>) -> [Self; 2] {
            if let Some(o) = offset {
                [MonthIndex(o.0 - 1), MonthIndex(o.0 + 1)]
            } else {
                [MonthIndex(-1), MonthIndex(1)]
            }
        }

        fn fmt_tick(
            &self,
            formatter: &mut std::fmt::Formatter,
            step: Self::UnitData,
            fmt: FmtFull,
        ) -> std::fmt::Result {
            tick_fmt::write_interval_i128(formatter, self.0, None)
        }

        fn scale(&self, val: [Self; 2], max: f64) -> f64 {
            let diff = (val[1].0 - val[0].0) as f64;

            let scale = max / diff;

            (self.0) as f64 * scale
        }

        fn dash_size(
            ideal_dash_size: f64,
            tick_info: &TickInfo<Self, Self::UnitData>,
            range: [Self; 2],
            max: f64,
        ) -> Option<f64> {
            None
            /*
            let one_step = tick_info.step.scale(range, max);
            let mut dash_multiple = tick_info.dash_multiple;

            assert!(dash_multiple > 0);

            if dash_multiple == 1 || dash_multiple == 12 {
                dash_multiple = 6;
            }

            for x in 1..50 {
                let dash_size = one_step / ((dash_multiple.pow(x)) as f64);
                if dash_size < ideal_dash_size {
                    return Some(dash_size);
                }
            }
            unreachable!(
                "Could not find a good dash step size! {:?}",
                (one_step, dash_multiple, ideal_dash_size)
            );
            */
        }
    }
}
*/
