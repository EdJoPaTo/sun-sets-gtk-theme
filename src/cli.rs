use clap::{Parser, ValueHint};

#[derive(Debug, Parser)]
#[command(about, version)]
pub struct Cli {
    /// Latitude of the position where the sun position is calculated from.
    ///
    /// Is loaded from ~/.config/heliocron.toml when specified
    #[arg(
        long,
        env,
        value_hint = ValueHint::Other,
        default_value = "51.4769N"
    )]
    pub latitude: String,

    /// Longitude of the position where the sun position is calculated from.
    ///
    /// Is loaded from ~/.config/heliocron.toml when specified
    #[arg(
        long,
        env,
        value_hint = ValueHint::Other,
        default_value = "0.0005W",
    )]
    pub longitude: String,

    /// GTK theme to be set when its dark outside
    #[arg(
        short,
        long,
        env,
        value_hint = ValueHint::Other,
        value_name = "THEME",
        default_value = "Adwaita-dark",
    )]
    pub dark_theme: String,

    /// GTK theme to be set when its bright outside
    #[arg(
        short,
        long,
        env,
        value_hint = ValueHint::Other,
        value_name = "THEME",
        default_value = "Adwaita",
    )]
    pub light_theme: String,
}

#[test]
fn verify() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
