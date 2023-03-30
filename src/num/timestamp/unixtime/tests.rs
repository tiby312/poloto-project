use super::*;

#[test]
fn leap_second() {
    let d = NaiveDate::from_ymd_opt(2015, 6, 30)
        .unwrap()
        .and_hms_milli_opt(23, 59, 59, 1_000)
        .unwrap();
    let dt1 = Utc.from_local_datetime(&d).single().unwrap();

    let t1 = dt1.timestamp() + (dt1.timestamp_subsec_millis() as i64) / 1000;
    let dt2 = Utc.with_ymd_and_hms(2015, 7, 1, 0, 0, 0).unwrap();
    let t2 = dt2.timestamp();

    assert_eq!(t1, t2);

    let dt1: UnixTime = Utc
        .with_ymd_and_hms(2015, 6, 30, 23, 59, 59)
        .unwrap()
        .into();
    let dt2: UnixTime = Utc.with_ymd_and_hms(2015, 7, 1, 0, 0, 0).unwrap().into();

    //When a UnixTime is created from a DateTime, it ignores subsecond millis, which is where the leap second
    //information is stored in chrono DateTime.
    assert_ne!(dt1, dt2);
}

#[test]
fn test_leap_day() {
    // 2024 2 29 is a leap day.

    let vt = &chrono::FixedOffset::east_opt(3600 * -5).unwrap();

    let t = UnixMonthGenerator {
        date: vt.with_ymd_and_hms(2023, 12, 1, 0, 0, 0).unwrap(),
    }
    .generate(2)
    .nth(10)
    .unwrap();

    let exp = vt.with_ymd_and_hms(2025, 09, 01, 0, 0, 0).unwrap().into();

    assert_eq!(t, exp);
}

/*
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
