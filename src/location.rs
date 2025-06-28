fn parse_decimal_degrees(str: &str) -> Option<f64> {
    str.get(..str.len().checked_sub(1)?)?
        .parse()
        .ok()
        .filter(|n: &f64| n.is_sign_positive())
}

fn parse_compass_direction(str: &str) -> Option<char> {
    str.chars().last().map(|char| char.to_ascii_lowercase())
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitude(pub f64);
impl core::str::FromStr for Latitude {
    type Err = &'static str;
    fn from_str(latitude: &str) -> Result<Self, Self::Err> {
        fn inner(latitude: &str) -> Option<f64> {
            let val = parse_decimal_degrees(latitude).filter(|n: &f64| (0.0..=90.0).contains(n))?;
            match parse_compass_direction(latitude)? {
                'n' => Some(val),
                's' => Some(-val),
                _ => None,
            }
        }
        inner(latitude).map(Self).ok_or("Latitude must be a positive value between 0.0 and 90.0 followed by a compass direction ('N' or 'S')")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitude(pub f64);
impl core::str::FromStr for Longitude {
    type Err = &'static str;
    fn from_str(longitude: &str) -> Result<Self, Self::Err> {
        fn inner(longitude: &str) -> Option<f64> {
            let val =
                parse_decimal_degrees(longitude).filter(|n: &f64| (0.0..=180.0).contains(n))?;
            match parse_compass_direction(longitude)? {
                'e' => Some(val),
                'w' => Some(-val),
                _ => None,
            }
        }
        inner(longitude).map(Self).ok_or("Longitude must be a positive value between 0.0 and 180.0 followed by a compass direction ('W' or 'E')")
    }
}

#[test]
fn parse_latitude() {
    let params = [
        (50.0, "50.0N"),
        (-50.0, "50.0S"),
        (-33.9, "33.9S"),
        (18.552, "18.552n"),
        (-26.02, "26.020s"),
        (90.0, "90.0n"),
        (0.0, "0.0n"),
    ];
    for (expected, arg) in params {
        let result = arg.parse::<Latitude>();
        assert_eq!(Ok(Latitude(expected)), result, "{arg}");
    }
}
#[test]
fn parse_longitude() {
    let params = [
        (50.0, "50.0E"),
        (-50.0, "50.0W"),
        (-33.9, "33.9W"),
        (18.552, "18.552e"),
        (-26.02, "26.020w"),
        (180.0, "180.0e"),
        (0.0, "0.0e"),
    ];
    for (expected, arg) in params {
        let result = arg.parse::<Longitude>();
        assert_eq!(Ok(Longitude(expected)), result, "{arg}");
    }
}
