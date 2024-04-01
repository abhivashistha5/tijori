pub mod commands;
pub mod generator;

/*
    Tests
*/

#[cfg(test)]
mod tests {
    use crate::generator::password_generator::{self, PASSWORD_LENGTH};

    #[test]
    fn test_generate_random_password() {
        let password = password_generator::random_password(None);
        assert_eq!(
            password.len(),
            PASSWORD_LENGTH,
            "Length of password should be: {} but recived {}",
            PASSWORD_LENGTH,
            password.len()
        );
    }

    #[test]
    fn test_generate_random_password_given_length() {
        let pass_length = 5;
        let password = password_generator::random_password(Some(pass_length));
        assert_eq!(
            password.len(),
            pass_length,
            "Length of password should be: {} but recived {}",
            pass_length,
            password.len()
        );
    }
}
