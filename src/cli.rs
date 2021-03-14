use std::fs;

use clap::{App, AppSettings, Arg, ArgMatches};
use heliocron::structs::Coordinates;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HeliocronConfig {
    pub latitude: Option<String>,
    pub longitude: Option<String>,
}

impl HeliocronConfig {
    pub fn load() -> Option<Self> {
        let path = dirs_next::config_dir()?.join("heliocron.toml");
        fs::read_to_string(path)
            .ok()
            .and_then(|content| toml::from_str::<HeliocronConfig>(&content).ok())
    }
}

pub fn build() -> App<'static, 'static> {
    App::new("Sun sets GTK theme")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("latitude")
                .value_name("NUMBER")
                .long("latitude")
                .takes_value(true)
                .default_value("51.4769N")
                .help("Latitude of the position where the sun position is calculated from")
                .long_help("Latitude of the position where the sun position is calculated from. Is loaded from ~/.config/heliocron.toml when specified"),
        )
        .arg(
            Arg::with_name("longitude")
                .value_name("NUMBER")
                .long("longitude")
                .takes_value(true)
                .default_value("0.0005W")
                .help("Longitude of the position where the sun position is calculated from")
                .long_help("Longitude of the position where the sun position is calculated from. Is loaded from ~/.config/heliocron.toml when specified"),
        )
        .arg(
            Arg::with_name("dark-theme")
                .value_name("TEXT")
                .short("d")
                .long("dark-theme")
                .takes_value(true)
                .default_value("Adwaita-dark")
                .help("GTK theme to be set when its dark outside"),
        )
        .arg(
            Arg::with_name("light-theme")
                .value_name("TEXT")
                .short("l")
                .long("light-theme")
                .takes_value(true)
                .default_value("Adwaita")
                .help("GTK theme to be set when its bright outside"),
        )
}

pub fn load_coordinates(matches: &ArgMatches) -> Coordinates {
    let (latitude, longitude) =
        HeliocronConfig::load().map_or((None, None), |config| (config.latitude, config.longitude));

    let latitude = latitude.unwrap_or_else(|| matches.value_of("latitude").unwrap().to_owned());
    let longitude = longitude.unwrap_or_else(|| matches.value_of("longitude").unwrap().to_owned());
    Coordinates::from_decimal_degrees(&latitude, &longitude).unwrap()
}
