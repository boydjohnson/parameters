use nom::bytes::complete::{tag, take_while};
use nom::multi::fold_many0;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use nom::{branch::alt, combinator::opt};
use std::collections::HashMap;


/// Parse query string parameters
/// 
/// ```
///   use parameters::parse_params;
/// 
///   let map = parse_params("?location=minneapolis&category=red");
///   assert_eq!(map.get("location"), Some(&"minneapolis".to_string()));
///   assert_eq!(map.get("category"), Some(&"red".to_string()));
/// ```
/// 
pub fn parse_params(data: &str) -> HashMap<String, String> {
    if let Ok((_, map)) = parse_parameters(data) {
        map
    } else {
        HashMap::new()
    }
}

fn parse_parameters(data: &str) -> IResult<&str, HashMap<String, String>> {
    fold_many0(parse_one, HashMap::new(), |mut m, item| {
        m.insert(item.0.to_string(), item.1.to_string());
        m
    })(data)
}

fn is_not_equals(data: char) -> bool {
    data != '='
}

fn is_not_amp(data: char) -> bool {
    data != '&'
}

fn parse_key(data: &str) -> IResult<&str, &str> {
    preceded(opt(alt((tag("&"), tag("?")))), take_while(is_not_equals))(data)
}

fn parse_value(data: &str) -> IResult<&str, &str> {
    preceded(opt(tag("=")), take_while(is_not_amp))(data)
}

fn parse_one(data: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(parse_key, tag("="), parse_value)(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_success() {
        assert_eq!(parse_key("?key=value"), Ok(("=value", "key")));
    }

    #[test]
    fn test_parse_value_success() {
        assert_eq!(parse_value("=value&key"), Ok(("&key", "value")));
    }

    #[test]
    fn test_parse_one_success() {
        assert_eq!(
            parse_one("?key=value&key2=value2&"),
            Ok(("&key2=value2&", ("key", "value")))
        );
    }

    #[test]
    fn test_parse_params_success() {
        let (_, map) = parse_parameters("?search=mining&gender=female").unwrap();

        assert_eq!(map.get("search"), Some(&"mining".to_string()));

        assert_eq!(map.get("gender"), Some(&"female".to_string()));
    }

    #[test]
    fn test_parse_params_success2() {
        let (_, map) = parse_parameters("?interest=500.15&principal=120000&frequency=monthly").unwrap();

        assert_eq!(map.get("interest"), Some(&"500.15".to_string()));

        assert_eq!(map.get("principal"), Some(&"120000".to_string()));

        assert_eq!(map.get("frequency"), Some(&"monthly".to_string()));
    }
}
