fn parse_decimal_degrees(str: &str) -> Option<f64> {
    str[..str.len() - 1]
        .parse()
        .ok()
        .filter(|n: &f64| n.is_sign_positive())
}

fn parse_compass_direction(str: &str) -> Option<char> {
    str.chars().last().map(|o| o.to_ascii_lowercase())
}

pub fn parse_latitude(latitude: &str) -> Option<f64> {
    let val = parse_decimal_degrees(latitude).filter(|n: &f64| (0.0..=90.0).contains(n))?;
    let compass = parse_compass_direction(latitude)?;
    match compass {
        'n' => Some(val),
        's' => Some(val * -1.0),
        _ => None,
    }
}

pub fn parse_longitude(longitude: &str) -> Option<f64> {
    let val = parse_decimal_degrees(longitude).filter(|n: &f64| (0.0..=180.0).contains(n))?;
    let compass = parse_compass_direction(longitude)?;
    match compass {
        'e' => Some(val),
        'w' => Some(val * -1.0),
        _ => None,
    }
}

#[test]
fn test_parse_latitude() {
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
        let result = parse_latitude(arg);
        assert_eq!(Some(expected), result, "{arg}");
    }
}
#[test]
fn test_parse_longitude() {
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
        let result = parse_longitude(arg);
        assert_eq!(Some(expected), result, "{arg}");
    }
}
