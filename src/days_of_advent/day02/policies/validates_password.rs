pub trait ValidatesPassword {
    fn is_password_valid(&self, password: &str) -> bool;
}