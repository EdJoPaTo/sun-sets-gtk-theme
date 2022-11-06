#![forbid(unsafe_code)]

use std::ops::Sub;
use std::thread::sleep;

use chrono::{DateTime, FixedOffset};
use clap::Parser;

mod cli;
mod color_scheme;
mod heliocron_config;
mod theme;
mod time;

fn main() {
    let matches = cli::Cli::parse();

    let coordinates = heliocron_config::load_coordinates(matches.latitude, matches.longitude);

    let theme_light = &matches.light_theme;
    let theme_dark = &matches.dark_theme;

    loop {
        let now = chrono::Local::now();
        let next_begin = time::next_begin_of_day(now.into(), &coordinates);
        let next_end = time::next_end_of_day(now.into(), &coordinates);

        if next_end < next_begin {
            println!("its day now");
            set_scheme("default");
            set_theme(theme_light);
            sleep_until(next_end);
        } else {
            println!("its night now");
            set_scheme("prefer-dark");
            set_theme(theme_dark);
            sleep_until(next_begin);
        }
    }
}

fn set_scheme(scheme: &str) {
    let current = color_scheme::current().unwrap();
    if current == scheme {
        println!("color-scheme is already '{scheme}'.");
    } else {
        println!("color-scheme '{scheme}' (was '{current}')");
        color_scheme::set(scheme).unwrap();
    }
}

fn set_theme(theme: &str) {
    let current = theme::current().unwrap();
    if current == theme {
        println!("theme is already '{theme}'.");
    } else {
        println!("set theme '{theme}' (was '{current}')");
        theme::set(theme).unwrap();
    }
}

fn sleep_until(target: DateTime<FixedOffset>) {
    let local_target: DateTime<chrono::Local> = target.into();
    println!("sleep until {local_target}...");
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
