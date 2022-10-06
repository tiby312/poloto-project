//!
//! Misc functions.
//!
//! Functions to write numbers formatted in a way that takes up the least amount of space.
//!
use super::*;
const SCIENCE: usize = 4;

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

///
/// Format a f64 with the specified precision. Formats using
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
    //want to display zero with a formatting that is consistent with others
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
pub fn write_interval_i128<T: fmt::Write>(
    mut fm: T,
    a: i128,
    step: Option<i128>,
) -> std::fmt::Result {
    //TODO handle zero???
    //want to display zero with a formatting that is consistent with others
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

pub(crate) struct WriteCounter<T> {
    counter: usize,
    writer: T,
}
impl<T: std::fmt::Write> WriteCounter<T> {
    #[inline(always)]
    pub fn new(writer: T) -> WriteCounter<T> {
        WriteCounter { writer, counter: 0 }
    }
    #[inline(always)]
    pub fn get_counter(&self) -> usize {
        self.counter
    }
}
impl<T: std::fmt::Write> std::fmt::Write for WriteCounter<T> {
    #[inline(always)]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.counter += s.len();
        self.writer.write_str(s)
    }
}

///
/// Convert a closure to a object that implements Display
///
pub(crate) fn disp_const<F: Fn(&mut fmt::Formatter) -> fmt::Result>(a: F) -> DisplayableClosure<F> {
    DisplayableClosure::new(a)
}

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
pub(crate) struct DisplayableClosure<F>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> DisplayableClosure<F> {
    #[inline(always)]
    pub fn new(a: F) -> Self {
        DisplayableClosure(a)
    }
}
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for DisplayableClosure<F> {
    #[inline(always)]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(formatter)
    }
}

///
/// Iterate over the specified range over num iterations.
///
pub fn range_iter(
    range: [f64; 2],
    num: usize,
) -> impl ExactSizeIterator<Item = f64> + Clone + Send + Sync + std::iter::FusedIterator {
    let [min, max] = range;
    let diff = max - min;
    let divf = num as f64;
    (0..num).map(move |x| min + (x as f64 / divf) * diff)
}
