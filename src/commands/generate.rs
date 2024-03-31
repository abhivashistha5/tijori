use crate::generator::password_generator;

use super::Command;

pub struct PasswordGenerator {}

impl PasswordGenerator {}

impl Command<Option<usize>> for PasswordGenerator {
    fn execute(args: Option<usize>) -> Result<String, String> {
        let password = password_generator::random_password(args);
        Ok(password)
    }
}
