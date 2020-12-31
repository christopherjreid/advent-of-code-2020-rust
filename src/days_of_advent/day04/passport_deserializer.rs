use std::collections::HashMap;

use super::passport::{Height, HexColor, Passport};

pub trait PassportDeserializer {
    fn deserialize(&self, passport: &str) -> Result<Passport, String>;
}

pub struct BatchFilePassportDeserializer {
    pub required_fields: Vec<String>,
}

impl PassportDeserializer for BatchFilePassportDeserializer {
    fn deserialize(&self, passport: &str) -> Result<Passport, String> {
        if !self.required_fields.iter().all(|f| passport.contains(f)) {
            return Err("Missing some required fields".to_string());
        }
        let fields: std::collections::HashMap<&str, &str> = passport
            .trim()
            .split(' ')
            .map(|f| BatchFilePassportDeserializer::parse_key_value_from_str(f))
            .into_iter()
            .collect();

        return Ok(Passport {
            birth_year: BatchFilePassportDeserializer::deserialize_value_from_map::<i32>(
                "byr", &fields,
            )?,
            issue_year: BatchFilePassportDeserializer::deserialize_value_from_map::<i32>(
                "iyr", &fields,
            )?,
            expiration_year: BatchFilePassportDeserializer::deserialize_value_from_map::<i32>(
                "eyr", &fields,
            )?,
            height: BatchFilePassportDeserializer::deserialize_value_from_map::<Height>(
                "hgt", &fields,
            )?,
            hair_color: BatchFilePassportDeserializer::deserialize_value_from_map::<HexColor>(
                "hcl", &fields,
            )?,
            eye_color: BatchFilePassportDeserializer::deserialize_value_from_map("ecl", &fields)?,
            passport_id: BatchFilePassportDeserializer::deserialize_value_from_map("pid", &fields)?,
        });
    }
}

impl BatchFilePassportDeserializer {

    fn parse_key_value_from_str(string: &str) -> (&str, &str) {
        let key_value: Vec<&str> = string.split(":").collect();
        (key_value[0].trim(), key_value[1].trim())
    }

    fn deserialize_value_from_map<T: std::str::FromStr>(
        key: &str,
        map: &HashMap<&str, &str>,
    ) -> Result<T, String> {
        let value = map
            .get(&key)
            .ok_or(format!("No '{}' field in HashMap", &key))?
            .parse::<T>()
            .map_err(|_e| format!("Field {} was not the proper type", &key))?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate_deserializer() {
        let _deserializer = BatchFilePassportDeserializer {
            required_fields: vec!["a".to_string(), "b".to_string(), "cde".to_string()],
        };

        assert!(true);
    }

    #[test]
    fn deserialize_valid_passport() {
        let input =
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        let deserializer = BatchFilePassportDeserializer {
            required_fields: vec![
                "byr".to_string(),
                "iyr".to_string(),
                "eyr".to_string(),
                "hgt".to_string(),
                "hcl".to_string(),
                "ecl".to_string(),
                "pid".to_string(),
            ],
        };

        let _passport = deserializer.deserialize(&input).expect("Could not deserialize the passport");
        assert!(true);
    }
}
