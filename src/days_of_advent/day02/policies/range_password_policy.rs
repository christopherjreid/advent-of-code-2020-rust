use super::validates_password::ValidatesPassword;

pub struct RangePasswordPolicy {
    pub range: std::ops::RangeInclusive<usize>,
    pub character: char
}

impl ValidatesPassword for RangePasswordPolicy {
    fn is_password_valid(&self, password: &str) -> bool {
        let num_matches = password.matches(self.character).count();
        self.range.contains(&num_matches)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let policy = RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(1, 3),
            character: 'a'
        };
        let password = "a";
        let result = policy.is_password_valid(&password);

        assert_eq!(result, true);
    }

    #[test]
    fn test_1() {
        let policy = RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(1, 3),
            character: 'a'
        };
        let password = "aaa";
        let result = policy.is_password_valid(&password);

        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let policy = RangePasswordPolicy {
            range: std::ops::RangeInclusive::new(1, 3),
            character: 'a'
        };
        let password = "aaaa";
        let result = policy.is_password_valid(&password);

        assert_eq!(result, false);
    }
}