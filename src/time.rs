use std::ops::Add;

use chrono::{Duration, NaiveDateTime, TimeZone, Utc};

#[derive(Clone, Copy)]
pub enum Event {
    BeginOfDay,
    EndOfDay,
}

pub fn next_begin_of_day(datetime: NaiveDateTime, latitude: f64, longitude: f64) -> NaiveDateTime {
    next_event(datetime, Event::BeginOfDay, latitude, longitude)
}

pub fn next_end_of_day(datetime: NaiveDateTime, latitude: f64, longitude: f64) -> NaiveDateTime {
    next_event(datetime, Event::EndOfDay, latitude, longitude)
}

fn next_event(
    datetime: NaiveDateTime,
    event: Event,
    latitude: f64,
    longitude: f64,
) -> NaiveDateTime {
    // Prevent jumping of themes, prevent jumps in the next 5 minutes
    let minimum_date = datetime.add(Duration::minutes(5)).timestamp_millis();
    let mut date = datetime;
    let mut attempt = 1;

    loop {
        let timestamp = suncalc::Timestamp(date.timestamp_millis());
        let sun_times = suncalc::get_times(timestamp, latitude, longitude, None);
        let event_time = match event {
            Event::BeginOfDay => sun_times.sunrise,
            Event::EndOfDay => sun_times.golden_hour,
        }
        .0;

        if event_time > minimum_date {
            return Utc.timestamp_millis_opt(event_time).unwrap().naive_utc();
        }

        assert!(attempt < 500, "did not found next event in 500 iterations");

        attempt += 1;
        date = date.add(chrono::Duration::hours(3));
    }
}
