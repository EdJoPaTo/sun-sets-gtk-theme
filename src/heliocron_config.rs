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
            .and_then(|content| toml::from_str(&content).ok())
    }
}

pub fn load_coordinates(matches: &ArgMatches) -> Coordinates {
    let (latitude, longitude) =
        HeliocronConfig::load().map_or((None, None), |config| (config.latitude, config.longitude));

    let latitude =
        latitude.unwrap_or_else(|| matches.get_one::<String>("latitude").unwrap().clone());
    let longitude =
        longitude.unwrap_or_else(|| matches.get_one::<String>("longitude").unwrap().clone());
    Coordinates::from_decimal_degrees(&latitude, &longitude).unwrap()
}
