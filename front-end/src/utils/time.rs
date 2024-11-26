use chrono::{Duration, NaiveDateTime};

// Get the date from timestamp using client's timezone
pub fn timestamp_to_date(timestamp: i64) -> String {
    // Get the client's timezone offset in minutes
    let date = js_sys::Date::new_0();
    let timezone_offset_minutes = date.get_timezone_offset() as i64;

    // Convert the offset to seconds and create a Duration
    let timezone_offset_seconds = timezone_offset_minutes * 60;
    let offset_duration = Duration::seconds(timezone_offset_seconds);

    // Convert the timestamp to a NaiveDateTime
    let naive_datetime = NaiveDateTime::from_timestamp(timestamp, 0);

    // Adjust the NaiveDateTime by the timezone offset
    let adjusted_datetime = naive_datetime - offset_duration;

    // Format the adjusted datetime
    adjusted_datetime
        .format("%-m/%-d/%Y, %-I:%M:%S %p")
        .to_string()
}
