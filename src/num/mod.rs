//!
//! Utility functions to help build numbers that implement [`PlotNum`] and [`DiscNum`].
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
    // so we can continue this cycle indefinately.
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
/// A distribution of steps manually specified by the user via an iterator.
///
/// Considering using contexts that automatically pick a good step distribution
/// before resulting to using this.
///
pub struct Steps<N, I, F> {
    pub steps: I,
    pub func: F,
    pub _p: PhantomData<N>,
}

impl<J: PlotNum, I: Iterator<Item = J>, F: FnMut(&mut dyn fmt::Write, &J) -> fmt::Result>
    Steps<J, I, F>
{
    pub fn new(steps: I, func: F) -> Steps<J, I, F> {
        Steps {
            steps,
            func,
            _p: PhantomData,
        }
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

impl<N, I, F> TickGenerator for Steps<N, I, F>
where
    N: PlotNum,
    I: Iterator<Item = N>,
    F: FnMut(&mut dyn fmt::Write, &N) -> fmt::Result,
{
    type Num = N;
    type Fmt = StepFmt<N, F>;

    fn generate(mut self, bound: crate::Bound<Self::Num>) -> TickDist<Self::Fmt> {
        let ticks: Vec<_> = (&mut self.steps)
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
                ticks,
                dash_size: None,
                display_relative: None,
            },
            fmt: StepFmt {
                func: self.func,
                _p: PhantomData,
            },
        }
    }
}
