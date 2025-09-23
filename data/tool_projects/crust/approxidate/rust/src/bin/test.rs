use approxidate::approxidate;
use std::time::UNIX_EPOCH;
use std::time::{Duration, SystemTime};
use chrono::Utc;
use chrono::TimeZone;
 fn _start_of_day(sec: approxidate::TimeT) -> approxidate::TimeT{
    sec - (sec % 86400)
}
pub fn monotonic_now() -> f64 {
    let now = SystemTime::now();
    let duration = now.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
    duration.as_secs() as f64 + (duration.subsec_nanos() as f64 / 1_000_000_000.0)
}
#[test]
 fn approxidate_test(){
    let mut errors = 0;
    macro_rules! assert_equal {
        ($a:expr, $b:expr) => {
            if $a != $b {
                eprintln!("Test failure: {} != {}", $a, $b);
                errors += 1;
            }
        };
    }
    let mut tv = approxidate::TimeVal { tv_sec: 0, tv_usec: 0 };
    // Placeholder function calls for approxidate
    approxidate::approxidate_main("10/Mar/2013:00:00:02.003 UTC", &mut tv);
    assert_eq!(tv.tv_sec, 1362873602);
    assert_eq!(tv.tv_usec, 3000);
    approxidate::approxidate_main("10/Mar/2013:00:00:02 UTC", &mut tv);
    assert_eq!(tv.tv_sec, 1362873602);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("10/Mar/2013:00:00:07 UTC", &mut tv);
    assert_eq!(tv.tv_sec, 1362873607);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("10/Mar/2012:00:00:07 UTC", &mut tv);
    assert_eq!(tv.tv_sec, 1331337607);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("10/Mar/2012:00:00:07 +0500", &mut tv);
    assert_eq!(tv.tv_sec, 1331319607);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("10/Mar/2012:00:00:07.657891 +0500", &mut tv);
    assert_eq!(tv.tv_sec, 1331319607);
    assert_eq!(tv.tv_usec, 657891);
    approxidate::approxidate_main("10/Mar/2012:00:00:07.657891 +1400", &mut tv);
    assert_eq!(tv.tv_sec, 1331287207);
    assert_eq!(tv.tv_usec, 657891);
    approxidate::approxidate_main("10/Mar/2012:00:00:07.657891 -0110", &mut tv);
    assert_eq!(tv.tv_sec, 1331341807);
    assert_eq!(tv.tv_usec, 657891);
    approxidate::approxidate_main("mar 10 2013 00:00:07 UTC", &mut tv);
    assert_eq!(tv.tv_sec, 1362873607);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("mar 10 2013 04:00:07 -0500", &mut tv);
    assert_eq!(tv.tv_sec, 1362906007);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("march 10 2013 04:00:07 -0500", &mut tv);
    assert_eq!(tv.tv_sec, 1362906007);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("march 10 2013 04:00:07 -0500", &mut tv);
    assert_eq!(tv.tv_sec, 1362906007);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("10 march 2013 04:00:07 -0500", &mut tv);
    assert_eq!(tv.tv_sec, 1362906007);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("2013 10 march 04:00:07 -0500", &mut tv);
    assert_eq!(tv.tv_sec, 1362906007);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("2013 march 10 04:00:07 -0500", &mut tv);
    assert_eq!(tv.tv_sec, 1362906007);
    assert_eq!(tv.tv_usec, 0);
    approxidate::approxidate_main("00:00:07.657891", &mut tv);
    assert_eq!(tv.tv_usec, 657891);
    approxidate::approxidate_main("23:11:07.9876 +1400", &mut tv);
    assert_eq!(tv.tv_usec, 987600);
    approxidate::approxidate_main("23:11:07.9876", &mut tv);
    assert_eq!(tv.tv_usec, 987600);
    approxidate::approxidate_main("1/1/2014", &mut tv);
    assert_eq!(_start_of_day(tv.tv_sec), 1388534400);
    approxidate::approxidate_main("1/1/2014 UTC", &mut tv);
    assert_eq!(_start_of_day(tv.tv_sec), 1388534400);
    let mut tv_rel = approxidate::TimeVal { tv_sec: 1577910617, tv_usec: 0 }; // 2020-01-01 12:30:17 -08:00
    approxidate::approxidate_relative("1/1/2014", &mut tv, &mut tv_rel);
    assert_eq!(tv.tv_sec, 1388608217);
    let mut ts = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs();
    ts += 86400 * 31 * 5;
    let tm = Utc.timestamp(ts as i64, 0);
    let buff = tm.format("%m/%d/%Y").to_string();
    approxidate::approxidate_main(&buff, &mut tv);
    assert_eq!(_start_of_day(tv.tv_sec), _start_of_day(ts as i64));
    let tv_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let usec = tv_now.subsec_micros();
    approxidate::approxidate_main("10/Mar/2012", &mut tv);
    if !((usec as i64 - 10000) < tv.tv_usec && (usec as i64 + 10000) > tv.tv_usec) {
        eprintln!("Error: usec calculation for anonymous time is off");
        errors += 1;
    }
    if errors > 0 {
        eprintln!("{} tests failed", errors);
    } else {
        println!("All tests passed");
    }
}
#[test]
 fn approxidate_benchmark(){
    let rounds = 1000000;
    let start = monotonic_now();
    let mut tv = approxidate::TimeVal { tv_sec: 0, tv_usec: 0 };
    for _ in 0..rounds {
        assert!(approxidate::approxidate_main("2019-01-01 00:00:00", &mut tv) >= 0);
    }
}
fn main(){}