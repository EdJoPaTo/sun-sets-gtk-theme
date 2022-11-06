use std::fs;

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

pub fn load_coordinates(cli_latitude: String, cli_longitude: String) -> Coordinates {
    let (latitude, longitude) =
        HeliocronConfig::load().map_or((None, None), |config| (config.latitude, config.longitude));

    let latitude = latitude.unwrap_or(cli_latitude);
    let longitude = longitude.unwrap_or(cli_longitude);
    Coordinates::from_decimal_degrees(&latitude, &longitude).unwrap()
}
