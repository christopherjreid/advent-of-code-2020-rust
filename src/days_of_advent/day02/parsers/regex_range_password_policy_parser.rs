use regex::Regex;

use crate::days_of_advent::day02::policies::range_password_policy::RangePasswordPolicy;
use super::range_password_policy_parser::RangePasswordPolicyParser;

pub struct RegexRangePasswordPolicyParser {
    regex: Regex
}

impl RegexRangePasswordPolicyParser {
    pub fn new() -> Self {
        RegexRangePasswordPolicyParser {
            regex: Regex::new(r"(?P<min>\d+)\-(?P<max>\d+) (?P<character>.): (?P<password>.*)").unwrap()
        }
    }
}

impl RangePasswordPolicyParser for RegexRangePasswordPolicyParser {
    fn parse(&self, string: &str) -> (String, RangePasswordPolicy) {
        let captures = self.regex.captures(&string).unwrap();
        let min = captures.name("min").unwrap().as_str().parse().unwrap();
        let max = captures.name("max").unwrap().as_str().parse().unwrap();

        let policy = RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(min, max),
            character: captures.name("character").unwrap().as_str().parse().unwrap(),
        };
        let password = captures.name("password").unwrap().as_str().to_string();

        (password, policy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let parser = RegexRangePasswordPolicyParser::new();
        let policy = parser.parse("1-3 a: abcde");

        assert_eq!(policy.0, "abcde");
        assert_eq!(policy.1.range, std::ops::RangeInclusive::new(1,3));
        assert_eq!(policy.1.character, 'a');
    }
}