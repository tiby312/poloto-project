use super::*;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Duration;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct UnixTime(i64);

impl UnixTime {
    fn years(&self, step_value: i64) -> UnixYears {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let yy = this.year() as i64;

        let counter = (((yy + step_value) / step_value) * step_value) as i32;

        UnixYears {
            counter,
            step_value: step_value as i32,
        }
    }

    fn year(&self) -> i32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).year()
    }

    fn months(&self, step_value: i64) -> UnixMonths {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mm = this.month0() as i64;

        let month_counter = (((mm + step_value) / step_value) * step_value) as u32;

        let month1 = month_counter / 12;
        let month2 = month_counter % 12;

        UnixMonths {
            month_counter: month2,
            year_counter: this.year() + month1 as i32,
            step_value: step_value as u32,
        }
    }
    //1 to 12
    fn month(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).month()
    }

    fn days(&self, step_value: i64) -> UnixDays {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let dd = this.day0() as i64;

        let dd = ((dd + step_value) / step_value) * step_value;

        let d1 = dd / 30;
        let d2 = dd % 30;

        let month = this.month() + d1 as u32;
        let month1 = month / 12;
        let month2 = month % 12;

        let snapped = chrono::NaiveDateTime::new(
            NaiveDate::from_ymd(this.year() + month1 as i32, month2, (d2 + 1) as u32),
            NaiveTime::from_hms(0, 0, 0),
        );
        let counter = snapped.timestamp();

        UnixDays {
            counter,
            step_value,
        }
    }

    //1 to 30
    fn day(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).day()
    }

    fn hours(&self, step_value: i64) -> UnixHours {
        let hours = 60 * 60;

        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let hh = this.hour() as i64;

        let hh = ((hh + step_value) / step_value) * step_value;

        let hour1 = hh / 24;
        let hour2 = hh % 24;

        let day = this.day() + hour1 as u32;
        let day1 = day / 30;
        let day2 = day % 30;

        let month = this.month() + day1;
        let month1 = month / 12;
        let month2 = month % 12;

        let year = this.year() + month1 as i32;

        let snapped = chrono::NaiveDateTime::new(
            NaiveDate::from_ymd(year, month2, day2),
            NaiveTime::from_hms(hour2 as u32, 0, 0),
        );
        let counter = snapped.timestamp();

        UnixHours {
            counter,
            step_value,
        }
    }

    //0 to 23
    fn hour(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).hour()
    }

    fn minutes(&self, step_value: i64) -> UnixMinutes {
        let minutes = 60;

        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mm = this.minute() as i64;

        let mm = ((mm + step_value) / step_value) * step_value;

        let snapped =
            chrono::NaiveDateTime::new(this.date(), NaiveTime::from_hms(this.hour(), mm as u32, 0));
        let counter = snapped.timestamp();

        UnixMinutes {
            counter,
            step_value,
        }
    }
    //0 to 59
    fn minute(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).minute()
    }

    fn seconds(&self, step_value: i64) -> UnixSeconds {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let ss = this.second() as i64;

        let ss = ((ss + step_value) / step_value) * step_value;

        let snapped = chrono::NaiveDateTime::new(
            this.date(),
            NaiveTime::from_hms(this.hour(), this.minute(), ss as u32),
        );
        let counter = snapped.timestamp();

        UnixSeconds {
            counter,
            step_value,
        }
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

#[derive(Clone)]
pub struct UnixYears {
    counter: i32,
    step_value: i32,
}

impl Iterator for UnixYears {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let y = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_yo(self.counter, 1),
            chrono::NaiveTime::from_hms(0, 0, 0),
        );
        self.counter += self.step_value;
        Some(UnixTime(y.timestamp()))
    }
}

#[derive(Clone)]
pub struct UnixMonths {
    year_counter: i32,
    month_counter: u32,
    step_value: u32,
}

impl Iterator for UnixMonths {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let y = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd(self.year_counter, self.month_counter + 1, 1),
            chrono::NaiveTime::from_hms(0, 0, 0),
        );

        self.month_counter += self.step_value;
        if self.month_counter >= 12 {
            let extra = self.month_counter - 11;
            self.year_counter += 1;
            self.month_counter = extra;
        }

        Some(UnixTime(y.timestamp()))
    }
}

#[derive(Clone)]
pub struct UnixDays {
    counter: i64,
    step_value: i64,
}

impl Iterator for UnixDays {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * 60 * 24 * self.step_value;
        Some(UnixTime(r))
    }
}

#[derive(Clone)]
pub struct UnixHours {
    counter: i64,
    step_value: i64,
}

impl Iterator for UnixHours {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * 60 * self.step_value;
        Some(UnixTime(r))
    }
}

#[derive(Clone)]
pub struct UnixMinutes {
    counter: i64,
    step_value: i64,
}
impl Iterator for UnixMinutes {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * self.step_value;
        Some(UnixTime(r))
    }
}

#[derive(Clone)]
pub struct UnixSeconds {
    counter: i64,
    step_value: i64,
}
impl Iterator for UnixSeconds {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += self.step_value;
        Some(UnixTime(r))
    }
}

pub struct Diff {
    //total number of months
    pub num: i64,
    //if year, zero
    //if month, the offset of the first month in the context of a year (feburary would be 1).
    //if day, the offset of the first day in the context of a month (the 2nd would be 1).
    //if hour, the offset of the first hour in the context of a day (2pm would be 14).
    //if minute, the offset of the first minute in the context of an hour ()
    pub offset: i64,
}

fn round_up_to_nearest_multiple(val: i64, multiple: i64) -> i64 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

//TODO add tests comparing two timestamps and comparing iterator with num_* functions.

#[test]
fn test_hours() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642232500);
    println!("start:{}", a);
    let mut it = a.hours(1);

    for a in it.take(4) {
        println!("{}", a);
    }

    panic!();
    /*
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
    */
}

#[test]
fn test_years() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    let a = UnixTime(1642123584);

    println!("start:{}", a);
    let mut it = a.years(2);

    for a in it.take(4) {
        println!("{}", a);
    }

    panic!();
    /*
    let a = UnixTime(1642121137);

    let mut it = a.years();

    assert_eq!(2023, it.next().unwrap().year());
    assert_eq!(2024, it.next().unwrap().year());
    */
}

#[test]
fn test_minutes() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642123584);

    println!("start:{}", a);
    let mut it = a.minutes(5);

    for a in it.take(4) {
        println!("{}", a);
    }

    panic!();
    /*
    assert_eq!(27, it.next().unwrap().minute());
    assert_eq!(28, it.next().unwrap().minute());
    assert_eq!(29, it.next().unwrap().minute());
    assert_eq!(30, it.next().unwrap().minute());
    */
}

#[test]
fn test_seconds() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642123584);

    println!("start:{}", a);
    let mut it = a.seconds(5);

    for a in it.take(4) {
        println!("{}", a);
    }

    panic!();
    /*
    assert_eq!(25, it.next().unwrap().second());
    assert_eq!(26, it.next().unwrap().second());
    assert_eq!(27, it.next().unwrap().second());
    assert_eq!(28, it.next().unwrap().second());
    */
}

#[test]
fn test_months() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    let a = UnixTime(1642232500);
    println!("start:{}", a);
    let mut it = a.months(2);

    for a in it.take(4) {
        println!("{}", a);
    }

    panic!();
    /*
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
    */
}

#[test]
fn test_days() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    let a = UnixTime(1642121137);

    println!("start:{}", a);
    let mut it = a.days(5);

    for a in it.take(4) {
        println!("{}", a);
    }

    panic!();

    //assert_eq!(15, it.next().unwrap().day());
    //assert_eq!(16, it.next().unwrap().day());
}

fn collect_ticks(
    mut a: impl Iterator<Item = UnixTime>,
    max: UnixTime,
    amount: usize,
    offset: usize,
) -> Option<Vec<UnixTime>> {
    assert!(amount > 0);
    let mut v = Vec::new();
    for aa in (offset..amount - 1).cycle() {
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

/*
fn collect_ticks_all(
    mut a: impl Iterator<Item = UnixTime> + Clone,
    max: UnixTime,
    offset:usize,  //Offset is where it is in relation to the parent unit
    amount: &[usize],
    ideal_ticks: u32,
) -> Option<Vec<UnixTime>> {
    let origin = a.clone();
    let ideal_ticks: isize = ideal_ticks as isize;

    let mut res=Vec::new();

    for &a in amount{
        if let Some(r)=a.collect_ticks(origin.clone(),max,a,offset){
            res.push(r);
        }

    }

    res.into_iter().min_by(|a, b| {
        (ideal_ticks - a.len() as isize)
            .abs()
            .cmp(&(ideal_ticks - b.len() as isize).abs())
    })
}
*/

impl PlotNum for UnixTime {
    fn is_hole(&self) -> bool {
        false
    }
    fn compute_ticks(ideal_num_steps: u32, range: [Self; 2]) -> TickInfo<Self> {
        let [min, max] = range;
        assert!(min <= max);

        /*
        let arr = vec![
            (
                Unit::Year,
                collect_ticks_all(min.years(), max, &[1, 2, 5, 10,100,200,500,1000,2000,5000], ideal_num_steps),
            ),
            (
                Unit::Month,
                collect_ticks_all(min.months(), max, &[1, 2, 6, 12,24,48], ideal_num_steps),
            ),
            (
                Unit::Day,
                collect_ticks_all(min.days(), max, &[1, 2, 5,7, 10,30,60,100,365], ideal_num_steps),
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

        let valid: Vec<_> = arr
            .into_iter()
            .filter(|(_, x)| x.is_some())
            .map(|(a, x)| (a, x.unwrap()))
            .collect();

        let best = valid.into_iter().min_by(|a, b| {
            (ideal_num_steps as isize - a.1.len() as isize)
                .abs()
                .cmp(&(ideal_num_steps as isize - b.1.len() as isize).abs())
        });

        let best = best.expect("Couldnt find a good tick size");

        enum Unit {
            Year,
            Month,
            Day,
            Hour,
            Minute,
            Second,
        }
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
