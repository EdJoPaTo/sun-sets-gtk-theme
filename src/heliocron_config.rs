use std::fs;

use clap::ArgMatches;
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

pub fn load_coordinates(matches: &ArgMatches) -> Coordinates {
    let (latitude, longitude) =
        HeliocronConfig::load().map_or((None, None), |config| (config.latitude, config.longitude));

    let latitude = latitude.unwrap_or_else(|| matches.value_of("latitude").unwrap().to_owned());
    let longitude = longitude.unwrap_or_else(|| matches.value_of("longitude").unwrap().to_owned());
    Coordinates::from_decimal_degrees(&latitude, &longitude).unwrap()
}
