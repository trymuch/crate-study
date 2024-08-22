#![allow(unused, deprecated)]
use std::{i64, str::FromStr, time::Duration};

use chrono::{DateTime, FixedOffset, Local, TimeDelta, TimeZone, Utc};

fn main() {
    // duration_study();
    datetime_study();
    fixed_offset_study();
    timezone_study();
}

// 1. 学习使用Duration/TimeDelta
fn duration_study() {
    // 1.创建一个时间段
    // 1.1 new -> Option(TimeDelta)
    let duration =
        TimeDelta::new(10, 100).expect("duration is out of bounds, or nanos >= 1,000,000,000 ");
    println!("duration: {:?}", duration);
    println!("duration: {}", duration);

    // 1.2 weeks,days,hours,minutes,seconds,milliseconds,microseconds,nanoseconds -> TimeDelta
    let weeks = TimeDelta::weeks(1);
    println!("weeks: {}", weeks);
    let days = TimeDelta::days(7);
    println!("days: {}", days);
    let hours = TimeDelta::hours(7 * 24);
    println!("hours: {}", hours);
    let minutes = TimeDelta::minutes(7 * 24 * 60);
    println!("minutes: {}", minutes);
    let seconds = TimeDelta::seconds(7 * 24 * 60 * 60);
    println!("seconds: {}", seconds);
    let milliseconds = TimeDelta::milliseconds(7 * 24 * 60 * 60 * 1000);
    println!("milliseconds: {}", milliseconds);
    let microseconds = TimeDelta::microseconds(7 * 24 * 60 * 60 * 1000 * 1000);
    println!("microseconds: {}", microseconds);
    let nanoseconds = TimeDelta::nanoseconds(7 * 24 * 60 * 60 * 1000 * 1000 * 1000);
    println!("nanoseconds: {}", nanoseconds);

    // 1.3 try_*系列 -> Option<TimeDelta>
    let try_days = TimeDelta::try_days(7).expect("duration is out of bounds");
    println!("try_days:  {}", try_days);
    println!("try_days: {:?}", try_days);

    // 1.4 zero 创建一个零时间段/ is_zero 判断一个时间段是否为零时间段
    let zero = TimeDelta::zero();
    println!("zero: {}", zero);
    println!("is_zero: {}", zero.is_zero());

    // 2. 从时间段中获取各种时间单位的总数 num_*系列方法 -> i64
    let one_week_and_three_days = TimeDelta::weeks(1) + TimeDelta::days(3);
    println!("one_week_and_three_days: {}", one_week_and_three_days);
    let num_weeks = one_week_and_three_days.num_weeks();
    println!("num_weeks: {}", num_weeks);
    let num_days = one_week_and_three_days.num_days();
    println!("num_days: {}", num_days);

    // 3. 和标准库Duration相互转换 to_std/ from_std
    let two_hours_ten_minutes = TimeDelta::hours(2) + TimeDelta::minutes(10);
    let std_duration = two_hours_ten_minutes.to_std().unwrap();
    println!("two_hours_ten_minutes: {}", two_hours_ten_minutes);
    println!("std_duration: {:?}", std_duration);
    let from_duration = Duration::from_secs(3600);
    let chrono_duration = TimeDelta::from_std(from_duration).unwrap();
    println!("from_duration: {:?}", from_duration);
    println!("chrono_duration: {}", chrono_duration);
}

// 2. 学习使用DateTime
fn datetime_study() {
    // 2.1 从时区Timezone对象创建当前的日期时间
    // 2.1.1 获取当前Utc时区的当前日期时间
    let dt_utc = Utc::now();
    println!("dt_utc: {}", dt_utc);

    // 2.1.2 获取当前Local时区的当前日期时间
    let dt_local = Local::now();
    println!("dt_local: {}", dt_local);

    // 2.1.3 获取特定时区的当前日期时间

    let fixed_offset = FixedOffset::east_opt(6 * 3600).unwrap(); // 获取到东六区的时区
    println!("fixed_offset: {}", fixed_offset);
    let dt_fixed_offset = dt_utc.with_timezone(&fixed_offset); // 将当前日期时间转换成该时区的日期时间
    println!("dt_fixed_offset: {}", dt_fixed_offset);
}

fn fixed_offset_study() {
    let tz_east_6 = FixedOffset::east_opt(6 * 3600).expect("Out-of-bounds secs");
    println!("tz_east_6: {}", tz_east_6);

    let tz_west_10 = FixedOffset::west_opt(10 * 3600).expect("Out-of-bouds secs");
    println!("tz_west_6: {}", tz_west_10);

    let tz = FixedOffset::from_str("+0930").unwrap();
    println!("tz: {:?}", tz);

    let tz2 = FixedOffset::from_str("-1000").unwrap();
    println!("tz2: {}", tz2);
}

fn timezone_study() {
    // 学习TimeZone中的方法
    println!("{}", i64::MAX);
    let tz_local = FixedOffset::from_str("+0800").expect("out-of-bouds secs"); // 构建东八区时区

    // 从Unix时间戳中构建日期时间
    let dt_timestamp = tz_local.timestamp(i32::MAX as i64, 0); // deprecated 已经被弃用了
    println!("dt_timestamp: {}", dt_timestamp);
    let dt_timetamp_opt = tz_local.timestamp_opt(3000000, 0).unwrap();
    println!("dt: {}", dt_timetamp_opt);

    let dt_timestamp_nanos = tz_local.timestamp_nanos(i64::MAX); // 永远不会失败，不会out of bounds
    println!("dt_timestamp_nanos: {}", dt_timestamp_nanos);

    let dt_timestamp_micros = tz_local
        .timestamp_micros(100 * 365 * 24 * 60 * 60 * 1000 * 1000)
        .unwrap(); // 1970年1月1日开始大约100年
    println!("dt_timestamp_micros: {:?}", dt_timestamp_micros);

    let dt_timestamp_millis = tz_local.timestamp_millis(100000 * 365 * 24 * 60 * 60 * 1000);
    println!("dt_timestamp_mills: {}", dt_timestamp_millis);

    let dt_timestamp_millis_opt = tz_local.timestamp_millis_opt(100000 * 365 * 24 * 60 * 60 * 1000);
    println!("dt_timestamp_millis_opt: {:?}", dt_timestamp_millis_opt);

    // 从年月日和时分秒等组件和时区构建日期时间
    let dt = tz_local.with_ymd_and_hms(262142, 12, 31, 12, 30, 50);
    println!("dt: {:?}", dt);
}
