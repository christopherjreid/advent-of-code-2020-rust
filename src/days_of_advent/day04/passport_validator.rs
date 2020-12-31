use super::passport::{Height, HeightUnits, HexColor, Passport};

pub trait PassportValidator {
    fn validate(&self, passport: &Passport) -> Result<bool, String>;
}

pub struct StrictPassportValidator {
    pub birth_year_range: std::ops::RangeInclusive<i32>,
    pub issue_year_range: std::ops::RangeInclusive<i32>,
    pub expiration_year_range: std::ops::RangeInclusive<i32>,
    pub height_range: (std::ops::RangeInclusive<u16>, std::ops::RangeInclusive<u16>),
    pub eye_color: Vec<String>,
    pub passport_id_length: usize,
}

impl StrictPassportValidator {
    pub fn is_birth_year_valid(&self, birth_year: i32) -> bool {
        self.birth_year_range.contains(&birth_year)
    }

    pub fn is_issue_year_valid(&self, issue_year: i32) -> bool {
        self.issue_year_range.contains(&issue_year)
    }

    pub fn is_expiration_year_valid(&self, expiration_year: i32) -> bool {
        self.expiration_year_range.contains(&expiration_year)
    }

    pub fn is_height_valid(&self, height: &Height) -> bool {
        match height.units {
            HeightUnits::CM => self.height_range.0.contains(&height.value),
            HeightUnits::IN => self.height_range.1.contains(&height.value),
        }
    }

    pub fn is_eye_color_valid(&self, eye_color: &str) -> bool {
        const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        VALID_EYE_COLORS.contains(&eye_color)
    }

    pub fn is_passport_id_valid(&self, id: &str) -> bool {
        id.chars().count() == 9 && id.parse::<u32>().is_ok()
    }
}

impl PassportValidator for StrictPassportValidator {
    fn validate(&self, passport: &Passport) -> Result<bool, String> {
        let is_valid = self.is_birth_year_valid(passport.birth_year)
            && self.is_issue_year_valid(passport.issue_year)
            && self.is_expiration_year_valid(passport.expiration_year)
            && self.is_height_valid(&passport.height)
            && self.is_eye_color_valid(&passport.eye_color)
            && self.is_passport_id_valid(&passport.passport_id);

        Ok(is_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strict_validator() {
        let input = 
        Passport {
            birth_year: 1921,
            issue_year: 2015,
            expiration_year: 2025,
            height: Height{ 
                value: 155,
                units: HeightUnits::CM
            },
            hair_color: "#abcdef".parse::<HexColor>().unwrap(),
            eye_color: "amb".to_string(),
            passport_id: "860033327".to_string()
        };

        let validator = StrictPassportValidator {
            birth_year_range: std::ops::RangeInclusive::new(1920, 2002),
            issue_year_range: std::ops::RangeInclusive::new(2010, 2020),
            expiration_year_range: std::ops::RangeInclusive::new(2020, 2030),
            height_range: (
                std::ops::RangeInclusive::new(150, 193),
                std::ops::RangeInclusive::new(59, 76),
            ),
            eye_color: [
                "amb".to_string(),
                "blu".to_string(),
                "brn".to_string(),
                "gry".to_string(),
                "grn".to_string(),
                "hzl".to_string(),
                "oth".to_string(),
            ]
            .to_vec(),
            passport_id_length: 9,
        };

        assert_eq!(validator.validate(&input).unwrap(), true);
    }
}
