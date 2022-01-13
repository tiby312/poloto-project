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


impl PlotNum for UnixTime{
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        
        let [min,max]=range;
        assert!(min<=max);

        let mind = NaiveDateTime::from_timestamp(min.0, 0);
        let maxd = NaiveDateTime::from_timestamp(max.0, 0);

        //let duration=min.signed_duration_singe(max);


        
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

        let second_difference={
            max.0-min.0
        };


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
