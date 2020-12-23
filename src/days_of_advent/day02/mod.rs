pub mod parsers;
pub mod policies;

use crate::days_of_advent::day02::parsers::regex_range_password_policy_parser::RegexRangePasswordPolicyParser;
use crate::days_of_advent::day02::parsers::regex_index_password_policy_parser::RegexIndexPasswordPolicyParser;
use crate::days_of_advent::day02::parsers::range_password_policy_parser::RangePasswordPolicyParser;
use crate::days_of_advent::day02::parsers::index_password_policy_parser::IndexPasswordPolicyParser;
use crate::days_of_advent::day02::policies::range_password_policy::RangePasswordPolicy;
use crate::days_of_advent::day02::policies::index_password_policy::IndexPasswordPolicy;
use crate::days_of_advent::day02::policies::validates_password::ValidatesPassword;

pub fn run() -> () {
    let puzzle_input = load_input_from_file();
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }
    let puzzle_input = puzzle_input.unwrap();

    let regex_range_parser = RegexRangePasswordPolicyParser::new();
    let regex_index_parser = RegexIndexPasswordPolicyParser::new();

    let range_policies: Vec<(String, RangePasswordPolicy)> = puzzle_input
        .lines()
        .map(|line| regex_range_parser.parse(line))
        .collect();
    let index_policies: Vec<(String, IndexPasswordPolicy)> = puzzle_input
        .lines()
        .map(|line| regex_index_parser.parse(line))
        .collect();

    let num_valid_range_passwords = count_valid_passwords(&range_policies).unwrap();
    let num_valid_index_passwords = count_valid_passwords(&index_policies).unwrap();
    println!(
        "\tThe given password file has {} passwords that match the policy",
        num_valid_range_passwords
    );
    println!(
        "\tThe given password file has {} passwords that match the policy",
        num_valid_index_passwords
    );
}

fn count_valid_passwords(
    passwords_with_policies: &[(String, impl ValidatesPassword)],
) -> Result<u32, &str> {
    let mut valid_passwords: u32 = 0;
    for entry in passwords_with_policies {
        if entry.1.is_password_valid(&entry.0.to_string()) {
            valid_passwords += 1;
        }
    }
    Ok(valid_passwords)
}

fn load_input_from_file() -> std::io::Result<String> {
    let cargo_path = env!("CARGO_MANIFEST_DIR");
    let input_file_path = format!("{}/share/days_of_advent/day02/input", cargo_path);

    std::fs::read_to_string(&input_file_path)
}

#[cfg(test)]
mod tests {

    #[test]
    fn provided_acceptance_test() {
        use super::RangePasswordPolicyParser;
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

        let solution = 2;

        let parser = super::RegexRangePasswordPolicyParser::new();
        let policies: Vec<(String, super::RangePasswordPolicy)> =
            input.lines().map(|line| parser.parse(line)).collect();
        let num_valid_passwords = super::count_valid_passwords(&policies).unwrap();
        assert_eq!(num_valid_passwords, solution);
    }

    #[test]
    fn check_password_1() {
        use super::ValidatesPassword;
        let policy = super::RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(1, 3),
            character: 'a',
        };
        let password = "abcde";
        let result = policy.is_password_valid(&password);
        assert_eq!(result, true);
    }

    #[test]
    fn check_password_2() {
        use super::ValidatesPassword;
        let policy = super::RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(1, 3),
            character: 'b',
        };
        let password = "cdefg";
        let result = policy.is_password_valid(&password);
        assert_eq!(result, false);
    }

    #[test]
    fn check_password_3() {
        use super::ValidatesPassword;
        let policy = super::RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(2, 9),
            character: 'c',
        };
        let password = "ccccccccc";
        let result = policy.is_password_valid(&password);
        assert_eq!(result, true);
    }
}
