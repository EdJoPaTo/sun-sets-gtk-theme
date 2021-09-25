use std::ops::Add;

use chrono::{DateTime, Duration, FixedOffset};
use heliocron::calc::SolarCalculations;
use heliocron::enums::{Event, TimeOfDay};
use heliocron::structs::Coordinates;

const DAY_BEGIN: Event = Event::Sunrise {
    degrees_below_horizon: 3.0,
    time_of_day: TimeOfDay::AM,
};

const DAY_END: Event = Event::Sunset {
    degrees_below_horizon: -3.0,
    time_of_day: TimeOfDay::PM,
};

pub fn next_begin_of_day(
    datetime: DateTime<FixedOffset>,
    coordinates: &Coordinates,
) -> DateTime<FixedOffset> {
    next_event(datetime, &DAY_BEGIN, coordinates)
}

pub fn next_end_of_day(
    datetime: DateTime<FixedOffset>,
    coordinates: &Coordinates,
) -> DateTime<FixedOffset> {
    next_event(datetime, &DAY_END, coordinates)
}

fn next_event(
    datetime: DateTime<FixedOffset>,
    event: &Event,
    coordinates: &Coordinates,
) -> DateTime<FixedOffset> {
    // Prevent jumping of themes, prevent jumps in the next 5 minutes
    let minimum_date = datetime.add(Duration::minutes(5));
    let mut date = datetime;
    let mut attempt = 1;

    loop {
        let sol = SolarCalculations::new(date, *coordinates);

        if let Some(event_time) = sol.calculate_event_time(event.clone()).datetime {
            if event_time > minimum_date {
                return event_time;
            }
        }

        assert!(attempt < 500, "did not found next event in 500 iterations");

        attempt += 1;
        date = date.add(chrono::Duration::hours(3));
    }
}
