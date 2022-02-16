//!
//! Some impls of [`PlotNum`] and some functions that product a [`TickDist`].
//!
use super::*;
use crate::util;
use crate::DiscNum;
use crate::PlotNum;
use crate::Tick;
use crate::TickInfo;
use core::fmt;

pub mod float;
pub mod integer;

#[cfg(feature = "timestamp")]
pub mod timestamp;

use std::convert::TryFrom;

fn compute_best_dash_1_2_5(one_step: f64, ideal_dash_size: f64, normalized_step: u32) -> f64 {
    assert!(normalized_step == 1 || normalized_step == 2 || normalized_step == 5);

    // On a ruler, you can see one cm is split into 0.5 steps.
    // those 0.5 steps are split into 0.1 cm steps.
    //
    // alternatively, we could go a 5,2. Split 1cm
    // into 2,4,6,8mm. And then split those into 1m.
    //
    // in both cases, the end result is that we now have
    // a base unit that is a 10th of what we had originally,
    // so we can continue this cycle indefinitely.
    //
    // lets chose whichever one is better.

    let div_cycle1 = vec![2, 5].into_iter().cycle();
    let div_cycle2 = vec![5, 2].into_iter().cycle();

    let start = one_step;
    let ideal = ideal_dash_size;

    fn div_find_best(
        start: f64,
        ideal: f64,
        div_cycle1: impl std::iter::Iterator<Item = u32>,
        div_cycle2: impl std::iter::Iterator<Item = u32>,
    ) -> f64 {
        fn div_find(
            mut start: f64,
            ideal: f64,
            div_cycle: impl std::iter::Iterator<Item = u32>,
        ) -> f64 {
            let mut last = start;
            for a in div_cycle.take(1000) {
                if start <= ideal {
                    return if (last - ideal).abs() < (start - ideal).abs() {
                        last
                    } else {
                        start
                    };
                }
                last = start;
                start /= a as f64;
            }
            unreachable!()
        }

        let a = div_find(start, ideal, div_cycle1);
        let b = div_find(start, ideal, div_cycle2);
        if (a - ideal).abs() < (b - ideal).abs() {
            a
        } else {
            b
        }
    }

    match normalized_step {
        1 => div_find_best(start, ideal, div_cycle1, div_cycle2),
        2 => div_find_best(
            start,
            ideal,
            std::iter::once(2).chain(div_cycle1),
            std::iter::once(2).chain(div_cycle2),
        ),
        5 => div_find_best(
            start,
            ideal,
            std::iter::once(5).chain(div_cycle1),
            std::iter::once(5).chain(div_cycle2),
        ),
        _ => unreachable!(),
    }
}

///
/// Create a [`plotnum::TickDist`] from a step iterator.
///
pub fn steps<
    X: PlotNum,
    I: Iterator<Item = X>,
    F: FnMut(&mut dyn fmt::Write, &X) -> fmt::Result,
>(
    bound: Bound<X>,
    steps: I,
    func: F,
) -> plotnum::TickDist<num::StepFmt<X, F>> {
    let ticks: Vec<_> = steps
        .skip_while(|&x| x < bound.min)
        .take_while(|&x| x <= bound.max)
        .map(|x| Tick {
            value: x,
            position: x,
        })
        .collect();

    assert!(
        ticks.len() >= 2,
        "Atleast two ticks must be created for the given data range."
    );

    TickDist {
        ticks: TickInfo {
            bound,
            ticks,
            dash_size: None,
            display_relative: None,
        },
        fmt: StepFmt {
            func,
            _p: PhantomData,
        },
    }
}

pub struct StepFmt<J, F> {
    func: F,
    _p: PhantomData<J>,
}
impl<J: PlotNum, F: FnMut(&mut dyn fmt::Write, &J) -> fmt::Result> TickFormat for StepFmt<J, F> {
    type Num = J;
    fn write_tick(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: &Self::Num,
    ) -> std::fmt::Result {
        (self.func)(writer, val)
    }
}
