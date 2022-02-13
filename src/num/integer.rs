//!
//! Plot integers
//!
use super::*;

fn round_up_to_nearest_multiple(val: i128, multiple: i128) -> i128 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}
/*
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
*/

/*
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
*/

pub struct IntegerTickFmt {
    step: i128,
}
impl IntegerTickFmt {
    pub fn step(&self)->i128{
        self.step
    }
}
impl TickFormat for IntegerTickFmt {
    type Num = i128;

    fn write_tick(&self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        util::write_interval_i128(writer, *val, Some(self.step))
    }
    fn write_where(&self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        util::write_interval_i128(writer, *val, None)
    }
}

pub struct IntegerTickGen;
//TODO use this thing!!!

impl TickGenerator for IntegerTickGen {
    type Num = i128;
    type Fmt = IntegerTickFmt;
    fn generate(bound: crate::Bound<Self::Num>) -> (TickInfo<Self::Num>, Self::Fmt) {
        let range = [bound.min, bound.max];
        let ideal_num_steps = bound.ideal_num_steps;
        let dash = bound.dash_info;

        let tick_layout = TickLayout::new(&[1, 2, 5], ideal_num_steps, range);

        let (display_relative, ticks) = tick_layout.generate();

        let dash_size = Some(compute_best_dash_1_2_5(
            tick_layout.step.default_scale(range, dash.max),
            dash.ideal_dash_size,
            tick_layout.normalized_step,
        ));

        (
            TickInfo {
                ticks,
                display_relative,
                dash_size,
            },
            IntegerTickFmt {
                step: tick_layout.step,
            },
        )
    }
}

impl PlotNum for i128 {
    type DefaultTickGenerator = IntegerTickGen;
    fn default_scale(&self, range: [i128; 2], max: f64) -> f64 {
        let val = *self;
        let diff = (range[1] - range[0]) as f64;

        let scale = max / diff;

        val as f64 * scale
    }

    fn default_unit_range(offset: Option<i128>) -> [i128; 2] {
        if let Some(o) = offset {
            [o - 1, o + 1]
        } else {
            [-1, 1]
        }
    }
}

struct TickLayout {
    step: i128,
    start_tick: i128,
    num_steps: u32,
    normalized_step: u32,
}
impl TickLayout {
    fn generate(&self) -> (Option<i128>, Vec<Tick<i128>>) {
        let (display_relative, first_tick) = {
            let tick_layout = self;
            let end =
                tick_layout.start_tick + ((tick_layout.num_steps - 1) as i128) * tick_layout.step;

            let mut start_s = String::new();
            let mut end_s = String::new();

            util::write_interval_float(
                &mut start_s,
                tick_layout.start_tick as f64,
                Some(tick_layout.step as f64),
            )
            .unwrap();
            util::write_interval_float(&mut end_s, end as f64, Some(tick_layout.step as f64))
                .unwrap();

            if start_s.len() > 7 || end_s.len() > 7 {
                (Some(tick_layout.start_tick), 0)
            } else {
                (None, tick_layout.start_tick)
            }
        };

        let mut ticks = Vec::with_capacity(usize::try_from(self.num_steps).unwrap());
        for a in 0..self.num_steps {
            let position = self.start_tick + self.step * (a as i128);
            let value = first_tick + self.step * (a as i128);

            ticks.push(Tick { position, value });
        }
        (display_relative, ticks)
    }

    fn new(good_steps: &[u32], ideal_num_steps: u32, range_all: [i128; 2]) -> TickLayout {
        let ideal_num_steps = ideal_num_steps.max(2);

        let range = range_all[1] - range_all[0];

        let rough_step = range as f64 / (ideal_num_steps - 1) as f64;

        //TODO replace with integer log
        let step_power = 10.0f64.powf((rough_step as f64).log10().floor()).ceil() as i128;

        let cc = good_steps.iter().map(|&normalized_step| {
            assert!(normalized_step > 0);
            let step = normalized_step as i128 * step_power;

            let start_tick = round_up_to_nearest_multiple(range_all[0], step);

            let num_steps = {
                let mut counter = start_tick;
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

            let res = TickLayout {
                step,
                normalized_step,
                num_steps,
                start_tick,
            };

            (res, (num_steps as i32 - ideal_num_steps as i32).abs())
        });

        let best = cc.min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let best = best.0;
        assert!(best.num_steps >= 2);
        best
    }
}
