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
pub struct IntegerContext;
impl PlotNumContext for IntegerContext {
    type StepInfo = i128;
    type Num = i128;

    fn tick_fmt(
        &mut self,
        writer: &mut dyn fmt::Write,
        val: i128,
        _bound: [i128; 2],
        info: &Self::StepInfo,
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
        let good_ticks = &[1, 2, 5];

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


        TickInfo {
            unit_data: step,
            ticks,
            dash_size,
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
    type DefaultContext = IntegerContext;
}

impl PlotNum for i128 {}
