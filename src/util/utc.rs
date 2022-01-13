use chrono::DateTime;
use chrono::prelude::*;
use chrono::Duration;
use super::*;


#[derive(Eq,PartialEq,Ord,PartialOrd,Debug,Copy,Clone)]
pub struct UnixTime(u64);

impl std::fmt::Display for UnixTime{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
        Ok(())
    }
}

impl PlotNum for UnixTime{
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {

        //determine how many years fit in.
        //  if less than ideal num,
        //     
        unimplemented!();
    }

    fn fmt_tick(
        &self,
        formatter: &mut std::fmt::Formatter,
        step: Option<Self>,
    ) -> std::fmt::Result {
        unimplemented!();
    }

    fn unit_range(offset: Option<Self>) -> [Self; 2] {
        if let Some(o)=offset{
            [o,UnixTime(o.0+1)]
        }else{
            [UnixTime(0),UnixTime(1)]    
        }
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let [val1,val2]=val;
        let [val1,val2]=[val1.0,val2.0];
        assert!(val1<=val2);
        let diff = (val2-val1) as f64;
        let scale = max / diff;
        self.0 as f64 * scale

    }

    fn dash_size(
        ideal_dash_size: f64,
        tick_info: &TickInfo<Self>,
        range: [Self; 2],
        max: f64,
    ) -> Option<f64> {
        unimplemented!();
        //compute_dash_size(ideal_dash_size, tick_info, range, max)
    }
}
