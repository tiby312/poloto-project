use super::*;

impl DiscNum for f64 {
    fn hole() -> Self {
        f64::NAN
    }
}

impl PlotNum for f64 {
    fn is_hole(&self) -> bool {
        self.is_nan()
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        compute_ticks(ideal_num_steps, &[1, 2, 5, 10], range)
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        tick_fmt::write_interval_float(formatter, *self, step)
    }

    fn unit_range(offset: Option<Self>) -> [Self; 2] {
        if let Some(o) = offset {
            [o - 1.0, o + 1.0]
        } else {
            [-1.0, 1.0]
        }
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let diff = val[1] - val[0];
        let scale = max / diff;
        (*self) * scale
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

/// Generate out good tick interval defaults for `f64`.
pub fn compute_ticks(ideal_num_steps: u32, good_ticks: &[u32], range: [f64; 2]) -> TickInfo<f64> {
    let (step, good_normalized_step) = find_good_step(good_ticks, ideal_num_steps, range);
    let (start_step, step_num) = get_range_info(step, range);

    let display_relative = tick_fmt::should_fmt_offset(
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

    let dash_multiple = good_normalized_step;


    assert!(ticks.len()>=2);
    
    TickInfo {
        ticks,
        //dash_multiple,
        display_relative: display_relative.then(|| start_step),
    }
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
