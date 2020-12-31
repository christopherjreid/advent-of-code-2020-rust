use crate::days_of_advent::common::io;

pub mod passport;
pub mod passport_deserializer;
pub mod passport_validator;

use passport_deserializer::PassportDeserializer;
use passport_validator::PassportValidator;

pub fn run() -> () {
    let puzzle_input = io::load_input_from_file("day04");
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }
    let puzzle_input = puzzle_input.unwrap();

    let deserializer = passport_deserializer::BatchFilePassportDeserializer {
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

    let strict_validator = passport_validator::StrictPassportValidator {
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

    let passports_w_required_fields = deserialize_passports(&puzzle_input, &deserializer);
    let num_passports_w_required_fields = passports_w_required_fields.len();

    let passports_w_valid_fields = validate_passports(&passports_w_required_fields, &strict_validator);
    let num_passports_w_valid_fields = passports_w_valid_fields.len();

    let content = format!(
        "\
        Found {} simply valid passports\n\
        and {} strictly valid passports",
        num_passports_w_required_fields,
        num_passports_w_valid_fields
    );

    let report = io::format_day_report(
        4,
        "Passport Processing",
        "Count the number of valid passport",
        &content,
    );
    println!("{}", report);
}

fn deserialize_passports(
    batch_file: &str,
    deserializer: &impl PassportDeserializer,
) -> Vec<passport::Passport> {
    let serialized_passports = batch_file.replace('\n', " ");
    let serialized_passports = serialized_passports.split("  ");

    serialized_passports
        .filter_map(|sp| deserializer.deserialize(&sp).ok())
        .collect::<Vec<passport::Passport>>()
}

fn validate_passports<'a>(
    passports: &'a Vec<passport::Passport>,
    validator: &impl PassportValidator,
) -> Vec<&'a passport::Passport> {
    passports
        .iter()
        .filter(|p| {let r = validator.validate(p); r.is_ok() && r.unwrap()})
        .collect::<Vec<&passport::Passport>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_acceptance_test_a() {
        let input = "\
        ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\n\
        byr:1937 iyr:2017 cid:147 hgt:183cm\n\
        \n\
        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\n\
        hcl:#cfa07d byr:1929\n\
        \n\
        hcl:#ae17e1 iyr:2013\n\
        eyr:2024\n\
        ecl:brn pid:760753108 byr:1931\n\
        hgt:179cm\n\
        \n\
        hcl:#cfa07d eyr:2025 pid:166559648\n\
        iyr:2011 ecl:brn hgt:59in\n\
        ";

        let deserialized_passports = deserialize_passports(&input, &create_deserializer());
        
        let num_passports_w_required_fields = deserialized_passports.len();
        assert_eq!(num_passports_w_required_fields, 2);
    }

    #[test]
    fn provided_acceptance_test_b() {
        let input = "\
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n\
        hcl:#623a2f\n\
        \n\
        eyr:2029 ecl:blu cid:129 byr:1989\n\
        iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n\
        \n\
        hcl:#888785\n\
        hgt:164cm byr:2001 iyr:2015 cid:88\n\
        pid:545766238 ecl:hzl\n\
        eyr:2022\n\
        \n\
        iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719\n\
        ";

        let deserialized_passports = deserialize_passports(&input, &create_deserializer());
        let valid_passports = validate_passports(&deserialized_passports, &create_validator());

        assert_eq!(valid_passports.len(), 4);
    }

    fn create_deserializer() -> passport_deserializer::BatchFilePassportDeserializer {
        passport_deserializer::BatchFilePassportDeserializer {
            required_fields: vec![
                "byr".to_string(),
                "iyr".to_string(),
                "eyr".to_string(),
                "hgt".to_string(),
                "hcl".to_string(),
                "ecl".to_string(),
                "pid".to_string(),
            ],
        }
    }

    fn create_validator() -> passport_validator::StrictPassportValidator {
        passport_validator::StrictPassportValidator {
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
        }
    }
}
