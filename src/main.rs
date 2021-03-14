#![forbid(unsafe_code)]

use std::ops::Sub;
use std::thread::sleep;

use chrono::{DateTime, FixedOffset};

mod cli;
mod theme;
mod time;

fn main() {
    let matches = cli::build().get_matches();

    let coordinates = cli::load_coordinates(&matches);

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
        println!("theme is already '{}'.", theme)
    } else {
        println!("set theme '{}' (was '{}')", theme, current);
        theme::set(theme).unwrap();
    }
}

fn sleep_until(target: DateTime<FixedOffset>) {
    let local_target: DateTime<chrono::Local> = target.into();
    println!("sleep until {}...", local_target);

    let now: DateTime<FixedOffset> = chrono::Local::now().into();
    if let Ok(duration) = target.sub(now).to_std() {
        sleep(duration);
    }
}
