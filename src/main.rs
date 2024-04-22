use std::ops::Sub;
use std::thread::sleep;

use clap::Parser;

mod cli;
mod gsettings;
mod time;

fn main() {
    let matches = cli::Cli::parse();

    let gsettings = gsettings::Gsettings::new();

    loop {
        let now = chrono::Local::now();
        let next_begin = time::next_begin_of_day(now, matches.latitude, matches.longitude);
        let next_end = time::next_end_of_day(now, matches.latitude, matches.longitude);

        let sleep_until = if next_end < next_begin {
            println!("its day now");
            gsettings.set_color_scheme("default");
            next_end
        } else {
            println!("its night now");
            gsettings.set_color_scheme("prefer-dark");
            next_begin
        }
        .naive_local();

        println!("sleep until {sleep_until}...");

        loop {
            // Set theme based on color scheme
            if gsettings.get_color_scheme().contains("dark") {
                gsettings.set_theme(&matches.dark_theme);
            } else {
                gsettings.set_theme(&matches.light_theme);
            }

            // Check current time regularly.
            // When the device gets suspended the sleep also seems to be paused -> wrong times. Checking regularly prevents this
            let now = chrono::Local::now().naive_local();
            let remaining = sleep_until.sub(now).to_std().unwrap_or_default();
            if remaining.as_secs() > 2 {
                sleep(std::time::Duration::from_secs(2));
            } else {
                sleep(remaining);
                break;
            }
        }
    }
}
