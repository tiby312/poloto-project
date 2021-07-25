use core::fmt;

/// Specify ideal number of steps and range.
/// Returns:
/// number of intervals.
/// size of each interval
/// first interval location.
pub fn find_good_step(num_steps: usize, range_all: [f64; 2]) -> (usize, f64, f64) {
    let range_all = [range_all[0] as f64, range_all[1] as f64];
    let range = range_all[1] - range_all[0];

    //https://stackoverflow.com/questions/237220/tickmark-algorithm-for-a-graph-axis

    let rough_step = range / (num_steps - 1) as f64;

    let step_power = 10.0f64.powf(-rough_step.abs().log10().floor()) as f64;
    let normalized_step = rough_step * step_power;

    let good_steps = [1.0, 2.0, 5.0, 10.0];
    let good_normalized_step = good_steps.iter().find(|a| **a > normalized_step).unwrap();

    let step = good_normalized_step / step_power;

    let start_step = {
        //naively find starting point.
        let aa = (range_all[0] / step).floor() * step;
        let bb = (range_all[0] / step).ceil() * step;
        if aa < bb {
            if aa < range_all[0] {
                bb
            } else {
                aa
            }
        } else if bb < range_all[0] {
            aa
        } else {
            bb
        }
    };
    assert!(start_step >= range_all[0]);

    let num_step = {
        //naively find number of steps
        let mut counter = start_step;
        let mut num = 0;
        loop {
            if counter > range_all[1] {
                break;
            }
            counter += step;

            num += 1;
        }
        num
    };
    assert!(num_step >= 1);

    // Because of the requirement for the num step to be atleast one, this assertion isnt
    // necessarily true.
    // assert!(start_step + step * ((num_step - 1) as f64) <= range_all[1]);

    (num_step, step as f64, start_step as f64)
}

fn write_normal<T: fmt::Write>(fm: &mut T, a: f64, step: Option<f64>) -> fmt::Result {
    if let Some(step) = step {
        let k = (-step.log10()).ceil();
        let k = k.max(0.0);
        write!(fm, "{0:.1$}", a, k as usize)
    } else {
        write!(fm, "{0:e}", a)
    }
}

fn write_science<T: fmt::Write>(fm: &mut T, a: f64, step: Option<f64>) -> fmt::Result {
    if let Some(step) = step {
        let precision = if a == 0.0 {
            0
        } else {
            let k1 = -step.log10().ceil();
            let k2 = -a.abs().log10().ceil();
            let k1 = k1 as isize;
            let k2 = k2 as isize;

            (k1 - k2).max(0) as usize
        };

        write!(fm, "{0:.1$e}", a, precision)
    } else {
        write!(fm, "{}", a)
    }
}

pub fn determine_if_should_use_strat(start: f64, end: f64, step: f64) -> Result<bool, fmt::Error> {
    let mut start_s = String::new();
    let mut end_s = String::new();

    interval_float(&mut start_s, start, Some(step))?;
    interval_float(&mut end_s, end, Some(step))?;

    if start_s.len() > 7 || end_s.len() > 7 {
        Ok(true)
    } else {
        Ok(false)
    }
}

const SCIENCE: usize = 4;

/// The step amount dictates the precision we need to show at each interval
/// in order to capture the changes from each step
pub fn interval_float<T: fmt::Write>(fm: &mut T, a: f64, step: Option<f64>) -> fmt::Result {
    //TODO handle zero???
    //want to display zero with a formatting that is cosistent with others

    if a.abs().log10().floor().abs() > SCIENCE as f64 {
        let mut k = String::new();
        write_science(&mut k, a, step)?;

        let mut j = String::new();
        write_normal(&mut j, a, step)?;

        //Even if we use scientific notation,
        //it could end up as more characters
        //because of the needed precision.
        let ans = if k.len() < j.len() { k } else { j };
        write!(fm, "{}", ans)?;
    } else {
        write_normal(fm, a, step)?;
    }
    Ok(())
}

pub fn find_bounds<K: PartialOrd + Copy>(
    it: impl IntoIterator<Item = [K; 2]>,
    xmarkers: impl IntoIterator<Item = K>,
    ymarkers: impl IntoIterator<Item = K>,
) -> Option<[K; 4]> {
    let mut ii = it.into_iter();

    if let Some([x, y]) = ii.next() {
        let mut val = [x, x, y, y];

        let ii = ii
            .chain(xmarkers.into_iter().map(|xx| [xx, y]))
            .chain(ymarkers.into_iter().map(|yy| [x, yy]));

        ii.fold(&mut val, |val, [x, y]| {
            if x < val[0] {
                val[0] = x;
            } else if x > val[1] {
                val[1] = x;
            }
            if y < val[2] {
                val[2] = y;
            } else if y > val[3] {
                val[3] = y;
            }
            val
        });
        Some(val)
    } else {
        //If there isnt any plots to draw, then no point looking at the markers.
        None
    }
}
