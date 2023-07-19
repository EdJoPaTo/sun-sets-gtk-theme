#![forbid(unsafe_code)]

use std::ops::Sub;
use std::thread::sleep;

use chrono::{DateTime, TimeZone};
use clap::Parser;

mod cli;
mod gsettings;
mod location;
mod time;

fn main() {
    let matches = cli::Cli::parse();

    let latitude = location::parse_latitude(&matches.latitude).expect("Latitude must be a positive value between 0.0 and 90.0 followed by a compass direction ('N' or 'S')");
    let longitude = location::parse_longitude(&matches.longitude).expect("Longitude must be a positive value between 0.0 and 180.0 followed by a compass direction ('W' or 'E')");

    let gsettings = gsettings::Gsettings::new();

    loop {
        let now = chrono::Local::now();
        let next_begin = time::next_begin_of_day(now, latitude, longitude);
        let next_end = time::next_end_of_day(now, latitude, longitude);

        if next_end < next_begin {
            println!("its day now");
            gsettings.set_color_scheme("default");
            gsettings.set_theme(&matches.light_theme);
            sleep_until(&next_end);
        } else {
            println!("its night now");
            gsettings.set_color_scheme("prefer-dark");
            gsettings.set_theme(&matches.dark_theme);
            sleep_until(&next_begin);
        }
    }
}

fn sleep_until<Tz: TimeZone>(target: &DateTime<Tz>) {
    let target = target.naive_local();
    println!("sleep until {target}...");
    loop {
        // Check current time regularly.
        // When the device gets suspended the sleep also seems to be paused -> wrong times. Checking regularly prevents this
        let now = chrono::Local::now().naive_local();
        let remaining = target.sub(now).to_std().unwrap_or_default();
        if remaining.as_secs() > 5 {
            sleep(std::time::Duration::from_secs(5));
        } else {
            sleep(remaining);
            break;
        }
    }
}
