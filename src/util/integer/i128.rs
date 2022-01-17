use super::*;

#[derive(Copy, Clone, Debug)]
pub struct StepAmount(pub i128);

/*
/// Generate out good tick interval defaults for `i128`.
pub fn compute_ticks(
    ideal_num_steps: u32,
    good_ticks: &[u32],
    range: [i128; 2],
) -> TickInfo<i128, StepAmount> {

}
*/

#[derive(Default)]
pub struct Defaulti128Context;

impl PlotNumContext for Defaulti128Context {
    type Num = i128;
    type UnitData = StepAmount;
    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [i128; 2],
        dash: DashInfo,
    ) -> TickInfo<i128, StepAmount> {
        let good_ticks = &[1, 2, 5, 10];

        let (step, good_normalized_step) = find_good_step(good_ticks, ideal_num_steps, range);
        let (start_step, step_num) = get_range_info(step, range);

        let display_relative = tick_fmt::should_fmt_offset(
            start_step as f64,
            (start_step + ((step_num - 1) as i128) * step) as f64,
            step as f64,
        );

        let first_tick = if display_relative { 0 } else { start_step };

        let mut ticks = Vec::with_capacity(usize::try_from(step_num).unwrap());
        for a in 0..step_num {
            let position = start_step + step * (a as i128);
            let value = first_tick + step * (a as i128);

            ticks.push(Tick { position, value });
        }

        let dash_size = {
            let dash_multiple = good_normalized_step;
            let max = dash.max;
            let ideal_dash_size = dash.ideal_dash_size;
            let one_step = self.scale(step, range, max);

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
        };

        TickInfo {
            unit_data: StepAmount(step),
            ticks,
            dash_size,
            display_relative: display_relative.then(|| start_step),
        }
    }

    fn unit_range(&mut self, offset: Option<i128>) -> [i128; 2] {
        if let Some(o) = offset {
            [o - 1, o + 1]
        } else {
            [-1, 1]
        }
    }

    fn fmt_tick<T: std::fmt::Write>(
        &mut self,
        formatter: T,
        val: i128,
        step: StepAmount,
        fmt: FmtFull,
    ) -> std::fmt::Result {
        let step = match fmt {
            FmtFull::Tick => Some(step.0),
            FmtFull::Full => None,
        };
        tick_fmt::write_interval_i128(formatter, val, step)
    }

    fn scale(&mut self, val: i128, range: [i128; 2], max: f64) -> f64 {
        let diff = (range[1] - range[0]) as f64;

        let scale = max / diff;

        val as f64 * scale
    }
}

impl PlotNum for i128 {}

/*
pub use month::MonthIndex;
mod month {
    use super::*;

    ///
    /// A wrapper type that displays ticks at intervals that make sense for indexing to months.
    /// Ticks will appear at 1,2,6,12 instead of 1,2,5,10.
    /// See the month example.
    ///
    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
    pub struct MonthIndex(pub i128);

    impl fmt::Display for MonthIndex {
        fn fmt(&self, a: &mut fmt::Formatter) -> fmt::Result {
            write!(a, "{}", self.0)
        }
    }
    impl PlotNum for MonthIndex {
        type UnitData = <i128 as PlotNum>::UnitData;
        fn compute_ticks(ideal_num_steps: u32, range: [Self; 2],dash:DashInfo) -> TickInfo<Self, Self::UnitData> {
            let cc = [1, 2, 6, 12].iter().map(|&step| {
                let (a, num_steps) = get_range_info(step, [range[0].0, range[1].0]);
                (
                    step,
                    a,
                    num_steps,
                    (ideal_num_steps as i32 - num_steps as i32).abs(),
                )
            });

            let (step, start_step, num_steps, _) = cc.min_by(|a, b| a.3.cmp(&b.3)).unwrap();

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

            assert!(ticks.len() >= 2);

            TickInfo {
                unit_data: StepAmount(step.0),
                ticks,
                //dash_multiple,
                display_relative: None,
            }
        }

        fn unit_range(offset: Option<Self>) -> [Self; 2] {
            if let Some(o) = offset {
                [MonthIndex(o.0 - 1), MonthIndex(o.0 + 1)]
            } else {
                [MonthIndex(-1), MonthIndex(1)]
            }
        }

        fn fmt_tick(
            &self,
            formatter: &mut std::fmt::Formatter,
            step: Self::UnitData,
            fmt: FmtFull,
        ) -> std::fmt::Result {
            tick_fmt::write_interval_i128(formatter, self.0, None)
        }

        fn scale(&self, val: [Self; 2], max: f64) -> f64 {
            let diff = (val[1].0 - val[0].0) as f64;

            let scale = max / diff;

            (self.0) as f64 * scale
        }

        fn dash_size(
            ideal_dash_size: f64,
            tick_info: &TickInfo<Self, Self::UnitData>,
            range: [Self; 2],
            max: f64,
        ) -> Option<f64> {
            None
            /*
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
            */
        }
    }
}
*/
