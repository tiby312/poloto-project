//!
//! Plot floats
//!

use super::*;

impl DiscNum for f64 {
    fn hole() -> Self {
        f64::NAN
    }
}

///
/// Default float context. It will attempt to find reasonable step sizes, and format them as regular floats.
///
#[derive(Default)]
pub struct FloatContext;
impl PlotNumContext for FloatContext {
    type Num = f64;
    type StepInfo = f64;

    fn tick_fmt(
        &mut self,
        writer: &mut dyn fmt::Write,
        val: Self::Num,
        _bound: [Self::Num; 2],
        info: &Self::StepInfo,
    ) -> std::fmt::Result {
        util::write_interval_float(writer, val, Some(*info))
    }

    fn where_fmt(
        &mut self,
        writer: &mut dyn std::fmt::Write,
        val: f64,
        _bound: [f64; 2],
    ) -> std::fmt::Result {
        util::write_interval_float(writer, val, None)
    }

    fn scale(&mut self, val: f64, range: [f64; 2], max: f64) -> f64 {
        let diff = range[1] - range[0];
        let scale = max / diff;
        val * scale
    }

    fn compute_ticks(
        &mut self,
        ideal_num_steps: u32,
        range: [f64; 2],
        dash: DashInfo,
    ) -> TickInfo<f64, f64> {
        let good_ticks = &[1, 2, 5];


        let tick_layout=find_good_step(good_ticks,ideal_num_steps,range);

        let TickLayout{step,normalized_step,start_tick,num_steps}=tick_layout;
        
        let display_relative = util::should_fmt_offset(
            start_tick,
            start_tick + ((num_steps - 1) as f64) * step,
            step,
        );

        let first_tick = if display_relative { 0.0 } else { start_tick };

        let ticks=tick_layout.generate(first_tick);

        assert!(ticks.len() >= 2);

        let dash_size = {
            //let dash_multiple = normalized_step;
            let max = dash.max;
            let ideal_dash_size = dash.ideal_dash_size;
            let one_step = self.scale(step, range, max);



            // On a ruler, you can see that one inch gets split
            // in a 2,2,5 fashion. Copy this.  
            let div_cycle=vec![2,2,5].into_iter().cycle();
            let start=one_step;
            let ideal=ideal_dash_size;
            let dash_size=match normalized_step{
                1=>div_find(start,ideal,div_cycle),
                2=>div_find(start,ideal,std::iter::once(2).chain(div_cycle)),
                5=>div_find(start,ideal,std::iter::once(5).chain(div_cycle)),
                _=>unreachable!()
            };


            fn div_find(mut start:f64,ideal:f64,div_cycle:impl std::iter::Iterator<Item=u32>)->f64{
                for a in div_cycle.take(1000){
                    if start<= ideal{
                        return start;
                    }
                    start/=a as f64;
                }
                unreachable!()
            }

            Some(dash_size)
        };

        TickInfo {
            unit_data: step,
            ticks,
            display_relative: display_relative.then(|| start_tick),
            dash_size,
        }
    }
    fn unit_range(&mut self, offset: Option<f64>) -> [f64; 2] {
        if let Some(o) = offset {
            [o - 1.0, o + 1.0]
        } else {
            [-1.0, 1.0]
        }
    }
}

impl PlotNum for f64 {
    fn is_hole(&self) -> bool {
        self.is_nan()
    }
}

impl HasDefaultContext for f64 {
    type DefaultContext = FloatContext;
}

fn round_up_to_nearest_multiple(val: f64, multiple: f64) -> f64 {
    ((val) / multiple).ceil() * multiple
}

fn get_range_info(step: f64, range_all: [f64; 2]) -> (f64, u32) {
    let start_step = round_up_to_nearest_multiple(range_all[0], step);

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

pub struct TickLayout{
    step:f64,
    start_tick:f64,
    num_steps:u32,
    normalized_step:u32
}
impl TickLayout{
    fn generate(&self,first_tick:f64)->Vec<Tick<f64>>{
        let mut ticks = Vec::with_capacity(usize::try_from(self.num_steps).unwrap());
        for a in 0..self.num_steps {
            let position = self.start_tick + self.step * (a as f64);
            let value = first_tick + self.step * (a as f64);

            ticks.push(Tick { position, value });
        }
        ticks

    }
}


fn find_good_step(good_steps: &[u32], ideal_num_steps: u32, range_all: [f64; 2]) -> TickLayout {
    let range = range_all[1] - range_all[0];

    let rough_step = range / (ideal_num_steps - 1) as f64;

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor());

    let cc = good_steps.iter().map(|&normalized_step| {
        assert!(normalized_step>0);
        let step=normalized_step as f64*step_power;
        let (start_tick,num_steps) = get_range_info(step, range_all);

        let res=TickLayout{
            step,
            normalized_step,
            num_steps,
            start_tick,
        };

        (res,(num_steps as i32 - ideal_num_steps as i32).abs())
    });

    let best = cc.min_by(|a, b| a.1.cmp(&b.1)).unwrap();
    best.0
}
