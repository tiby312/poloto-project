use super::*;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct UnixTime(i64);

impl UnixTime {
    fn year(&self) -> i32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).year()
    }

    //1 to 12
    fn month(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).month()
    }

    //1 to 30
    fn day(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).day()
    }

    //0 to 23
    fn hour(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).hour()
    }

    //0 to 59
    fn minute(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).minute()
    }

    //0 to 59
    fn second(&self)->u32{
        chrono::NaiveDateTime::from_timestamp(self.0, 0).second()
    }
}
impl std::fmt::Display for UnixTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let naive = NaiveDateTime::from_timestamp(self.0, 0);

        // Create a normal DateTime from the NaiveDateTime
        let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

        // Format the datetime how you want
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");

        // Print the newly formatted date and time
        write!(f, "{}", newdate)
    }
}



pub trait DateLike {
    fn years(&self) -> Box<dyn Iterator<Item = Self>>;
    fn months(&self) -> Box<dyn Iterator<Item = Self>>;
    fn days(&self) -> Box<dyn Iterator<Item = Self>>;
    fn hours(&self) -> Box<dyn Iterator<Item = Self>>;
    fn minutes(&self)->Box<dyn Iterator<Item=Self>>;
    fn seconds(&self)->Box<dyn Iterator<Item=Self>>;

}

impl DateLike for UnixTime {
    fn years(&self) -> Box<dyn Iterator<Item = Self>> {
        let mut counter = chrono::NaiveDateTime::from_timestamp(self.0, 0).year() + 1;
        Box::new(std::iter::repeat_with(move || {
            let y = chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_yo(counter, 1),
                chrono::NaiveTime::from_hms(0, 0, 0),
            );
            counter += 1;
            UnixTime(y.timestamp())
        }))
    }

    fn months(&self) -> Box<dyn Iterator<Item = Self>> {
        let t = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mut month_counter = t.month();
        let mut year_counter = t.year();

        Box::new(std::iter::repeat_with(move || {
            month_counter += 1;
            if month_counter > 12 {
                year_counter += 1;
                month_counter = 1;
            }
            let y = chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd(year_counter, month_counter, 1),
                chrono::NaiveTime::from_hms(0, 0, 0),
            );
            UnixTime(y.timestamp())
        }))
    }

    fn days(&self) -> Box<dyn Iterator<Item = Self>> {
        let mut t = chrono::NaiveDateTime::from_timestamp(self.0, 0)
            .date()
            .iter_days();
        let _ = t.next().unwrap();

        Box::new(std::iter::repeat_with(move || {
            let d = t.next().unwrap();
            let d = chrono::NaiveDateTime::new(d, chrono::NaiveTime::from_hms(0, 0, 0));

            UnixTime(d.timestamp())
        }))
    }

    fn hours(&self) -> Box<dyn Iterator<Item = Self>> {

        let hours=60*60;

        let mut counter=((self.0+hours)/hours)*hours;
        Box::new(std::iter::repeat_with(move ||{
            let r=counter;
            counter+=hours;
            UnixTime(r)
        }))
    }

    fn minutes(&self) -> Box<dyn Iterator<Item = Self>> {

        let min=60;

        let mut counter=((self.0+min)/min)*min;
        Box::new(std::iter::repeat_with(move ||{
            let r=counter;
            counter+=min;
            UnixTime(r)
        }))
    }

    fn seconds(&self) -> Box<dyn Iterator<Item = Self>> {
        let mut counter=self.0;
        Box::new(std::iter::repeat_with(move ||{
            counter+=1;
            UnixTime(counter)
        }))
    }

}

#[test]
fn test_years() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    let a = UnixTime(1642121137);

    let mut it = a.years();

    assert_eq!(2023, it.next().unwrap().year());
    assert_eq!(2024, it.next().unwrap().year());
}

#[test]
fn test_hours() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642123584);

    let mut it = a.hours();

    assert_eq!(2, it.next().unwrap().hour());
    assert_eq!(3, it.next().unwrap().hour());
    assert_eq!(4, it.next().unwrap().hour());
    assert_eq!(5, it.next().unwrap().hour());
    assert_eq!(6, it.next().unwrap().hour());
    assert_eq!(7, it.next().unwrap().hour());
    assert_eq!(8, it.next().unwrap().hour());
    assert_eq!(9, it.next().unwrap().hour());
    assert_eq!(10, it.next().unwrap().hour());
    assert_eq!(11, it.next().unwrap().hour());
    assert_eq!(12, it.next().unwrap().hour());
    assert_eq!(13, it.next().unwrap().hour());
    assert_eq!(14, it.next().unwrap().hour());
    assert_eq!(15, it.next().unwrap().hour());
    assert_eq!(16, it.next().unwrap().hour());
    assert_eq!(17, it.next().unwrap().hour());
    assert_eq!(18, it.next().unwrap().hour());
    assert_eq!(19, it.next().unwrap().hour());
    assert_eq!(20, it.next().unwrap().hour());
    assert_eq!(21, it.next().unwrap().hour());
    assert_eq!(22, it.next().unwrap().hour());
    assert_eq!(23, it.next().unwrap().hour());
    assert_eq!(0, it.next().unwrap().hour());
    assert_eq!(1, it.next().unwrap().hour());
    assert_eq!(2, it.next().unwrap().hour());
}

#[test]
fn test_minutes() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642123584);

    let mut it = a.minutes();

    assert_eq!(27, it.next().unwrap().minute());
    assert_eq!(28, it.next().unwrap().minute());
    assert_eq!(29, it.next().unwrap().minute());
    assert_eq!(30, it.next().unwrap().minute());

}


#[test]
fn test_seconds() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642123584);

    let mut it = a.seconds();

    assert_eq!(25, it.next().unwrap().second());
    assert_eq!(26, it.next().unwrap().second());
    assert_eq!(27, it.next().unwrap().second());
    assert_eq!(28, it.next().unwrap().second());

}

#[test]
fn test_months() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    let a = UnixTime(1642121137);

    let mut it = a.months();

    //assert_eq!(1,it.next().unwrap().month());
    assert_eq!(2, it.next().unwrap().month());
    assert_eq!(3, it.next().unwrap().month());
    assert_eq!(4, it.next().unwrap().month());
    assert_eq!(5, it.next().unwrap().month());
    assert_eq!(6, it.next().unwrap().month());
    assert_eq!(7, it.next().unwrap().month());
    assert_eq!(8, it.next().unwrap().month());
    assert_eq!(9, it.next().unwrap().month());
    assert_eq!(10, it.next().unwrap().month());
    assert_eq!(11, it.next().unwrap().month());
    assert_eq!(12, it.next().unwrap().month());
    assert_eq!(1, it.next().unwrap().month());
    assert_eq!(2, it.next().unwrap().month());
}

#[test]
fn test_days() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    let a = UnixTime(1642121137);

    let mut it = a.days();

    assert_eq!(15, it.next().unwrap().day());
    assert_eq!(16, it.next().unwrap().day());
}

impl PlotNum for UnixTime {
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        let [min, max] = range;
        assert!(min <= max);

        let mind = NaiveDateTime::from_timestamp(min.0, 0);
        let maxd = NaiveDateTime::from_timestamp(max.0, 0);

        let year_diff: i64 = {
            let min_year = mind.year();
            let max_year = maxd.year();
            (max_year - min_year) as i64
        };

        let month_difference = {
            let min_month = mind.month0() as i64;
            let max_month = maxd.month0() as i64;
            (12 - min_month) + year_diff + max_month
        };

        let day_difference = { (maxd.num_days_from_ce() - mind.num_days_from_ce()) as i64 };

        let hour_difference = { (max.0 - min.0) / 60 / 60 };

        let minute_difference = { (max.0 - min.0) / 60 };

        let second_difference = { max.0 - min.0 };

        enum Unit {
            Year,
            Month,
            Day,
            Hour,
            Minute,
            Second,
        }

        struct Unit2<'a> {
            diff: i64,
            stepa: &'a [u32],
            unit: Unit,
        }

        let differences = [
            Unit2 {
                diff: year_diff,
                stepa: &[1, 2, 5, 10],
                unit: Unit::Year,
            },
            Unit2 {
                diff: month_difference,
                stepa: &[1, 2, 6, 12],
                unit: Unit::Month,
            },
            Unit2 {
                diff: day_difference,
                stepa: &[1, 2, 5, 10],
                unit: Unit::Day,
            },
            Unit2 {
                diff: hour_difference,
                stepa: &[1, 2, 5, 10],
                unit: Unit::Hour,
            },
            Unit2 {
                diff: minute_difference,
                stepa: &[1, 2, 5, 10],
                unit: Unit::Minute,
            },
            Unit2 {
                diff: second_difference,
                stepa: &[1, 2, 5, 10],
                unit: Unit::Second,
            },
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

        /*
        let diffs=differences.into_iter().map(|unit|{
            let g=find_good_step(unit.stepa, ideal_num_steps, unit.diff);
            let diff=g.num_steps-ideal_num_steps;
            ( diff.abs(),g,unit.unit)
        }).collect();

        let best = diffs.min_by(|a, b| a.0.cmp(&b.0)).unwrap();
        */

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
        if let Some(o) = offset {
            [o, UnixTime(o.0 + 1)]
        } else {
            [UnixTime(0), UnixTime(1)]
        }
    }

    fn scale(&self, val: [Self; 2], max: f64) -> f64 {
        let [val1, val2] = val;
        let [val1, val2] = [val1.0, val2.0];
        assert!(val1 <= val2);
        let diff = (val2 - val1) as f64;
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
