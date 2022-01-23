use super::*;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct UnixTime(pub i64);

pub use chrono::ParseResult;
pub use chrono::Weekday;

fn round_up_to_nearest_multiple(val: i64, multiple: i64) -> i64 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

impl UnixTime {
    pub fn parse_from_str(s: &str, fmt: &str) -> ParseResult<UnixTime> {
        NaiveDateTime::parse_from_str(s, fmt).map(|x| UnixTime(x.timestamp()))
    }
    pub fn from_year(year: i32) -> UnixTime {
        UnixTime(
            NaiveDateTime::new(
                NaiveDate::from_ymd(year, 1, 1),
                NaiveTime::from_hms(0, 0, 0),
            )
            .timestamp(),
        )
    }

    pub fn from_ymd(year: i32, month: u32, day: u32) -> UnixTime {
        UnixTime(
            NaiveDateTime::new(
                NaiveDate::from_ymd(year, month, day),
                NaiveTime::from_hms(0, 0, 0),
            )
            .timestamp(),
        )
    }

    pub fn from_ymd_hms(day: (i32, u32, u32), hours: u32, min: u32, seconds: u32) -> UnixTime {
        let (year, month, day) = day;
        UnixTime(
            NaiveDateTime::new(
                NaiveDate::from_ymd(year, month, day),
                NaiveTime::from_hms(hours, min, seconds),
            )
            .timestamp(),
        )
    }

    pub fn year(&self) -> i32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).year()
    }
    //1 to 12
    pub fn month(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).month()
    }

    pub fn weekday(&self) -> chrono::Weekday {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).weekday()
    }

    //1 to 30
    pub fn day(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).day()
    }

    //0 to 23
    pub fn hour(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).hour()
    }
    //0 to 59
    pub fn minute(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).minute()
    }

    //0 to 59
    pub fn second(&self) -> u32 {
        chrono::NaiveDateTime::from_timestamp(self.0, 0).second()
    }

    pub fn years(&self, step_value: i64) -> UnixYears {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let yy = this.year() as i64;

        let counter = round_up_to_nearest_multiple(yy, step_value) as i32;

        UnixYears {
            counter,
            step_value: step_value as i32,
        }
    }

    pub fn months(&self, step_value: i64) -> UnixMonths {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);

        let mm = this.month0() as i64;

        let mut m = helper::MonthCounter::new(this.year(), mm as u32);

        //round up to nearest month
        if this.day0() != 0 || this.hour() != 0 || this.minute() != 0 || this.second() != 0 {
            m.step(1);
        }

        m.round_up_to_nearest_multiple_month(step_value as u32);

        UnixMonths {
            counter: m,
            step_value: step_value as u32,
        }
    }

    pub fn days(&self, step_value: i64) -> UnixDays {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mut dd = this.day0() as i64;

        //round up to nearest day.
        if this.hour() != 0 || this.minute() != 0 || this.second() != 0 {
            dd += 1;
        }

        let dd = round_up_to_nearest_multiple(dd, step_value);

        let base = chrono::NaiveDateTime::new(
            NaiveDate::from_ymd(this.year(), this.month(), 1),
            chrono::NaiveTime::from_hms(0, 0, 0),
        );

        let counter = base.timestamp() + dd * 24 * 60 * 60;

        UnixDays {
            counter,
            step_value,
        }
    }

    pub fn hours(&self, step_value: i64) -> UnixHours {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mut hh = this.hour() as i64;

        //round up to nearest hour.
        if this.minute() != 0 || this.second() != 0 {
            hh += 1;
        }
        let hh = round_up_to_nearest_multiple(hh, step_value);

        let base = chrono::NaiveDateTime::new(this.date(), chrono::NaiveTime::from_hms(0, 0, 0));

        let counter = base.timestamp() + hh * 60 * 60;

        UnixHours {
            counter,
            step_value,
        }
    }

    pub fn minutes(&self, step_value: i64) -> UnixMinutes {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let mut mm = this.minute() as i64;

        //round up to nearest minute
        if this.second() != 0 {
            mm += 1;
        }

        let mm = round_up_to_nearest_multiple(mm, step_value);

        let base =
            chrono::NaiveDateTime::new(this.date(), chrono::NaiveTime::from_hms(this.hour(), 0, 0));

        let counter = base.timestamp() + mm * 60;

        UnixMinutes {
            counter,
            step_value,
        }
    }

    pub fn seconds(&self, step_value: i64) -> UnixSeconds {
        let this = chrono::NaiveDateTime::from_timestamp(self.0, 0);
        let ss = this.second() as i64;

        let ss = round_up_to_nearest_multiple(ss, step_value);
        let base = chrono::NaiveDateTime::new(
            this.date(),
            chrono::NaiveTime::from_hms(this.hour(), this.minute(), 0),
        );

        let counter = base.timestamp() + ss;

        UnixSeconds {
            counter,
            step_value,
        }
    }

    pub fn default_fmt<T: fmt::Write>(
        &self,
        mut formatter: T,
        step: FmtFull,
        info: TimestampType,
    ) -> fmt::Result {
        let val = self;
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

        match step {
            FmtFull::Full => {
                write!(formatter, "{}", val)
            }
            FmtFull::Short => match info {
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
            },
        }
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
#[derive(Debug, Clone)]
pub struct UnixMonths {
    counter: helper::MonthCounter,
    step_value: u32,
}

impl Iterator for UnixMonths {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let y = chrono::NaiveDateTime::new(
            chrono::NaiveDate::from_ymd(self.counter.year(), self.counter.month() + 1, 1),
            chrono::NaiveTime::from_hms(0, 0, 0),
        );

        self.counter.step(self.step_value);

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

/*
#[test]
fn test_hours() {
    // 1642123584 (update)
    // (2022-01-14T01:26:24+00:00)

    let a = UnixTime(1642232500);
    println!("start:{}", a);
    let mut it = a.hours(5);

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
