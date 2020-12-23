use crate::days_of_advent::day02::policies::index_password_policy::IndexPasswordPolicy;

pub trait IndexPasswordPolicyParser {
    fn parse(&self, string: &str) -> (String, IndexPasswordPolicy);
}