use super::validates_password::ValidatesPassword;

pub struct IndexPasswordPolicy {
    pub indices: (usize, usize),
    pub character: char
}

impl ValidatesPassword for IndexPasswordPolicy {
    fn is_password_valid(&self, password: &str) -> bool {
        let index0_char = password.chars().nth(self.indices.0 - 1);
        let index1_char = password.chars().nth(self.indices.1 - 1);

        (index0_char.is_none() && index1_char.is_none()) || 
        (index0_char.is_some() && index0_char.unwrap() == self.character) != 
        (index1_char.is_some() && index1_char.unwrap() == self.character)
    }
}

#[cfg(test)]
mod tests {
    use super::ValidatesPassword;

    #[test]
    fn test_0() {
        let input = super::IndexPasswordPolicy { indices: (1, 2), character: 'a'};
        let password = "ab";
        let result = input.is_password_valid(&password);

        assert_eq!(result, true);
    }

    #[test]
    fn test_1() {
        let input = super::IndexPasswordPolicy { indices: (1, 2), character: 'a'};
        let password = "a";
        let result = input.is_password_valid(&password);

        assert_eq!(result, true);
    }

    #[test]
    fn test_2() {
        let input = super::IndexPasswordPolicy { indices: (1, 2), character: 'a'};
        let password = "b";
        let result = input.is_password_valid(&password);

        assert_eq!(result, false);
    }
}