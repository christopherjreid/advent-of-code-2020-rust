use regex::Regex;

pub fn run() -> () {
    let puzzle_input = load_input_from_file();
    if puzzle_input.is_err() {
        panic!("Could not load entries from file");
    }

    let regex_parser = RegexPasswordWithPolicyParser::new();

    let policies : Vec<PasswordWithPolicy> = puzzle_input.unwrap().lines().map(|line| regex_parser.parse(line)).collect();

    let num_valid_passwords = count_valid_passwords(&policies).unwrap();
    println!("\tThe given password file has {} passwords that match the policy", num_valid_passwords);
}

struct RegexPasswordWithPolicyParser {
    regex: Regex
}

trait PasswordWithPolicyParser {
    fn parse(&self, string: &str) -> PasswordWithPolicy;
}

impl RegexPasswordWithPolicyParser {
    fn new() -> RegexPasswordWithPolicyParser {
        RegexPasswordWithPolicyParser {
            regex: Regex::new(r"(?P<min>\d+)\-(?P<max>\d+) (?P<character>.): (?P<password>.*)").unwrap()
        }
    }
}

impl PasswordWithPolicyParser for RegexPasswordWithPolicyParser {
    fn parse(&self, string: &str) -> PasswordWithPolicy {
        let captures = self.regex.captures(&string).unwrap();
        let min = captures.name("min").unwrap().as_str().parse().unwrap();
        let max = captures.name("max").unwrap().as_str().parse().unwrap();

        PasswordWithPolicy {
            range: std::ops::RangeInclusive::new(min, max),
            character: captures.name("character").unwrap().as_str().parse().unwrap(),
            password: captures.name("password").unwrap().as_str().to_string()
        }
    }
}

struct PasswordWithPolicy {
    range: std::ops::RangeInclusive<usize>,
    character: char,
    password: String
}

fn count_valid_passwords(passwords_with_policies: &[PasswordWithPolicy]) -> Result<u32, &str> {
    let mut valid_passwords: u32 = 0;
    for policy in passwords_with_policies {
        if is_password_valid(&policy) {
            valid_passwords +=1;
        }
    }
    Ok(valid_passwords)
}

fn is_password_valid(policy: &PasswordWithPolicy) -> bool {
    let num_matches = policy.password.matches(policy.character).count();
    policy.range.contains(&num_matches)
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
        use super::PasswordWithPolicyParser;

        let input =
            "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

        let solution = 2;

        let parser = super::RegexPasswordWithPolicyParser::new();
        let policies : Vec<super::PasswordWithPolicy> = input.lines().map(|line| parser.parse(line)).collect();
        let num_valid_passwords = super::count_valid_passwords(&policies).unwrap();
        assert_eq!(num_valid_passwords, solution);
    }

    #[test]
    fn check_password_1() {
        let policy = super::PasswordWithPolicy { 
            range: std::ops::RangeInclusive::new(1, 3),
            character: 'a',
            password: "abcde".to_string()
        };
        let result = super::is_password_valid(&policy);
        assert_eq!(result, true);
    }

    #[test]
    fn check_password_2() {
        let policy = super::PasswordWithPolicy { 
            range: std::ops::RangeInclusive::new(1, 3), 
            character: 'b', 
            password: "cdefg".to_string()
        };
        let result = super::is_password_valid(&policy);
        assert_eq!(result, false);
    }

    #[test]
    fn check_password_3() {
        let policy = super::PasswordWithPolicy { 
            range: std::ops::RangeInclusive::new(2, 9), 
            character: 'c', 
            password: "ccccccccc".to_string()
        };
        let result = super::is_password_valid(&policy);
        assert_eq!(result, true);
    }

}