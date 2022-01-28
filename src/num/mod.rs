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
