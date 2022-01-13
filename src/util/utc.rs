use chrono::DateTime;
use chrono::prelude::*;
use chrono::Duration;
use super::*;


#[derive(Eq,PartialEq,Ord,PartialOrd,Debug,Copy,Clone)]
pub struct UnixTime(i64);

impl std::fmt::Display for UnixTime{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error>{
        let naive = NaiveDateTime::from_timestamp(self.0, 0);
    
        // Create a normal DateTime from the NaiveDateTime
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
        
        // Format the datetime how you want
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

        // Print the newly formatted date and time
        write!(f,"{}",newdate)
    }
}


/*
//number of whole years
fn years(a:Duration)->i64{
    //num second sin a year:
    let num_second_in_year=365 * 24 * 60 * 60;
    a.num_seconds()/num_second_in_year
}

fn months(a:Duration)->i64{
    //num second sin a year:
    let num_second_in_month=12 * 24 * 60 * 60;
    a.num_seconds()/num_second_in_month
}
*/



fn find_good_step(good_steps: &[u32], ideal_num_steps: u32, range_all: [i128; 2]) -> (i128, u32) {
    let range = range_all[1] - range_all[0];

    let rough_step = (range / (ideal_num_steps - 1) as i128).max(1);

    let step_power = 10.0f64.powf((rough_step as f64).log10().floor()) as i128;

    let cc = good_steps.iter().map(|&x| {
        let num_steps = get_range_info(x as i128 * step_power, range_all).1;
        (x, (num_steps as i32 - ideal_num_steps as i32).abs())
    });

    let best = cc.min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    (best.0 as i128 * step_power, best.0)
}


impl PlotNum for UnixTime{
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        
        let [min,max]=range;
        assert!(min<=max);

        let mind = NaiveDateTime::from_timestamp(min.0, 0);
        let maxd = NaiveDateTime::from_timestamp(max.0, 0);
        
        let year_diff:i64={
            let min_year=mind.year();
            let max_year=maxd.year();
            (max_year-min_year) as i64

        };

        let month_difference={
            let min_month=mind.month0() as i64;
            let max_month=maxd.month0() as i64;
            (12-min_month)+year_diff+max_month
        };

        let day_difference={
            maxd.num_days_from_ce()-mind.num_days_from_ce()
        };

        let hour_difference={
            (max.0-min.0)/60/60
        };

        let min_difference={
            (max.0-min.0)/60
        };
        
        let second_difference={
            max.0-min.0
        };

        let differences=[
            (year_diff,&[1, 2, 5, 10]),
            (month_difference,&[1, 2, 6, 12]),
            (day_difference,&[1, 2, 5,10]),
            (hour_difference,&[1, 2, 5,10]),
            (minute_difference,&[1, 2, 5,10]),
            (second_difference,&[1, 2, 5,10])
        ];



        //INPUT:
        //num year
        //num month
        //num day
        //num hour
        //num second
        //ideal number of ticks
        //allowed steps
        //OUTPUT:
        //Best tick distribution

        
        let diffs=differences.into_iter().map(|range,good_ticks|{
            let (step, good_normalized_step) = find_good_step(good_ticks, ideal_num_steps, range);
            (step,good_normalized_step)   
        }).collect();

        diffs.min_by(step_diff)
   
        

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
