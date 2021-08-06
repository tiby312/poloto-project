use core::fmt;
use fmt::Write;

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

fn make_normal(a: f64, step: Option<f64>) -> impl fmt::Display {
    crate::DisplayableClosure::new(move |fm| {
        if let Some(step) = step {
            let k = (-step.log10()).ceil();
            let k = k.max(0.0);
            write!(fm, "{0:.1$}", a, k as usize)
        } else {
            write!(fm, "{0:e}", a)
        }
    })
}

fn make_science(a: f64, step: Option<f64>) -> impl fmt::Display {
    crate::DisplayableClosure::new(move |fm| {
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
    })
}

pub fn determine_if_should_use_strat(start: f64, end: f64, step: f64) -> bool {
    let mut start_s = String::new();
    let mut end_s = String::new();

    write!(&mut start_s, "{}", interval_float(start, Some(step))).unwrap();
    write!(&mut end_s, "{}", interval_float(end, Some(step))).unwrap();

    start_s.len() > 7 || end_s.len() > 7
}

const SCIENCE: usize = 4;

/// The step amount dictates the precision we need to show at each interval
/// in order to capture the changes from each step
pub fn interval_float(a: f64, step: Option<f64>) -> impl fmt::Display {
    //TODO handle zero???
    //want to display zero with a formatting that is cosistent with others
    crate::DisplayableClosure::new(move |fm| {
        if a.abs().log10().floor().abs() > SCIENCE as f64 {
            let mut k = String::new();
            write!(&mut k, "{}", make_science(a, step))?;

            let mut j = String::new();
            write!(&mut j, "{}", make_normal(a, step))?;

            //Even if we use scientific notation,
            //it could end up as more characters
            //because of the needed precision.
            let ans = if k.len() < j.len() { k } else { j };
            write!(fm, "{}", ans)?;
        } else {
            write!(fm, "{}", make_normal(a, step))?;
        }
        Ok(())
    })
}

pub fn find_bounds<K: crate::AsF64>(
    it: impl IntoIterator<Item = [f64; 2]>,
    xmarkers: impl IntoIterator<Item = K>,
    ymarkers: impl IntoIterator<Item = K>,
) -> [f64; 4] {
    let mut ii = it
        .into_iter()
        .filter(|[x, y]| x.is_finite() && y.is_finite());

    if let Some([x, y]) = ii.next() {
        let mut val = [x, x, y, y];

        let ii = ii
            .chain(
                xmarkers
                    .into_iter()
                    .map(|x| x.as_f64())
                    .filter(|x| x.is_finite())
                    .map(|xx| [xx, y]),
            )
            .chain(
                ymarkers
                    .into_iter()
                    .map(|x| x.as_f64())
                    .filter(|x| x.is_finite())
                    .map(|yy| [x, yy]),
            );

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

        let [minx, maxx, miny, maxy] = val;

        const EPSILON: f64 = f64::MIN_POSITIVE * 10.0;

        //Insert a range if the range is zero.
        let [miny, maxy] = if (maxy - miny).abs() < EPSILON {
            [miny - 1.0, miny + 1.0]
        } else {
            [miny, maxy]
        };

        //Insert a range if the range is zero.
        let [minx, maxx] = if (maxx - minx).abs() < EPSILON {
            [minx - 1.0, minx + 1.0]
        } else {
            [minx, maxx]
        };

        [minx, maxx, miny, maxy]
    } else {
        //If there isnt any plots to draw, make up a range.
        [-1.0, 1.0, -1.0, 1.0]
    }
}

pub struct WriteCounter<T> {
    counter: usize,
    writer: T,
}
impl<T: fmt::Write> WriteCounter<T> {
    pub fn new(writer: T) -> WriteCounter<T> {
        WriteCounter { writer, counter: 0 }
    }
    pub fn get_counter(&self) -> usize {
        self.counter
    }
}
impl<T: fmt::Write> fmt::Write for WriteCounter<T> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.counter += s.len();
        self.writer.write_str(s)
    }
}
