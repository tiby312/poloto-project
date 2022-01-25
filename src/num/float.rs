//!
//! Plot floats
//!
use super::*;

impl DiscNum for f64 {
    fn hole() -> Self {
        f64::NAN
    }
}

///
/// Default float context. It will attempt to find reasonable step sizes, and format them as regular floats.
///
#[derive(Default)]
pub struct DefaultFloatContext;
impl PlotNumContext for DefaultFloatContext {
    type Num = f64;
    type StepInfo = f64;

    fn tick_fmt(
        &mut self,
        val: Self::Num,
        writer: &mut dyn fmt::Write,
        _bound: [Self::Num; 2],
        info: &mut Self::StepInfo,
    ) -> std::fmt::Result {
        util::write_interval_float(writer, val, Some(*info))
    }

    fn where_fmt(
        &mut self,
        val: f64,
        writer: &mut dyn std::fmt::Write,
        _bound: [f64; 2],
    ) -> std::fmt::Result {
        util::write_interval_float(writer, val, None)
    }

    fn scale(&mut self, val: f64, range: [f64; 2], max: f64) -> f64 {
        let diff = range[1] - range[0];
        let scale = max / diff;
        val * scale
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [f64; 2],
        dash: DashInfo,
    ) -> TickInfo<f64, f64> {
        let good_ticks = &[1, 2, 5, 10];

        let (step, good_normalized_step) = find_good_step(good_ticks, ideal_num_steps, range);
        let (start_step, step_num) = get_range_info(step, range);

        let display_relative = util::should_fmt_offset(
            start_step,
            start_step + ((step_num - 1) as f64) * step,
            step,
        );

        let first_tick = if display_relative { 0.0 } else { start_step };

        let mut ticks = Vec::with_capacity(usize::try_from(step_num).unwrap());
        for a in 0..step_num {
            let position = start_step + step * (a as f64);
            let value = first_tick + step * (a as f64);

            ticks.push(Tick { position, value });
        }

        assert!(ticks.len() >= 2);

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
            display_relative: display_relative.then(|| start_step),
            dash_size,
        }
        //compute_ticks(ideal_num_steps, &[1, 2, 5, 10], range)
    }
    fn unit_range(&mut self, offset: Option<f64>) -> [f64; 2] {
        if let Some(o) = offset {
            [o - 1.0, o + 1.0]
        } else {
            [-1.0, 1.0]
        }
    }
}

impl PlotNum for f64 {
    fn is_hole(&self) -> bool {
        self.is_nan()
    }
}

impl HasDefaultContext for f64 {
    type DefaultContext = DefaultFloatContext;
}

fn round_up_to_nearest_multiple(val: f64, multiple: f64) -> f64 {
    ((val) / multiple).ceil() * multiple
}

//TODO handle case zero steps are found
fn get_range_info(step: f64, range_all: [f64; 2]) -> (f64, u32) {
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
fn find_good_step(good_steps: &[u32], ideal_num_steps: u32, range_all: [f64; 2]) -> (f64, u32) {
    let range = range_all[1] - range_all[0];

    let rough_step = range / (ideal_num_steps - 1) as f64;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor());

    let cc = good_steps.iter().map(|&x| {
        let num_steps = get_range_info(x as f64 * step_power, range_all).1;
        (x, (num_steps as i32 - ideal_num_steps as i32).abs())
    });

    let best = cc.min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    (best.0 as f64 * step_power, best.0)
}
