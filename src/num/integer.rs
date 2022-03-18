//!
//! Plot integers
//!
use super::*;

fn round_up_to_nearest_multiple(val: i128, multiple: i128) -> i128 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

pub struct IntegerTickFmt {
    step: i128,
    offset: Option<i128>,
    axis: crate::Axis,
}
impl IntegerTickFmt {
    pub fn step(&self) -> i128 {
        self.step
    }
    pub fn axis(&self) -> Axis {
        self.axis
    }
    pub fn offset(&self) -> Option<i128> {
        self.offset
    }
}
impl TickFormat for IntegerTickFmt {
    type Num = i128;

    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        let val = if let Some(offset) = self.offset {
            let val = *val - offset;
            match self.axis {
                Axis::X => {
                    write!(writer, "j+")?;
                }
                Axis::Y => {
                    write!(writer, "k+")?;
                }
            }
            val
        } else {
            *val
        };

        util::write_interval_i128(writer, val, Some(self.step))
    }
    fn write_where(&mut self, writer: &mut dyn std::fmt::Write) -> std::fmt::Result {
        if let Some(offset) = self.offset {
            match self.axis {
                Axis::X => {
                    write!(writer, "where j=")?;
                }
                Axis::Y => {
                    write!(writer, "where k=")?;
                }
            }
            util::write_interval_i128(writer, offset, None)
        } else {
            Ok(())
        }
    }
}

impl HasDefaultTicks for i128 {
    type Fmt = IntegerTickFmt;
    type Iter = std::vec::IntoIter<i128>;
    fn generate(bound: &crate::ticks::Bound<i128>) -> (TickInfo<Self::Iter>, IntegerTickFmt) {
        let range = [bound.data.min, bound.data.max];
        let ideal_num_steps = bound.canvas.ideal_num_steps;

        let tick_layout = TickLayout::new(&[1, 2, 5], ideal_num_steps, range);

        let (offset, ticks) = tick_layout.generate();

        let dash_size = Some(compute_best_dash_1_2_5(
            tick_layout.step.scale(range, bound.canvas.max),
            bound.canvas.ideal_dash_size,
            tick_layout.normalized_step,
        ));

        let axis = bound.canvas.axis;

        (
            TickInfo {
                ticks: ticks.into_iter(),
                dash_size,
            },
            IntegerTickFmt {
                step: tick_layout.step,
                offset,
                axis,
            },
        )
    }
}

impl PlotNum for i128 {
    fn scale(&self, range: [i128; 2], max: f64) -> f64 {
        let val = *self;
        let diff = (range[1] - range[0]) as f64;

        let scale = max / diff;

        val as f64 * scale
    }

    fn unit_range(offset: Option<i128>) -> [i128; 2] {
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
    fn generate(&self) -> (Option<i128>, Vec<i128>) {
        let display_relative = {
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
                Some(tick_layout.start_tick)
            } else {
                None
            }
        };

        let mut ticks = Vec::with_capacity(usize::try_from(self.num_steps).unwrap());
        for a in 0..self.num_steps {
            let position = self.start_tick + self.step * (a as i128);

            ticks.push(position);
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
