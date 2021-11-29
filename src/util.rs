use core::fmt;
use fmt::Write;

use crate::DisconectableNum;
use crate::PlotNum;
use crate::Tick;
use crate::TickInfo;

impl DisconectableNum for f64 {
    fn hole() -> Self {
        f64::NAN
    }
}

pub fn compute_ticks_f64(ideal_num_steps: usize, range: [f64; 2]) -> TickInfo<f64> {
    let (step, good_normalized_step) = find_good_step_f64(&[1, 2, 5, 10], ideal_num_steps, range);
    let (start_step, step_num) = get_range_info_f64(step, range);

    let display_relative = determine_if_should_use_strat(
        start_step,
        start_step + ((step_num - 1) as f64) * step,
        step,
    );

    let first_tick = if display_relative { 0.0 } else { start_step };

    let mut counter = 0;
    let ii = std::iter::from_fn(move || {
        if counter >= step_num {
            None
        } else {
            let position = start_step + step * (counter as f64);
            let value = first_tick + step * (counter as f64);
            counter += 1;
            Some(Tick { position, value })
        }
    })
    .fuse();

    TickInfo {
        ticks: ii.collect(),
        dash_multiple: good_normalized_step as usize,
        step,
        start_step,
        display_relative: display_relative.then(|| start_step),
    }
}
impl PlotNum for f64 {
    fn is_hole(&self) -> bool {
        self.is_nan()
    }
    fn compute_ticks(ideal_num_steps: usize, range: [Self; 2]) -> TickInfo<Self> {
        compute_ticks_f64(ideal_num_steps, range)
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        write!(formatter, "{}", crate::util::interval_float(*self, step))
    }

    fn unit_range() -> [Self; 2] {
        [-1.0, 1.0]
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let diff = val[1] - val[0];
        let scale = max / diff;
        (*self) * scale
    }
}

pub fn compute_ticks_i128(ideal_num_steps: usize, range: [i128; 2]) -> TickInfo<i128> {
    let (step, good_normalized_step) = find_good_step_int(&[1, 2, 5, 10], ideal_num_steps, range);
    let (start_step, step_num) = get_range_info_int(step, range);

    let mut counter = 0;
    let ii = std::iter::from_fn(move || {
        if counter >= step_num {
            None
        } else {
            let position = start_step + step * (counter as i128);
            counter += 1;
            Some(Tick {
                position,
                value: position,
            })
        }
    })
    .fuse();

    TickInfo {
        ticks: ii.collect(),
        step,
        start_step,
        dash_multiple: good_normalized_step as usize,
        display_relative: None,
    }
}

/*
impl PlotNum for usize {
    fn compute_ticks(ideal_num_steps: usize, range: [Self; 2]) -> TickInfo<Self> {
        compute_ticks_i128(ideal_num_steps, [range[0] as i128,range[1] as i128]).map(|x|x as usize)
    }

    fn unit_range() -> [Self; 2] {
        [0, 1]
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let diff = (val[1] - val[0]) as f64;

        let scale = max / diff;

        (*self) as f64 * scale
    }
}
*/

impl PlotNum for i128 {
    fn compute_ticks(ideal_num_steps: usize, range: [Self; 2]) -> TickInfo<Self> {
        compute_ticks_i128(ideal_num_steps, range)
    }

    fn unit_range() -> [Self; 2] {
        [-1, 1]
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let diff = (val[1] - val[0]) as f64;

        let scale = max / diff;

        (*self) as f64 * scale
    }
}

fn round_up_to_nearest_multiple_int(val: i128, multiple: i128) -> i128 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

fn round_up_to_nearest_multiple_f64(val: f64, multiple: f64) -> f64 {
    ((val) / multiple).ceil() * multiple
}

fn get_range_info_int(step: i128, range_all: [i128; 2]) -> (i128, usize) {
    let start_step = round_up_to_nearest_multiple_int(range_all[0], step);

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

//TODO handle case zero steps are found
fn get_range_info_f64(step: f64, range_all: [f64; 2]) -> (f64, usize) {
    let start_step = round_up_to_nearest_multiple_f64(range_all[0], step);

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

fn find_good_step_int(good_steps: &[u8], num_steps: usize, range_all: [i128; 2]) -> (i128, u8) {
    let range = range_all[1] - range_all[0];

    let rough_step = range / (num_steps - 1) as i128;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor()) as i128;

    let normalized_step = rough_step / step_power;

    let good_normalized_step = *good_steps
        .iter()
        .find(|a| **a as i128 > normalized_step)
        .unwrap() as i128;

    (
        good_normalized_step * step_power,
        good_normalized_step as u8,
    )
}

fn find_good_step_f64(good_steps: &[u8], num_steps: usize, range_all: [f64; 2]) -> (f64, u8) {
    let range = range_all[1] - range_all[0];

    let rough_step = range / (num_steps - 1) as f64;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor());

    let normalized_step = (rough_step / step_power) as usize;

    let good_normalized_step = *good_steps
        .iter()
        .find(|a| **a as usize > normalized_step)
        .unwrap();

    (
        good_normalized_step as f64 * step_power,
        good_normalized_step as u8,
    )
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

fn determine_if_should_use_strat(start: f64, end: f64, step: f64) -> bool {
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

pub(crate) fn find_bounds2<X: PlotNum, Y: PlotNum>(
    it: impl IntoIterator<Item = (X, Y)>,
    xmarkers: impl IntoIterator<Item = X>,
    ymarkers: impl IntoIterator<Item = Y>,
) -> ([X; 2], [Y; 2]) {
    let mut ii = it.into_iter().filter(|(x, y)| !x.is_hole() && !y.is_hole());

    if let Some((x, y)) = ii.next() {
        let mut val = ([x, x], [y, y]);
        let mut xmoved = false;
        let mut ymoved = false;
        let ii = ii
            .chain(
                xmarkers
                    .into_iter()
                    .filter(|a| !a.is_hole())
                    .map(|xx| (xx, y)),
            )
            .chain(
                ymarkers
                    .into_iter()
                    .filter(|a| !a.is_hole())
                    .map(|yy| (x, yy)),
            );

        ii.fold(&mut val, |val, (x, y)| {
            if x < val.0[0] {
                val.0[0] = x;
                xmoved = true;
            } else if x > val.0[1] {
                val.0[1] = x;
                xmoved = true;
            }
            if y < val.1[0] {
                val.1[0] = y;
                ymoved = true;
            } else if y > val.1[1] {
                val.1[1] = y;
                ymoved = true;
            }
            val
        });

        if !xmoved {
            val.0 = X::unit_range();
        }

        if !ymoved {
            val.1 = Y::unit_range();
        }

        val
    } else {
        (X::unit_range(), Y::unit_range())
    }
}

pub(crate) struct WriteCounter<T> {
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
