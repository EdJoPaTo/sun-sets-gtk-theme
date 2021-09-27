#![forbid(unsafe_code)]

use std::ops::Sub;
use std::thread::sleep;

use chrono::{DateTime, FixedOffset};

mod cli;
mod heliocron_config;
mod theme;
mod time;

fn main() {
    let matches = cli::build().get_matches();

    let coordinates = heliocron_config::load_coordinates(&matches);

    let theme_light = matches.value_of("light-theme").unwrap();
    let theme_dark = matches.value_of("dark-theme").unwrap();

    loop {
        let now = chrono::Local::now();
        let next_begin = time::next_begin_of_day(now.into(), &coordinates);
        let next_end = time::next_end_of_day(now.into(), &coordinates);

        if next_end < next_begin {
            println!("its day now");
            set_theme(theme_light);
            sleep_until(next_end);
        } else {
            println!("its night now");
            set_theme(theme_dark);
            sleep_until(next_begin);
        }
    }
}

fn set_theme(theme: &str) {
    let current = theme::current().unwrap();
    if current == theme {
        println!("theme is already '{}'.", theme);
    } else {
        println!("set theme '{}' (was '{}')", theme, current);
        theme::set(theme).unwrap();
    }
}

fn sleep_until(target: DateTime<FixedOffset>) {
    let local_target: DateTime<chrono::Local> = target.into();
    println!("sleep until {}...", local_target);
    loop {
        // Check current time regularly.
        // When the device gets suspended the sleep also seems to be paused -> wrong times. Checking regularly prevents this
        let now: DateTime<FixedOffset> = chrono::Local::now().into();
        let remaining = target.sub(now).to_std().unwrap_or_default();
        if remaining.as_secs() > 5 {
            sleep(std::time::Duration::from_secs(5));
        } else {
            sleep(remaining);
            break;
        }
    }
}
