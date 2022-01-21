use clap::{app_from_crate, App, Arg};

#[must_use]
pub fn build() -> App<'static> {
    app_from_crate!()
        .name("Sun sets GTK theme")
        .arg(
            Arg::new("latitude")
                .value_name("NUMBER")
                .long("latitude")
                .takes_value(true)
                .default_value("51.4769N")
                .help("Latitude of the position where the sun position is calculated from")
                .long_help("Latitude of the position where the sun position is calculated from. Is loaded from ~/.config/heliocron.toml when specified"),
        )
        .arg(
            Arg::new("longitude")
                .value_name("NUMBER")
                .long("longitude")
                .takes_value(true)
                .default_value("0.0005W")
                .help("Longitude of the position where the sun position is calculated from")
                .long_help("Longitude of the position where the sun position is calculated from. Is loaded from ~/.config/heliocron.toml when specified"),
        )
        .arg(
            Arg::new("dark-theme")
                .value_name("TEXT")
                .short('d')
                .long("dark-theme")
                .takes_value(true)
                .default_value("Adwaita-dark")
                .help("GTK theme to be set when its dark outside"),
        )
        .arg(
            Arg::new("light-theme")
                .value_name("TEXT")
                .short('l')
                .long("light-theme")
                .takes_value(true)
                .default_value("Adwaita")
                .help("GTK theme to be set when its bright outside"),
        )
}

#[test]
fn verify_app() {
    build().debug_assert();
}
