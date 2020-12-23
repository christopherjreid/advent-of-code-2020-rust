use super::super::policies::range_password_policy::RangePasswordPolicy;

pub trait RangePasswordPolicyParser {
    fn parse(&self, string: &str) -> (String, RangePasswordPolicy);
}