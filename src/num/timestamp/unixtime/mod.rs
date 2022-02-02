use super::*;

#[cfg(test)]
mod tests;

///
/// Common trait among unix time step generators.
///
pub(crate) trait UnixTimeGenerator {
    type Iter: Iterator<Item = UnixTime>;
    fn generate(&self, step_size: i64) -> Self::Iter;
}

///
/// A 64 bit integer unix timestamp in seconds since the epoch in UTC.
///
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct UnixTime(pub i64);

fn round_up_to_nearest_multiple(val: i64, multiple: i64) -> i64 {
    let ss = if val >= 0 { multiple - 1 } else { 0 };

    ((val + ss) / multiple) * multiple
}

impl<X: chrono::TimeZone> From<chrono::DateTime<X>> for UnixTime {
    fn from(a: chrono::DateTime<X>) -> UnixTime {
        UnixTime(a.timestamp())
    }
}

impl<X: chrono::TimeZone> From<chrono::Date<X>> for UnixTime {
    fn from(a: chrono::Date<X>) -> UnixTime {
        UnixTime(a.and_hms(0, 0, 0).timestamp())
    }
}

impl From<UnixTime> for chrono::DateTime<chrono::Utc> {
    fn from(a: UnixTime) -> Self {
        a.datetime(&chrono::Utc)
    }
}

///
/// Used by [`UnixTime::dynamic_format()`]
///
pub struct DynamicFormatter<T: TimeZone> {
    a: UnixTime,
    timezone: T,
    info: StepUnit,
}
impl<T: TimeZone> Display for DynamicFormatter<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let timezone = &self.timezone;
        let info = self.info;
        let val = self.a.datetime(timezone);
        use StepUnit::*;

        let m = month_str(val.month());

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
    }
}

impl UnixTime {
    ///
    /// Convert to a `chrono::DateTime` object.
    ///
    pub fn datetime<T: TimeZone>(&self, timezone: &T) -> DateTime<T> {
        timezone.timestamp(self.0, 0)
    }

    pub(crate) fn years<T: TimeZone>(&self, timezone: &T) -> UnixYearGenerator<T> {
        UnixYearGenerator {
            val: *self,
            timezone: timezone.clone(),
        }
    }

    pub(crate) fn months<T: TimeZone>(&self, timezone: &T) -> UnixMonthGenerator<T> {
        UnixMonthGenerator {
            val: *self,
            timezone: timezone.clone(),
        }
    }

    pub(crate) fn days<T: TimeZone>(&self, timezone: &T) -> UnixDayGenerator<T> {
        UnixDayGenerator {
            val: *self,
            timezone: timezone.clone(),
        }
    }

    pub(crate) fn hours<T: TimeZone>(&self, timezone: &T) -> UnixHourGenerator<T> {
        UnixHourGenerator {
            val: *self,
            timezone: timezone.clone(),
        }
    }

    pub(crate) fn minutes<T: TimeZone>(&self, timezone: &T) -> UnixMinuteGenerator<T> {
        UnixMinuteGenerator {
            val: *self,
            timezone: timezone.clone(),
        }
    }

    pub(crate) fn seconds<T: TimeZone>(&self, timezone: &T) -> UnixSecondGenerator<T> {
        UnixSecondGenerator {
            val: *self,
            timezone: timezone.clone(),
        }
    }

    ///
    /// Format a UnixTime with the given step unit specified.
    /// It will display that unit plus, the unit one above.
    /// So for example, if the step size is in hours,
    /// it will display the hour as well as the day of the week.
    ///
    pub fn dynamic_format<'a, T: TimeZone + 'a>(
        &'a self,
        timezone: &'a T,
        info: &'a StepUnit,
    ) -> DynamicFormatter<T> {
        DynamicFormatter {
            timezone: timezone.clone(),
            info: info.clone(),
            a: self.clone(),
        }
    }
}
impl std::fmt::Display for UnixTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let datetime = chrono::Utc.timestamp(self.0, 0);

        // Print the newly formatted date and time
        write!(f, "{}", datetime)
    }
}

///
/// Used by [`UnixTime::years()`]
///
pub(crate) struct UnixYearGenerator<T: TimeZone> {
    val: UnixTime,
    timezone: T,
}

impl<T: TimeZone> UnixTimeGenerator for UnixYearGenerator<T> {
    type Iter = UnixYears<T>;
    fn generate(&self, step_value: i64) -> Self::Iter {
        let this = self.val.datetime(&self.timezone);

        let yy = this.year() as i64;

        let counter = round_up_to_nearest_multiple(yy, step_value) as i32;

        UnixYears {
            timezone: self.timezone.clone(),
            counter,
            step_value: step_value as i32,
        }
    }
}

///
/// Used by [`UnixTime::months()`]
///
pub(crate) struct UnixMonthGenerator<T: TimeZone> {
    val: UnixTime,
    timezone: T,
}

impl<T: TimeZone> UnixTimeGenerator for UnixMonthGenerator<T> {
    type Iter = UnixMonths<T>;
    fn generate(&self, step_value: i64) -> Self::Iter {
        let this = self.val.datetime(&self.timezone);

        let mm = this.month0() as i64;

        let mut m = helper::MonthCounter::new(this.year(), mm as u32);

        //round up to nearest month
        if this.day0() != 0 || this.hour() != 0 || this.minute() != 0 || this.second() != 0 {
            m.step(1);
        }

        m.round_up_to_nearest_multiple_month(step_value as u32);

        UnixMonths {
            timezone: self.timezone.clone(),
            counter: m,
            step_value: step_value as u32,
        }
    }
}

///
/// Used by [`UnixTime::days()`]
///
pub(crate) struct UnixDayGenerator<T: TimeZone> {
    val: UnixTime,
    timezone: T,
}

impl<T: TimeZone> UnixTimeGenerator for UnixDayGenerator<T> {
    type Iter = UnixDays<T>;
    fn generate(&self, step_value: i64) -> Self::Iter {
        let this = self.val.datetime(&self.timezone);

        let mut dd = this.day0() as i64;

        //round up to nearest day.
        if this.hour() != 0 || this.minute() != 0 || this.second() != 0 {
            dd += 1;
        }

        let dd = round_up_to_nearest_multiple(dd, step_value);

        let base = self
            .timezone
            .ymd(this.year(), this.month(), 1)
            .and_hms(0, 0, 0);

        let d = chrono::Duration::days(dd);
        let base = base + d;

        UnixDays { base, step_value }
    }
}

///
/// Used by [`UnixTime::months()`]
///
pub(crate) struct UnixHourGenerator<T: TimeZone> {
    val: UnixTime,
    timezone: T,
}

impl<T: TimeZone> UnixTimeGenerator for UnixHourGenerator<T> {
    type Iter = UnixHours<T>;
    fn generate(&self, step_value: i64) -> Self::Iter {
        let this = self.val.datetime(&self.timezone);

        let mut hh = this.hour() as i64;

        //round up to nearest hour.
        if this.minute() != 0 || this.second() != 0 {
            hh += 1;
        }

        let hh = round_up_to_nearest_multiple(hh, step_value);

        let base = this.date().and_hms(0, 0, 0);

        let dur = chrono::Duration::hours(hh);

        let base = base + dur;

        UnixHours { base, step_value }
    }
}

///
/// Used by [`UnixTime::minutes()`]
///
pub(crate) struct UnixMinuteGenerator<T: TimeZone> {
    val: UnixTime,
    timezone: T,
}

impl<T: TimeZone> UnixTimeGenerator for UnixMinuteGenerator<T> {
    type Iter = UnixMinutes<T>;
    fn generate(&self, step_value: i64) -> Self::Iter {
        let this = self.val.datetime(&self.timezone);

        let mut mm = this.minute() as i64;

        //round up to nearest minute
        if this.second() != 0 {
            mm += 1;
        }

        let mm = round_up_to_nearest_multiple(mm, step_value);

        let base = this.date().and_hms(this.hour(), 0, 0);

        let dur = chrono::Duration::minutes(mm);
        let base = base + dur;
        //let counter = base.timestamp() + mm * 60;

        UnixMinutes { base, step_value }
    }
}

///
/// Used by [`UnixTime::seconds()`]
///
pub(crate) struct UnixSecondGenerator<T: TimeZone> {
    val: UnixTime,
    timezone: T,
}

impl<T: TimeZone> UnixTimeGenerator for UnixSecondGenerator<T> {
    type Iter = UnixSeconds<T>;
    fn generate(&self, step_value: i64) -> Self::Iter {
        let this = self.val.datetime(&self.timezone);

        let ss = this.second() as i64;

        let ss = round_up_to_nearest_multiple(ss, step_value);

        let base = this.date().and_hms(this.hour(), this.minute(), 0);

        let dur = chrono::Duration::seconds(ss);
        let base = base + dur;
        UnixSeconds { base, step_value }
    }
}

use chrono::TimeZone;
///
/// Used by [`UnixTime::years()`]
///
#[derive(Clone)]
pub(crate) struct UnixYears<T: TimeZone> {
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
pub(crate) struct UnixMonths<T: TimeZone> {
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
pub(crate) struct UnixDays<T: TimeZone> {
    base: chrono::DateTime<T>,
    step_value: i64,
}

impl<T: TimeZone> Iterator for UnixDays<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.base.timestamp();
        let dur = chrono::Duration::days(self.step_value);
        self.base = self.base.clone() + dur;
        Some(UnixTime(r))
    }
}

///
/// Used by [`UnixTime::hours()`]
///
#[derive(Clone)]
pub(crate) struct UnixHours<T: TimeZone> {
    base: chrono::DateTime<T>,
    step_value: i64,
}

impl<T: TimeZone> Iterator for UnixHours<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.base.timestamp();
        let dur = chrono::Duration::hours(self.step_value);
        self.base = self.base.clone() + dur;
        Some(UnixTime(r))
    }
}

///
/// Used by [`UnixTime::minutes()`]
///
#[derive(Clone)]
pub(crate) struct UnixMinutes<T: TimeZone> {
    base: chrono::DateTime<T>,
    step_value: i64,
}
impl<T: TimeZone> Iterator for UnixMinutes<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.base.timestamp();
        let dur = chrono::Duration::minutes(self.step_value);
        self.base = self.base.clone() + dur;

        Some(UnixTime(r))
    }
}

///
/// Used by [`UnixTime::seconds()`]
///
#[derive(Clone)]
pub(crate) struct UnixSeconds<T: TimeZone> {
    base: chrono::DateTime<T>,
    step_value: i64,
}
impl<T: TimeZone> Iterator for UnixSeconds<T> {
    type Item = UnixTime;
    fn next(&mut self) -> Option<Self::Item> {
        let r = self.base.timestamp();
        let dur = chrono::Duration::seconds(self.step_value);
        self.base = self.base.clone() + dur;
        Some(UnixTime(r))
    }
}
