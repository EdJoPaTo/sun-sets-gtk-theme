use clap::{command, Arg, Command, ValueHint};

#[allow(clippy::too_many_lines)]
#[must_use]
pub fn build() -> Command<'static> {
    command!()
        .name("Sun sets GTK theme")
        .arg(
            Arg::new("latitude")
                .long("latitude")
                .env("LATITUDE")
                .value_hint(ValueHint::Other)
                .value_name("NUMBER")
                .takes_value(true)
                .default_value("51.4769N")
                .help("Latitude of the position where the sun position is calculated from")
                .long_help("Latitude of the position where the sun position is calculated from. Is loaded from ~/.config/heliocron.toml when specified"),
        )
        .arg(
            Arg::new("longitude")
                .long("longitude")
                .env("LONGITUDE")
                .value_hint(ValueHint::Other)
                .value_name("NUMBER")
                .takes_value(true)
                .default_value("0.0005W")
                .help("Longitude of the position where the sun position is calculated from")
                .long_help("Longitude of the position where the sun position is calculated from. Is loaded from ~/.config/heliocron.toml when specified"),
        )
        .arg(
            Arg::new("dark-theme")
                .short('d')
                .long("dark-theme")
                .env("DARK_THEME")
                .value_hint(ValueHint::Other)
                .value_name("TEXT")
                .takes_value(true)
                .default_value("Adwaita-dark")
                .help("GTK theme to be set when its dark outside"),
        )
        .arg(
            Arg::new("light-theme")
                .short('l')
                .long("light-theme")
                .env("LIGHT_THEME")
                .value_hint(ValueHint::Other)
                .value_name("TEXT")
                .takes_value(true)
                .default_value("Adwaita")
                .help("GTK theme to be set when its bright outside"),
        )
}

#[test]
fn verify() {
    build().debug_assert();
}
