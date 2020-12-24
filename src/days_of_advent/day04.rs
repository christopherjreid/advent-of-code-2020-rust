use crate::days_of_advent::common::io;


pub fn run() -> () {
    let puzzle_input = io::load_input_from_file("day04");
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }
    let puzzle_input = puzzle_input.unwrap();

    let num_valid_passports = count_valid_passports(&puzzle_input);
    let content = format!("\
        Found {} valid passports", num_valid_passports);

    let report = io::format_day_report(4, "Passport Processing", "Count the number of valid passport", &content);
    println!("{}", report);
}

fn count_valid_passports(batch_file: &str) -> usize {
    let removed_newlines = batch_file.replace("\n", " ");
    let passports = removed_newlines.split("  ").collect::<Vec<&str>>();

    passports.iter().filter(|p| is_passport_valid(&p)).count()
}

fn is_passport_valid(passport: &str) -> bool {
    const REQUIRED_FIELDS : [&str; 7] = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid"
    ];

    REQUIRED_FIELDS.iter().all(|f| passport.contains(f))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provided_acceptance_test() {
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

    let num_valid_passports = count_valid_passports(&input);
    assert_eq!(num_valid_passports, 2);
    }

    #[test]
    fn is_passport1_valid() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm";
        
        assert_eq!(is_passport_valid(&input), true);
    }

    #[test]
    fn is_passport2_valid() {
        let input = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";
        
        assert_eq!(is_passport_valid(&input), false);
    }
}