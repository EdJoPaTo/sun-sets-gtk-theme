use chrono::{DateTime, Duration, Local, TimeZone};

#[derive(Clone, Copy)]
pub enum Event {
    BeginOfDay,
    EndOfDay,
}

pub fn next_begin_of_day<Tz: TimeZone>(
    datetime: DateTime<Tz>,
    latitude: f64,
    longitude: f64,
) -> DateTime<Local> {
    next_event(datetime, Event::BeginOfDay, latitude, longitude)
}

pub fn next_end_of_day<Tz: TimeZone>(
    datetime: DateTime<Tz>,
    latitude: f64,
    longitude: f64,
) -> DateTime<Local> {
    next_event(datetime, Event::EndOfDay, latitude, longitude)
}

fn next_event<Tz: TimeZone>(
    mut date: DateTime<Tz>,
    event: Event,
    latitude: f64,
    longitude: f64,
) -> DateTime<Local> {
    // Prevent jumping of themes, prevent jumps in the next 5 minutes
    let minimum_date = date.timestamp_millis() + Duration::minutes(5).num_milliseconds();
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
            return Local.timestamp_millis_opt(event_time).unwrap();
        }

        assert!(attempt < 500, "did not found next event in 500 iterations");

        attempt += 1;
        date += chrono::Duration::hours(3);
    }
}
