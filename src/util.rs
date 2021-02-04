
pub fn find_good_step(num_steps: usize, range: f32) -> (usize, f32) {
    let range = range as f64;

    //https://stackoverflow.com/questions/237220/tickmark-algorithm-for-a-graph-axis

    let rough_step = range / (num_steps - 1) as f64;

    let step_power = 10.0f64.powf(-rough_step.abs().log10().floor()) as f64;
    let normalized_step = rough_step * step_power;

    let good_steps = [1.0, 2.0, 5.0, 10.0];
    let good_normalized_step = good_steps.iter().find(|a| **a > normalized_step).unwrap();

    let step = good_normalized_step / step_power;

    let new_step = if range % step != 0.0 {
        (range / step) as usize + 1
    } else {
        (range / step) as usize
    };

    (new_step, step as f32)
}

pub fn print_interval_float(a: f32) -> String {
    //scientific notation: m x 10n
    let n = a.log10().floor();
    let m = a / 10.0f32.powf(n);

    //Assume we have just one decimal place of precision needed
    //for fractional part.
    //This is ok because we specifically chose the intervals
    //to be from a set of desired steps (e.g. 1,2,5,10)
    if (m * 10.0).round() as usize % 10 != 0 {
        format!("{0:.1$e}", a, 1)
    } else {
        format!("{0:.1$e}", a, 0)
    }
}

pub fn find_bounds(it: impl IntoIterator<Item = [f32; 2]>) -> Option<[f32; 4]> {
    let mut ii = it.into_iter();
    if let Some([x, y]) = ii.next() {
        let mut val = [x, x, y, y];
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
        None
    }
}
