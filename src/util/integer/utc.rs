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
    fn second(&self) -> u32 {
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
    type Years: IntoIterator<Item = Self> + Clone;
    type Months: IntoIterator<Item = Self> + Clone;
    type Days: IntoIterator<Item = Self> + Clone;
    type Hours: IntoIterator<Item = Self> + Clone;
    type Minutes: IntoIterator<Item = Self> + Clone;
    type Seconds: IntoIterator<Item = Self> + Clone;

    fn years(&self) -> Self::Years;
    fn months(&self) -> Self::Months;
    fn days(&self) -> Self::Days;
    fn hours(&self) -> Self::Hours;
    fn minutes(&self) -> Self::Minutes;
    fn seconds(&self) -> Self::Seconds;


    fn num_months(&self,other:UnixTime)->i64;  
    fn num_years(&self,other:UnixTime)->i64;
    fn num_days(&self,other:UnixTime)->i64;
    
    fn num_hours(&self,other:UnixTime)->i64;
    fn num_minutes(&self,other:UnixTime)->i64;
    fn num_seconds(&self,other:UnixTime)->i64;
    
}

#[derive(Clone)]
pub struct UnixYears {
    counter: i32,
}

impl Iterator for UnixYears {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let y = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_yo(self.counter, 1),
            chrono::NaiveTime::from_hms(0, 0, 0),
        );
        self.counter += 1;
        Some(UnixTime(y.timestamp()))
    }
}

#[derive(Clone)]
pub struct UnixMonths {
    year_counter: i32,
    month_counter: u32,
}

impl Iterator for UnixMonths {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        self.month_counter += 1;
        if self.month_counter > 12 {
            self.year_counter += 1;
            self.month_counter = 1;
        }
        let y = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd(self.year_counter, self.month_counter, 1),
            chrono::NaiveTime::from_hms(0, 0, 0),
        );
        Some(UnixTime(y.timestamp()))
    }
}

#[derive(Clone)]
pub struct UnixDays {
    //counter: chrono::naive::NaiveDate,
    counter:i64
}

impl Iterator for UnixDays {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * 60 * 24;
        Some(UnixTime(r))
        /*
        let d = self.counter.succ();
        let d = chrono::NaiveDateTime::new(d, chrono::NaiveTime::from_hms(0, 0, 0));

        Some(UnixTime(d.timestamp()))
        */
    }
}

#[derive(Clone)]
pub struct UnixHours {
    counter: i64,
}

impl Iterator for UnixHours {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * 60;
        Some(UnixTime(r))
    }
}

#[derive(Clone)]
pub struct UnixMinutes {
    counter: i64,
}
impl Iterator for UnixMinutes {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60;
        Some(UnixTime(r))
    }
}

#[derive(Clone)]
pub struct UnixSeconds {
    counter: i64,
}
impl Iterator for UnixSeconds {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        self.counter += 1;
        Some(UnixTime(self.counter))
    }
}

impl DateLike for UnixTime {
    type Years = UnixYears;
    type Months = UnixMonths;
    type Days = UnixDays;
    type Hours = UnixHours;
    type Minutes = UnixMinutes;
    type Seconds = UnixSeconds;

    fn years(&self) -> UnixYears {
        let mut counter = chrono::NaiveDateTime::from_timestamp(self.0, 0).year() + 1;
        UnixYears { counter }
    }

    fn num_years(&self,other:UnixTime)->i64{
        assert!(self.0<=other.0);
    
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let other = chrono::NaiveDateTime::from_timestamp(other.0, 0);
        
        other.year() as i64-this.year() as i64
    }


    fn months(&self) -> UnixMonths {
        let t = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mut month_counter = t.month();
        let mut year_counter = t.year();

        UnixMonths {
            month_counter,
            year_counter,
        }
    }

    fn num_months(&self,other:UnixTime)->i64{
        assert!(self.0<=other.0);
        
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let other = chrono::NaiveDateTime::from_timestamp(other.0, 0);
        let years=other.year()-this.year();
        let remainder=other.month0() as i64+(12-this.month0() as i64);

        years as i64*12+remainder
    }

    fn days(&self) -> UnixDays {
        let days = 60 * 60 * 24;

        let mut counter = ((self.0 + days) / days) * days;
        UnixDays { counter }
    }

    fn num_days(&self,other:UnixTime)->i64{
        assert!(self.0<=other.0);
    
        (other.0-self.0)/24/60/60
    }

    fn hours(&self) -> UnixHours {
        let hours = 60 * 60;

        let mut counter = ((self.0 + hours) / hours) * hours;
        UnixHours { counter }
    }

    fn num_hours(&self,other:UnixTime)->i64{
        assert!(self.0<=other.0);
    
        (other.0-self.0)/60/60
    }


    fn minutes(&self) -> UnixMinutes {
        let min = 60;

        let mut counter = ((self.0 + min) / min) * min;
        UnixMinutes { counter }
    }

    fn num_minutes(&self,other:UnixTime)->i64{
        assert!(self.0<=other.0);
    
        (other.0-self.0)/60
    }


    fn seconds(&self) -> UnixSeconds {
        let mut counter = self.0;
        UnixSeconds { counter }
    }

    fn num_seconds(&self,other:UnixTime)->i64{
        assert!(self.0<=other.0);
    
        (other.0-self.0)
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

fn collect_ticks_all(
    mut a: impl Iterator<Item = UnixTime> + Clone,
    max: UnixTime,
    amount: &[usize],
    ideal_ticks: u32,
) -> Option<Vec<UnixTime>> {
    let origin = a.clone();
    let ideal_ticks: isize = ideal_ticks as isize;
    let res: Vec<_> = amount
        .iter()
        .map(|a| collect_ticks(origin.clone(), max, *a))
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    res.into_iter().min_by(|a, b| {
        (ideal_ticks - a.len() as isize)
            .abs()
            .cmp(&(ideal_ticks - b.len() as isize).abs())
    })
}

fn collect_ticks(
    mut a: impl Iterator<Item = UnixTime>,
    max: UnixTime,
    amount: usize,
) -> Option<Vec<UnixTime>> {
    assert!(amount > 0);
    let mut v = Vec::new();
    for aa in (0..amount - 1).cycle() {
        if v.len() > 100 {
            //TODO better way?
            return None;
        }
        let a = a.next().unwrap();
        if a >= max {
            break;
        }

        if aa == 0 {
            v.push(a);
        }
    }
    Some(v)
}

impl PlotNum for UnixTime {
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        let [min, max] = range;
        assert!(min <= max);

        let arr = vec![
            (
                Unit::Year,
                collect_ticks_all(min.years(), max, &[1, 2, 5, 10], ideal_num_steps),
            ),
            (
                Unit::Month,
                collect_ticks_all(min.months(), max, &[1, 2, 6, 12], ideal_num_steps),
            ),
            (
                Unit::Day,
                collect_ticks_all(min.days(), max, &[1, 2, 5, 10], ideal_num_steps),
            ),
            (
                Unit::Hour,
                collect_ticks_all(min.hours(), max, &[1, 2, 5, 10], ideal_num_steps),
            ),
            (
                Unit::Minute,
                collect_ticks_all(min.minutes(), max, &[1, 2, 5, 10], ideal_num_steps),
            ),
            (
                Unit::Second,
                collect_ticks_all(min.seconds(), max, &[1, 2, 5, 10], ideal_num_steps),
            ),
        ];

        let valid:Vec<_>=arr.into_iter().filter(|(_,x)|x.is_some()).map(|(a,x)|(a,x.unwrap())).collect();
        
        
        let best=valid.into_iter().min_by(|a, b| {
            (ideal_num_steps as isize - a.1.len() as isize)
                .abs()
                .cmp(&(ideal_num_steps as isize - b.1.len() as isize).abs())});



        let best=best.expect("Couldnt find a good tick size");


        enum Unit {
            Year,
            Month,
            Day,
            Hour,
            Minute,
            Second,
        }

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
