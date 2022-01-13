//!
//! Utility functions to help build numbers that implement [`PlotNum`] and [`DiscNum`].
//!
use crate::tick_fmt;
use crate::DiscNum;
use crate::PlotNum;
use crate::Tick;
use crate::TickInfo;
use core::fmt;

pub use self::f64::compute_ticks as compute_ticks_f64;
pub use self::i128::compute_ticks as compute_ticks_i128;
mod f64;
mod i128;

pub use self::i128::month::MonthIndex;

use std::convert::TryFrom;

fn test_multiple<I: PlotNum>(
    ideal_dash_size: f64,
    one_step: f64,
    dash_multiple: u32,
    _range: [I; 2],
    _max: f64,
) -> Option<f64> {
    assert!(dash_multiple > 0);

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
    let dash_multiple = tick_info.dash_multiple;

    assert!(dash_multiple > 0);

    if dash_multiple == 1 || dash_multiple == 10 {
        let a = test_multiple(ideal_dash_size, one_step, 2, range, max).unwrap();
        let b = test_multiple(ideal_dash_size, one_step, 5, range, max).unwrap();
        if (a - ideal_dash_size).abs() < (b - ideal_dash_size).abs() {
            Some(a)
        } else {
            Some(b)
        }
    } else {
        Some(test_multiple(ideal_dash_size, one_step, dash_multiple, range, max).unwrap())
    }
}

pub(crate) fn find_bounds<X: PlotNum, Y: PlotNum>(
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
            val.0 = X::unit_range(Some(x));
        }

        if !ymoved {
            val.1 = Y::unit_range(Some(y));
        }

        val
    } else {
        (X::unit_range(None), Y::unit_range(None))
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
/// Shorthand to easily make a plot without dashed lines. See `nodash` example.
///
pub fn no_dash_tuple<A: crate::Plottable<X, Y>, X: PlotNum, Y: PlotNum>(
    a: A,
) -> (NoDash<X>, NoDash<Y>) {
    let (a, b) = a.make_plot();
    (NoDash(a), NoDash(b))
}
///
/// Wrapper around a `PlotNum` that removes the dashes for the relevant axis.
///
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct NoDash<I: PlotNum>(pub I);
impl<I: PlotNum> fmt::Display for NoDash<I> {
    fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, a)
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
        I::compute_ticks(ideal_num_steps, [range[0].0, range[1].0]).map(NoDash)
    }

    fn unit_range(o: Option<Self>) -> [Self; 2] {
        let [a, b] = I::unit_range(o.map(|x| x.0));
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