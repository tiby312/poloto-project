//!
//! Plot floats
//!

use super::*;

impl DiscNum for f64 {
    fn hole() -> Self {
        f64::NAN
    }
}
/*
impl PlotNumContextFromBound for FloatContext {
    fn new(bound: &crate::Bound<f64>) -> Self {
        FloatContext { bound: *bound }
    }
}
*/

pub struct FloatTickFmt {
    step: f64,
}
impl FloatTickFmt {
    pub fn step(&self)->f64{
        self.step
    }
}
impl TickFormat for FloatTickFmt {
    type Num = f64;

    fn write_tick(&self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        util::write_interval_float(writer, *val, Some(self.step))
    }
    fn write_where(&self, writer: &mut dyn std::fmt::Write, val: &Self::Num) -> std::fmt::Result {
        util::write_interval_float(writer, *val, None)
    }
}

pub struct FloatTickGen;
//TODO use this thing!!!

impl TickGenerator for FloatTickGen {
    type Num = f64;
    type Fmt = FloatTickFmt;
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
            FloatTickFmt {
                step: tick_layout.step,
            },
        )
    }
}

impl PlotNum for f64 {
    fn is_hole(&self) -> bool {
        self.is_nan()
    }
    fn default_scale(&self, range: [f64; 2], max: f64) -> f64 {
        let val = *self;
        let diff = range[1] - range[0];
        let scale = max / diff;
        val * scale
    }
    fn default_unit_range(offset: Option<f64>) -> [f64; 2] {
        if let Some(o) = offset {
            [o - 1.0, o + 1.0]
        } else {
            [-1.0, 1.0]
        }
    }
}

fn round_up_to_nearest_multiple(val: f64, multiple: f64) -> f64 {
    ((val) / multiple).ceil() * multiple
}

struct TickLayout {
    step: f64,
    start_tick: f64,
    num_steps: u32,
    normalized_step: u32,
}
impl TickLayout {
    fn generate(&self) -> (Option<f64>, Vec<Tick<f64>>) {
        let (display_relative, first_tick) = {
            let tick_layout = self;
            let end =
                tick_layout.start_tick + ((tick_layout.num_steps - 1) as f64) * tick_layout.step;

            let mut start_s = String::new();
            let mut end_s = String::new();

            util::write_interval_float(
                &mut start_s,
                tick_layout.start_tick,
                Some(tick_layout.step),
            )
            .unwrap();
            util::write_interval_float(&mut end_s, end, Some(tick_layout.step)).unwrap();

            if start_s.len() > 7 || end_s.len() > 7 {
                (Some(tick_layout.start_tick), 0.0)
            } else {
                (None, tick_layout.start_tick)
            }
        };

        let mut ticks = Vec::with_capacity(usize::try_from(self.num_steps).unwrap());
        for a in 0..self.num_steps {
            let position = self.start_tick + self.step * (a as f64);
            let value = first_tick + self.step * (a as f64);

            ticks.push(Tick { position, value });
        }
        (display_relative, ticks)
    }

    fn new(good_steps: &[u32], ideal_num_steps: u32, range_all: [f64; 2]) -> TickLayout {
        let ideal_num_steps = ideal_num_steps.max(2);

        let range = range_all[1] - range_all[0];

        let rough_step = range / (ideal_num_steps - 1) as f64;

        let step_power = 10.0f64.powf((rough_step as f64).log10().floor());

        let cc = good_steps.iter().map(|&normalized_step| {
            assert!(normalized_step > 0);
            let step = normalized_step as f64 * step_power;

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
