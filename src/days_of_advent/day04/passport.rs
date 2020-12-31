use std::cmp;

#[derive(Debug)]
pub enum HeightUnits {
    CM,
    IN,
}

#[derive(Debug)]
pub struct Height {
    pub value: u16,
    pub units: HeightUnits,
}

impl std::str::FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let height_units = match s.contains("cm") {
            true => HeightUnits::CM,
            false => HeightUnits::IN,
        };

        let height_scalar = s[..&s.len() - 2].parse::<u16>();

        match height_scalar {
            Ok(value) => Ok(Height {
                value: value,
                units: height_units,
            }),
            Err(_) => Err("Could not parse height as u16".to_string()),
        }
    }
}

#[derive(cmp::PartialEq, Debug)]
pub struct HexColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl std::str::FromStr for HexColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if !s.starts_with('#') {
            Err("String does not start with #".to_string())
        } else if s.chars().count() != 7 {
            Err("String has the wrong length".to_string())
        } else {
            let red_result = u8::from_str_radix(&s[1..=2], 16);
            if red_result.is_err() {
                return Err("Could not parse red value from string".to_string());
            }
            let green_result = u8::from_str_radix(&s[1..=2], 16);
            if green_result.is_err() {
                return Err("Could not parse green value from string".to_string());
            }
            let blue_result = u8::from_str_radix(&s[1..=2], 16);
            if blue_result.is_err() {
                return Err("Could not parse blue value from string".to_string());
            }

            Ok(HexColor {
                r: red_result.unwrap(),
                g: green_result.unwrap(),
                b: blue_result.unwrap(),
            })
        }
    }
}

#[derive(Debug)]
pub struct Passport {
    pub birth_year: i32,
    pub issue_year: i32,
    pub expiration_year: i32,
    pub height: Height,
    pub hair_color: HexColor,
    pub eye_color: String,
    pub passport_id: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hexcolor_from_string() {
        use std::str::FromStr;
        let solution = HexColor {
            r: 17,
            g: 17,
            b: 17,
        };
        let result = HexColor::from_str("#111111");
        assert_eq!(result.unwrap(), solution);
    }

    #[test]
    fn parse_bad_hexcolor_from_string_1() {
        use std::str::FromStr;
        let result = HexColor::from_str("111111");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn parse_bad_hexcolor_from_string_2() {
        use std::str::FromStr;
        let result = HexColor::from_str("#G111111");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn parse_bad_hexcolor_from_string_3() {
        use std::str::FromStr;
        let result = HexColor::from_str("#11111111");
        assert_eq!(result.is_err(), true);
    }
}
