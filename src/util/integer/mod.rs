use super::*;

pub mod i128;

pub use unix_timestamp::UnixTime;
pub mod unix_timestamp;

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
