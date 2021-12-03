//!
//! Utility functions to help build numbers that implement [`PlotNum`] and [`DiscNum`].
//!
use core::fmt;

use crate::DiscNum;
use crate::PlotNum;
use crate::Tick;
use crate::TickInfo;

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
        compute_ticks_f64(ideal_num_steps, &[1, 2, 5, 10], range)
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        write_interval_float(formatter, *self, step)
    }

    fn unit_range() -> [Self; 2] {
        [-1.0, 1.0]
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
        compute_dash_size(ideal_dash_size, tick_info, range, max)
    }
}

use std::convert::TryFrom;

/// Generate out good tick interval defaults for `f64`.
pub fn compute_ticks_f64(
    ideal_num_steps: u32,
    good_ticks: &[u32],
    range: [f64; 2],
) -> TickInfo<f64> {
    let (step, good_normalized_step) = find_good_step_f64(good_ticks, ideal_num_steps, range);
    let (start_step, step_num) = get_range_info_f64(step, range);

    let display_relative = determine_if_should_use_strat(
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

    /*
    //TODO needed to aovid 10 dashes between ticks.
    //see hover shader example.
    let dash_multiple = if good_normalized_step == 10 {
        5
    } else {
        good_normalized_step
    };
    */
    let dash_multiple = good_normalized_step;

    TickInfo {
        ticks,
        dash_multiple,
        step,
        start_step,
        display_relative: display_relative.then(|| start_step),
    }
}

/// Generate out good tick interval defaults for `i128`.
pub fn compute_ticks_i128(
    ideal_num_steps: u32,
    good_ticks: &[u32],
    range: [i128; 2],
) -> TickInfo<i128> {
    //let ideal_num_steps=(ideal_num_steps as i128).min(  (range[1]-range[0]).abs() +1 ) as u32;

    let (step, good_normalized_step) = find_good_step_int(good_ticks, ideal_num_steps, range);
    let (start_step, step_num) = get_range_info_int(step, range);

    let mut ticks = Vec::with_capacity(usize::try_from(step_num).unwrap());
    for a in 0..step_num {
        let position = start_step + step * (a as i128);
        ticks.push(Tick {
            position,
            value: position,
        });
    }

    /*
    //TODO needed to aovid 10 dashes between ticks.
    //see hover shader example.
    let dash_multiple = if good_normalized_step == 10 {
        5
    } else {
        good_normalized_step
    };
    */
    let dash_multiple = good_normalized_step;

    TickInfo {
        ticks,
        step,
        start_step,
        dash_multiple,
        display_relative: None,
    }
}

///
/// Compute a good dash size that aligned with the ticks.
/// This is only compatible for ticks using `[1,2,5,10]`.
///
pub fn compute_dash_size<I: PlotNum>(
    ideal_dash_size: f64,
    tick_info: &TickInfo<I>,
    range: [I; 2],
    max: f64,
) -> Option<f64> {
    let one_step = tick_info.step.scale(range, max);
    let mut dash_multiple = tick_info.dash_multiple;

    assert!(dash_multiple > 0);

    if dash_multiple == 1 || dash_multiple == 10 {
        dash_multiple = 5;
    }

    for x in 1..50 {
        let dash_size = one_step / ((dash_multiple.pow(x)) as f64);
        if dash_size < ideal_dash_size {
            return Some(dash_size);
        }
    }
    unreachable!(
        "Could not find a good dash step size! {:?}",
        (one_step, dash_multiple, ideal_dash_size)
    );
}

///
/// A wrapper type that displays ticks at intervals that make sense for indexing to months.
/// Ticks will apear at 1,2,6,12 instead of 1,2,5,10.
/// See the month example.
///
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct MonthIndex(pub i128);

impl fmt::Display for MonthIndex {
    fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
        write!(a, "{}", self.0)
    }
}
impl PlotNum for MonthIndex {
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        let foo = [1, 2, 6, 12];

        let cc: Vec<_> = foo
            .iter()
            .map(|&step| {
                let (a, num_steps) = get_range_info_int(step, [range[0].0, range[1].0]);
                (
                    step,
                    a,
                    num_steps,
                    (ideal_num_steps as i32 - num_steps as i32).abs(),
                )
            })
            .collect();

        let (step, start_step, num_steps, _) = cc.into_iter().min_by(|a, b| a.3.cmp(&b.3)).unwrap();

        let mut ticks = Vec::with_capacity(usize::try_from(num_steps).unwrap());
        for a in 0..num_steps {
            let position = MonthIndex(start_step + step * (a as i128));
            ticks.push(Tick {
                position,
                value: position,
            });
        }

        let dash_multiple = step as u32;

        let step = MonthIndex(step);
        let start_step = MonthIndex(start_step);

        TickInfo {
            ticks,
            step,
            start_step,
            dash_multiple,
            display_relative: None,
        }
    }

    fn unit_range() -> [Self; 2] {
        [MonthIndex(-1), MonthIndex(1)]
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        write_interval_int(formatter, self.0, step.map(|x| x.0))
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let diff = (val[1].0 - val[0].0) as f64;

        let scale = max / diff;

        (self.0) as f64 * scale
    }

    fn dash_size(
        ideal_dash_size: f64,
        tick_info: &TickInfo<Self>,
        range: [Self; 2],
        max: f64,
    ) -> Option<f64> {
        let one_step = tick_info.step.scale(range, max);
        let mut dash_multiple = tick_info.dash_multiple;

        assert!(dash_multiple > 0);

        if dash_multiple == 1 || dash_multiple == 12 {
            dash_multiple = 6;
        }

        for x in 1..50 {
            let dash_size = one_step / ((dash_multiple.pow(x)) as f64);
            if dash_size < ideal_dash_size {
                return Some(dash_size);
            }
        }
        unreachable!(
            "Could not find a good dash step size! {:?}",
            (one_step, dash_multiple, ideal_dash_size)
        );
    }
}

impl PlotNum for i128 {
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        compute_ticks_i128(ideal_num_steps, &[1, 2, 5, 10], range)
    }

    fn unit_range() -> [Self; 2] {
        [-1, 1]
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        write_interval_int(formatter, *self, step)
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let diff = (val[1] - val[0]) as f64;

        let scale = max / diff;

        (*self) as f64 * scale
    }

    fn dash_size(
        ideal_dash_size: f64,
        tick_info: &TickInfo<Self>,
        range: [Self; 2],
        max: f64,
    ) -> Option<f64> {
        compute_dash_size(ideal_dash_size, tick_info, range, max)
    }
}

fn round_up_to_nearest_multiple_int(val: i128, multiple: i128) -> i128 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

fn round_up_to_nearest_multiple_f64(val: f64, multiple: f64) -> f64 {
    ((val) / multiple).ceil() * multiple
}

fn get_range_info_int(step: i128, range_all: [i128; 2]) -> (i128, u32) {
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
fn get_range_info_f64(step: f64, range_all: [f64; 2]) -> (f64, u32) {
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

fn find_good_step_int(
    good_steps: &[u32],
    ideal_num_steps: u32,
    range_all: [i128; 2],
) -> (i128, u32) {
    let range = range_all[1] - range_all[0];

    let rough_step = (range / (ideal_num_steps - 1) as i128).max(1);

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor()) as i128;

    let cc: Vec<_> = good_steps
        .iter()
        .map(|&x| {
            let num_steps = get_range_info_int(x as i128 * step_power, range_all).1;
            (x, (num_steps as i32 - ideal_num_steps as i32).abs())
        })
        .collect();

    let best = cc.into_iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    (best.0 as i128 * step_power, best.0)
}

fn find_good_step_f64(good_steps: &[u32], ideal_num_steps: u32, range_all: [f64; 2]) -> (f64, u32) {
    let range = range_all[1] - range_all[0];

    let rough_step = range / (ideal_num_steps - 1) as f64;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor());

    let cc: Vec<_> = good_steps
        .iter()
        .map(|&x| {
            let num_steps = get_range_info_f64(x as f64 * step_power, range_all).1;
            (x, (num_steps as i32 - ideal_num_steps as i32).abs())
        })
        .collect();

    let best = cc.into_iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    (best.0 as f64 * step_power, best.0)
}

fn write_normal_float<T: fmt::Write>(mut fm: T, a: f64, step: Option<f64>) -> fmt::Result {
    if let Some(step) = step {
        let k = (-step.log10()).ceil();
        let k = k.max(0.0);
        write!(fm, "{0:.1$}", a, k as usize)
    } else {
        write!(fm, "{0:e}", a)
    }
}

fn write_science_float<T: fmt::Write>(mut fm: T, a: f64, step: Option<f64>) -> fmt::Result {
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

fn determine_if_should_use_strat(start: f64, end: f64, step: f64) -> bool {
    let mut start_s = String::new();
    let mut end_s = String::new();

    write_interval_float(&mut start_s, start, Some(step)).unwrap();
    write_interval_float(&mut end_s, end, Some(step)).unwrap();

    start_s.len() > 7 || end_s.len() > 7
}

const SCIENCE: usize = 4;

///
/// Format a f64 with the specified prevision. Formats using
/// either decimal or scientific notation, whichever is shorter.
///
/// The step amount dictates the precision we need to show at each interval
/// in order to capture the changes from each step
///
/// If the step size is not specified, the number will be formatted
/// with no limit to the precision.
///
pub fn write_interval_float<T: fmt::Write>(
    mut fm: T,
    a: f64,
    step: Option<f64>,
) -> std::fmt::Result {
    //TODO handle zero???
    //want to display zero with a formatting that is cosistent with others
    if a.abs().log10().floor().abs() > SCIENCE as f64 {
        let mut k = String::new();
        write_science_float(&mut k, a, step)?;

        let mut j = String::new();
        write_normal_float(&mut j, a, step)?;

        //Even if we use scientific notation,
        //it could end up as more characters
        //because of the needed precision.
        let ans = if k.len() < j.len() { k } else { j };
        write!(fm, "{}", ans)?;
    } else {
        write_normal_float(fm, a, step)?;
    }
    Ok(())
}

///
/// Format an int Formats using either decimal or scientific notation, whichever is shorter.
///
/// If its written in scientific notation, it will do so at the precision specified.
///
/// If the step size is not specified, the number will be formatted
/// with no limit to the precision if scientific mode is picked.
///
pub fn write_interval_int<T: fmt::Write>(
    mut fm: T,
    a: i128,
    step: Option<i128>,
) -> std::fmt::Result {
    //TODO handle zero???
    //want to display zero with a formatting that is cosistent with others
    if (a.abs() as f64).log10().floor().abs() > SCIENCE as f64 {
        let mut k = String::new();
        write_science_float(&mut k, a as f64, step.map(|x| x as f64))?;

        use std::fmt::Write;
        let mut j = String::new();
        write!(&mut j, "{}", a)?;

        //Even if we use scientific notation,
        //it could end up as more characters
        //because of the needed precision.
        let ans = if k.len() < j.len() { k } else { j };
        write!(fm, "{}", ans)?;
    } else {
        write!(fm, "{}", a)?;
    }
    Ok(())
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

///
/// Wrapper around a `PlotNum` that removes the dashes for the relevant axis.
///
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct NoDash<I: PlotNum>(pub I);
impl<I: PlotNum> fmt::Display for NoDash<I> {
    fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(a)
    }
}

impl<I: DiscNum> DiscNum for NoDash<I> {
    fn hole() -> Self {
        NoDash(I::hole())
    }
}
impl<I: PlotNum> PlotNum for NoDash<I> {
    fn is_hole(&self) -> bool {
        self.0.is_hole()
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        I::compute_ticks(ideal_num_steps, [range[0].0, range[1].0]).map(|x| NoDash(x))
    }

    fn unit_range() -> [Self; 2] {
        let [a, b] = I::unit_range();
        [NoDash(a), NoDash(b)]
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        self.0.scale([val[0].0, val[1].0], max)
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        self.0.fmt_tick(formatter, step.map(|x| x.0))
    }

    fn dash_size(
        _ideal_dash_size: f64,
        _tick_info: &TickInfo<Self>,
        _range: [Self; 2],
        _max: f64,
    ) -> Option<f64> {
        None
    }
}
