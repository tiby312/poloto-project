use super::*;

///
/// A 64 bit integer unix timestamp in seconds since the epoch.
///
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct UnixTime(pub i64);

pub use chrono::ParseResult;
pub use chrono::Weekday;

fn round_up_to_nearest_multiple(val: i64, multiple: i64) -> i64 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

impl<X: chrono::TimeZone> From<chrono::DateTime<X>> for UnixTime {
    fn from(a: chrono::DateTime<X>) -> UnixTime {
        UnixTime(a.timestamp())
    }
}

impl UnixTime {
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<UnixTime> {
        NaiveDateTime::parse_from_str(s, fmt).map(|x| UnixTime(x.timestamp()))
    }

    /// Convenience function.
    pub fn from_year<T: TimeZone>(timezone: T, year: i32) -> UnixTime {
        UnixTime(timezone.yo(year, 1).and_hms(0, 0, 0).timestamp())
    }

    pub fn datetime<T: TimeZone>(&self, timezone: &T) -> DateTime<T> {
        timezone.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(self.0, 0))
    }

    /// Convenience function.
    pub fn from_ymd<T: TimeZone>(timezone: T, year: i32, month: u32, day: u32) -> UnixTime {
        UnixTime(timezone.ymd(year, month, day).and_hms(0, 0, 0).timestamp())
    }

    /// Convenience function.
    pub fn from_ymd_hms<T: TimeZone>(
        timezone: T,
        (year, month, day): (i32, u32, u32),
        hours: u32,
        min: u32,
        seconds: u32,
    ) -> UnixTime {
        UnixTime(
            timezone
                .ymd(year, month, day)
                .and_hms(hours, min, seconds)
                .timestamp(),
        )
    }

    pub fn years<T: TimeZone>(&self, timezone: &T, step_value: i64) -> UnixYears<T> {
        let this = self.datetime(timezone);

        let yy = this.year() as i64;

        let counter = round_up_to_nearest_multiple(yy, step_value) as i32;

        UnixYears {
            timezone: timezone.clone(),
            counter,
            step_value: step_value as i32,
        }
    }

    pub fn months<T: chrono::TimeZone>(&self, timezone: &T, step_value: i64) -> UnixMonths<T> {
        let this = self.datetime(timezone);

        let mm = this.month0() as i64;

        let mut m = helper::MonthCounter::new(this.year(), mm as u32);

        //round up to nearest month
        if this.day0() != 0 || this.hour() != 0 || this.minute() != 0 || this.second() != 0 {
            m.step(1);
        }

        m.round_up_to_nearest_multiple_month(step_value as u32);

        UnixMonths {
            timezone: timezone.clone(),
            counter: m,
            step_value: step_value as u32,
        }
    }

    pub fn days<T: chrono::TimeZone>(&self, timezone: &T, step_value: i64) -> UnixDays<T> {
        let this = self.datetime(timezone);

        let mut dd = this.day0() as i64;

        //round up to nearest day.
        if this.hour() != 0 || this.minute() != 0 || this.second() != 0 {
            dd += 1;
        }

        let dd = round_up_to_nearest_multiple(dd, step_value);

        let base = timezone.ymd(this.year(), this.month(), 1).and_hms(0, 0, 0);

        let counter = base.timestamp() + dd * 24 * 60 * 60;

        UnixDays {
            _timezone: timezone.clone(),
            counter,
            step_value,
        }
    }

    pub fn hours<T: chrono::TimeZone>(&self, timezone: &T, step_value: i64) -> UnixHours<T> {
        let this = self.datetime(timezone);

        let mut hh = this.hour() as i64;

        //round up to nearest hour.
        if this.minute() != 0 || this.second() != 0 {
            hh += 1;
        }
        let hh = round_up_to_nearest_multiple(hh, step_value);

        let base = this.date().and_hms(0, 0, 0);

        let counter = base.timestamp() + hh * 60 * 60;

        UnixHours {
            _timezone: this.timezone(),
            counter,
            step_value,
        }
    }

    pub fn minutes<T: TimeZone>(&self, timezone: &T, step_value: i64) -> UnixMinutes<T> {
        let this = self.datetime(timezone);

        let mut mm = this.minute() as i64;

        //round up to nearest minute
        if this.second() != 0 {
            mm += 1;
        }

        let mm = round_up_to_nearest_multiple(mm, step_value);

        let base = this.date().and_hms(this.hour(), 0, 0);

        let counter = base.timestamp() + mm * 60;

        UnixMinutes {
            _timezone: timezone.clone(),
            counter,
            step_value,
        }
    }

    pub fn seconds<T: TimeZone>(&self, timezone: &T, step_value: i64) -> UnixSeconds<T> {
        let this = self.datetime(timezone);

        let ss = this.second() as i64;

        let ss = round_up_to_nearest_multiple(ss, step_value);

        let base = this.date().and_hms(this.hour(), this.minute(), 0);

        let counter = base.timestamp() + ss;

        UnixSeconds {
            _timezone: timezone.clone(),
            counter,
            step_value,
        }
    }

    pub fn format<'a, T: TimeZone + 'a>(
        &self,
        timezone: &'a T,
        a: &'a str,
    ) -> chrono::format::DelayedFormat<chrono::format::StrftimeItems<'a>>
    where
        T::Offset: Display,
    {
        self.datetime(timezone).format(a)
    }

    pub fn dynamic_format<'a, T: TimeZone + 'a>(
        &'a self,
        timezone: &'a T,
        info: &'a TimestampType,
    ) -> impl Display + 'a {
        crate::disp_const(move |formatter| {
            let val = self.datetime(timezone);
            use TimestampType::*;

            let m = match val.month() {
                1 => "Jan",
                2 => "Feb",
                3 => "Mar",
                4 => "Apr",
                5 => "May",
                6 => "Jun",
                7 => "Jul",
                8 => "Aug",
                9 => "Sep",
                10 => "Oct",
                11 => "Nov",
                12 => "Dec",
                _ => unreachable!(),
            };

            match info {
                YR => {
                    write!(formatter, "{}", val.year())
                }
                MO => {
                    write!(formatter, "{} {}", val.year(), m)
                }
                DY => {
                    write!(formatter, "{} {}", m, val.day())
                }
                HR => {
                    write!(formatter, "{}:{}", val.weekday(), val.hour())
                }
                MI => {
                    write!(formatter, "{}:{}", val.hour(), val.minute())
                }
                SE => {
                    write!(formatter, "{}:{}", val.minute(), val.second())
                }
            }
        })
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

use chrono::TimeZone;
///
/// Used by [`UnixTime::years()`]
///
#[derive(Clone)]
pub struct UnixYears<T: TimeZone> {
    timezone: T,
    counter: i32,
    step_value: i32,
}

impl<T: TimeZone> Iterator for UnixYears<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let y = self.timezone.yo(self.counter, 1).and_hms(0, 0, 0);
        self.counter += self.step_value;
        Some(UnixTime(y.timestamp()))
    }
}

mod helper {
    use super::*;
    ///Month is zero indexed.
    #[derive(Debug, Copy, Clone)]
    pub struct MonthCounter {
        year: i32,
        month: u32,
    }
    impl MonthCounter {
        pub fn year(&self) -> i32 {
            self.year
        }
        pub fn month(&self) -> u32 {
            self.month
        }
        pub fn new(year: i32, month: u32) -> Self {
            assert!(month < 12);
            MonthCounter { year, month }
        }
        pub fn round_up_to_nearest_multiple_month(&mut self, step_value: u32) {
            let month_counter =
                round_up_to_nearest_multiple(self.month as i64, step_value as i64) as u32;
            let month1 = month_counter / 12;
            let month2 = month_counter % 12;
            self.year += month1 as i32;
            self.month = month2;
        }
        pub fn step(&mut self, step_value: u32) {
            let new = self.month + step_value;

            let new1 = new / 12;
            let new2 = new % 12;

            self.year += new1 as i32;
            self.month = new2;
        }
    }
}

///
/// Used by [`UnixTime::months()`]
///
#[derive(Debug, Clone)]
pub struct UnixMonths<T: TimeZone> {
    timezone: T,
    counter: helper::MonthCounter,
    step_value: u32,
}

impl<T: TimeZone> Iterator for UnixMonths<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let y = self
            .timezone
            .ymd(self.counter.year(), self.counter.month() + 1, 1)
            .and_hms(0, 0, 0);

        self.counter.step(self.step_value);

        Some(UnixTime(y.timestamp()))
    }
}

///
/// Used by [`UnixTime::days()`]
///
#[derive(Clone)]
pub struct UnixDays<T: TimeZone> {
    _timezone: T,
    counter: i64,
    step_value: i64,
}

impl<T: TimeZone> Iterator for UnixDays<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * 60 * 24 * self.step_value;
        Some(UnixTime(r))
    }
}

///
/// Used by [`UnixTime::hours()`]
///
#[derive(Clone)]
pub struct UnixHours<T: TimeZone> {
    _timezone: T,
    counter: i64,
    step_value: i64,
}

impl<T: TimeZone> Iterator for UnixHours<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * 60 * self.step_value;
        Some(UnixTime(r))
    }
}

///
/// Used by [`UnixTime::minutes()`]
///
#[derive(Clone)]
pub struct UnixMinutes<T: TimeZone> {
    _timezone: T,
    counter: i64,
    step_value: i64,
}
impl<T: TimeZone> Iterator for UnixMinutes<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += 60 * self.step_value;
        Some(UnixTime(r))
    }
}

///
/// Used by [`UnixTime::seconds()`]
///
#[derive(Clone)]
pub struct UnixSeconds<T: TimeZone> {
    _timezone: T,
    counter: i64,
    step_value: i64,
}
impl<T: TimeZone> Iterator for UnixSeconds<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.counter;
        self.counter += self.step_value;
        Some(UnixTime(r))
    }
}

#[test]
fn test_hours() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642232500);

    println!(
        "{}  {}  vx {} tz",
        a,
        a.datetime(&chrono::Utc).hour(),
        a.datetime(&chrono::Local).hour()
    );

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
/*

#[test]
fn test_years() {
    //1642121137
    //(2022-01-14T00:45:37+00:00)

    //let a = UnixTime(1642123584);
    let a = UnixTime::from_year(2010);

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

    //let a = UnixTime(1642232500);
    let a = UnixTime::from_ymd(2020, 8, 5);

    println!("start:{}", a);
    let mut it = a.months(1);

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


*/
