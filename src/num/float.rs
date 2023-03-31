//!
//! Plot floats
//!

use super::*;

impl DiscNum for f64 {
    fn hole() -> Self {
        f64::NAN
    }
}

pub struct FloatTickFmt;

pub struct FloatFmt {
    offset: Option<f64>,
    axis: Axis,
    step: f64,
}
impl FloatFmt {
    pub fn step(&self) -> &f64 {
        &self.step
    }
    pub fn offset(&self) -> &Option<f64> {
        &self.offset
    }
}
impl crate::ticks::tick_fmt::TickFmt<f64> for FloatFmt {
    fn write_tick(&self, writer: &mut dyn std::fmt::Write, val: &f64) -> std::fmt::Result {
        let val = if let Some(offset) = self.offset {
            let val = val - offset;
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

        util::write_interval_float(writer, val, Some(self.step))
    }
    fn write_where(&self, writer: &mut dyn std::fmt::Write) -> std::fmt::Result {
        if let Some(offset) = self.offset {
            match self.axis {
                Axis::X => {
                    write!(writer, "where j=")?;
                }
                Axis::Y => {
                    write!(writer, "where k=")?;
                }
            }
            util::write_interval_float(writer, offset, None)
        } else {
            Ok(())
        }
    }
}

impl TickDistGen<f64> for FloatTickFmt {
    type Res = TickDistribution<Vec<f64>, FloatFmt>;
    fn generate(
        self,
        data: &ticks::DataBound<f64>,
        canvas: &RenderOptionsBound,
        _: IndexRequester,
    ) -> Self::Res {
        let range = &[data.min, data.max];
        let ideal_num_steps = canvas.ideal_num_steps;

        let tick_layout = TickLayout::new(&[1, 2, 5], ideal_num_steps, range);

        let (offset, ticks) = tick_layout.generate();

        let dash_size = compute_best_dash_1_2_5(
            tick_layout.step.scale(range, canvas.max),
            canvas.ideal_dash_size,
            tick_layout.normalized_step,
        );

        let axis = canvas.axis;

        TickDistribution {
            res: TickRes {
                dash_size: Some(dash_size),
            },
            iter: ticks,
            fmt: FloatFmt {
                offset,
                axis,
                step: tick_layout.step,
            },
        }
    }
}

impl plotnum::AsPlotnum for &f64 {
    type Target = f64;
    fn as_plotnum(&self) -> &Self::Target {
        self
    }
}

impl plotnum::AsPlotnum for &mut f64 {
    type Target = f64;
    fn as_plotnum(&self) -> &Self::Target {
        self
    }
}

impl HasDefaultTicks for f64 {
    type DefaultTicks = FloatTickFmt;
    fn default_ticks() -> FloatTickFmt {
        FloatTickFmt
    }
}

impl PlotNum for f64 {
    #[inline(always)]
    fn is_hole(&self) -> bool {
        self.is_nan()
    }
    #[inline(always)]
    fn scale(&self, range: &[f64; 2], max: f64) -> f64 {
        let val = *self;
        let diff = range[1] - range[0];
        let scale = max / diff;
        val * scale
    }
    #[inline(always)]
    fn unit_range(offset: Option<f64>) -> [f64; 2] {
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
    fn generate(&self) -> (Option<f64>, Vec<f64>) {
        let display_relative = {
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
                Some(tick_layout.start_tick)
            } else {
                None
            }
        };

        let mut ticks = Vec::with_capacity(usize::try_from(self.num_steps).unwrap());
        for a in 0..self.num_steps {
            let position = self.start_tick + self.step * (a as f64);

            ticks.push(position);
        }
        (display_relative, ticks)
    }

    fn new(good_steps: &[u32], ideal_num_steps: u32, range_all: &[f64; 2]) -> TickLayout {
        let ideal_num_steps = ideal_num_steps.max(2);

        let range = range_all[1] - range_all[0];

        let rough_step = range / (ideal_num_steps - 1) as f64;

        let step_power = 10.0f64.powf(rough_step.log10().floor());

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

impl HasZero for f64 {
    #[inline(always)]
    fn zero() -> f64 {
        0.0
    }
}
