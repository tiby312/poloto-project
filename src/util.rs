//!
//! Misc functions.
//!
//! Functions to write numbers formatted in a way that takes up the least amount of space.
//!
use super::*;
const SCIENCE: usize = 4;

fn write_normal_float<T: sfmt::Write>(mut fm: T, a: f64, step: Option<f64>) -> sfmt::Result {
    if let Some(step) = step {
        let k = (-step.log10()).ceil();
        let k = k.max(0.0);
        write!(fm, "{0:.1$}", a, k as usize)
    } else {
        write!(fm, "{0:e}", a)
    }
}

fn write_science_float<T: sfmt::Write>(mut fm: T, a: f64, step: Option<f64>) -> sfmt::Result {
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
pub fn write_interval_float<T: sfmt::Write>(
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
pub fn write_interval_i128<T: sfmt::Write>(
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

use std::cell::RefCell;

/// Convert a moved closure into a impl fmt::Display.
/// This is useful because std's `format_args!()` macro
/// has a shorter lifetime.
pub struct DisplayableClosure<F>(pub F);

impl<F: Fn(&mut sfmt::Formatter) -> sfmt::Result> DisplayableClosure<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosure(a)
    }
}
impl<F: Fn(&mut sfmt::Formatter) -> sfmt::Result> sfmt::Display for DisplayableClosure<F> {
    fn fmt(&self, formatter: &mut sfmt::Formatter) -> sfmt::Result {
        (self.0)(formatter)
    }
}

///
/// Wrap a mutable closure in a `RefCell` to allow it to be called inside of `fmt::Display::fmt`
///
pub struct DisplayableClosureOnce<F>(pub RefCell<Option<F>>);

impl<F: FnOnce(&mut sfmt::Formatter) -> sfmt::Result> DisplayableClosureOnce<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosureOnce(RefCell::new(Some(a)))
    }
}
impl<F: FnOnce(&mut sfmt::Formatter) -> sfmt::Result> sfmt::Display for DisplayableClosureOnce<F> {
    fn fmt(&self, formatter: &mut sfmt::Formatter) -> sfmt::Result {
        if let Some(f) = (self.0.borrow_mut()).take() {
            (f)(formatter)
        } else {
            Ok(())
        }
    }
}

///
/// Wrap a mutable closure in a `RefCell` to allow it to be called inside of `fmt::Display::fmt`
///
pub struct DisplayableClosureMut<F>(pub RefCell<F>);

impl<F: FnMut(&mut sfmt::Formatter) -> sfmt::Result> DisplayableClosureMut<F> {
    pub fn new(a: F) -> Self {
        DisplayableClosureMut(RefCell::new(a))
    }
}
impl<F: FnMut(&mut sfmt::Formatter) -> sfmt::Result> sfmt::Display for DisplayableClosureMut<F> {
    fn fmt(&self, formatter: &mut sfmt::Formatter) -> sfmt::Result {
        (self.0.borrow_mut())(formatter)
    }
}

pub(crate) struct WriteCounter<T> {
    counter: usize,
    writer: T,
}
impl<T: std::fmt::Write> WriteCounter<T> {
    pub fn new(writer: T) -> WriteCounter<T> {
        WriteCounter { writer, counter: 0 }
    }
    pub fn get_counter(&self) -> usize {
        self.counter
    }
}
impl<T: std::fmt::Write> std::fmt::Write for WriteCounter<T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.counter += s.len();
        self.writer.write_str(s)
    }
}

pub(crate) fn find_bounds<X: PlotNum, Y: PlotNum>(
    it: impl IntoIterator<Item = (X, Y)>,
    xmarkers: Vec<X>,
    ymarkers: Vec<Y>,
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
                if !xmoved {
                    xmoved = true
                };
            } else if x > val.0[1] {
                val.0[1] = x;
                if !xmoved {
                    xmoved = true
                };
            }
            if y < val.1[0] {
                val.1[0] = y;
                if !ymoved {
                    ymoved = true
                };
            } else if y > val.1[1] {
                val.1[1] = y;
                if !ymoved {
                    ymoved = true
                };
            }
            val
        });

        if !xmoved {
            val.0 = X::default_unit_range(Some(x));
        }

        if !ymoved {
            val.1 = Y::default_unit_range(Some(y));
        }

        val
    } else {
        (X::default_unit_range(None), Y::default_unit_range(None))
    }
}
