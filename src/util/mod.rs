//!
//! Utility functions to help build numbers that implement [`PlotNum`] and [`DiscNum`].
//!
use super::*;
use crate::tick_fmt;
use crate::DiscNum;
use crate::PlotNum;
use crate::Tick;
use crate::TickInfo;
use core::fmt;

pub mod f64_;
pub mod integer;

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

pub(crate) fn find_bounds<X: PlotNumContext, Y: PlotNumContext>(
    xcontext: &mut X,
    ycontext: &mut Y,
    it: impl IntoIterator<Item = (X::Num, Y::Num)>,
) -> ([X::Num; 2], [Y::Num; 2]) {
    let xmarkers = xcontext.get_markers();
    let ymarkers = ycontext.get_markers();

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
            val.0 = xcontext.unit_range(Some(x));
        }

        if !ymoved {
            val.1 = ycontext.unit_range(Some(y));
        }

        val
    } else {
        (xcontext.unit_range(None), ycontext.unit_range(None))
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

/*
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
    type Context = I::Context;

    fn is_hole(&self) -> bool {
        self.0.is_hole()
    }
    fn compute_ticks(
        ideal_num_steps: u32,
        range: [Self; 2],
        dash: DashInfo,
    ) -> TickInfo<Self, Self::UnitData> {
        let mut k=I::compute_ticks(ideal_num_steps, [range[0].0, range[1].0], dash).map(NoDash, |x| x);
        k.dash_size=None;
        k
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
        step: Self::UnitData,
        fmt: FmtFull,
    ) -> std::fmt::Result {
        self.0.fmt_tick(formatter, step, fmt)
    }

    /*
    fn dash_size(
        _ideal_dash_size: f64,
        _tick_info: &TickInfo<Self, Self::UnitData>,
        _range: [Self; 2],
        _max: f64,
    ) -> Option<f64> {
        None
    }*/
}

*/
