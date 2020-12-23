use regex::Regex;

use crate::days_of_advent::day02::policies::index_password_policy::IndexPasswordPolicy;
use super::index_password_policy_parser::IndexPasswordPolicyParser;

pub struct RegexIndexPasswordPolicyParser {
    regex: Regex
}

impl RegexIndexPasswordPolicyParser {
    pub fn new() -> Self {
        RegexIndexPasswordPolicyParser {
            regex: Regex::new(r"(?P<index1>\d+)\-(?P<index2>\d+) (?P<character>.): (?P<password>.*)").unwrap()
        }
    }
}

impl IndexPasswordPolicyParser for RegexIndexPasswordPolicyParser {
    fn parse(&self, string: &str) -> (String, IndexPasswordPolicy) {
        let captures = self.regex.captures(&string).unwrap();
        let index1 = captures.name("index1").unwrap().as_str().parse().unwrap();
        let index2 = captures.name("index2").unwrap().as_str().parse().unwrap();

        let policy = IndexPasswordPolicy {
            indices: (index1, index2),
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
        let parser = RegexIndexPasswordPolicyParser::new();
        let policy = parser.parse("1-3 a: abcde");

        assert_eq!(policy.0, "abcde");
        assert_eq!(policy.1.indices, (1,3));
        assert_eq!(policy.1.character, 'a');
    }
}