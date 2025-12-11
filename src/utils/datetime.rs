use chrono::{DateTime, FixedOffset, TimeZone};

pub fn get_datetime_from_unix_timestamp(
    unix_timestamp: i64,
    offset_in_hours: Option<i32>,
    offset_in_minutes: Option<i32>,
) -> DateTime<FixedOffset> {
    let mut offset_in_seconds = 0;
    if let Some(offset_in_hours) = offset_in_hours {
        offset_in_seconds += offset_in_hours * 3600;
    }
    if let Some(offset_in_minutes) = offset_in_minutes {
        offset_in_seconds += offset_in_minutes * 60;
    }
    let offset = FixedOffset::east_opt(offset_in_seconds).unwrap();
    offset.timestamp_opt(unix_timestamp, 0).unwrap()
}

pub fn parse_offset(offset: &str) -> (i32, i32) {
    assert!(offset.len() == 5, "Offset must be in +HHMM or -HHMM format");

    let sign = if &offset[0..1] == "+" { 1 } else { -1 };
    let hours: i32 = offset[1..3].parse().expect("Invalid hours in offset");
    let minutes: i32 = offset[3..5].parse().expect("Invalid minutes in offset");
    (sign * hours, sign * minutes)
}
