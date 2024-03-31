use rand::Rng;

pub const PASSWORD_LENGTH: usize = 10;

pub fn random_password(password_length: Option<usize>) -> String {
    // Initialize the characters that are valid for the passwords
    let valid_chars_string =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_-!@#$%&(){};?";
    let mut valid_chars: Vec<String> = Vec::new();
    for chars in valid_chars_string.chars() {
        valid_chars.push(chars.to_string());
    }

    // Randomly select characters
    let mut rng = rand::thread_rng();
    let mut password: Vec<String> = Vec::new();
    let mut random_index: usize;

    let length = match password_length {
        Some(x) => x,
        None => PASSWORD_LENGTH,
    };

    for _ in 0..length {
        random_index = rng.gen_range(0..valid_chars.len());
        password.push(valid_chars[random_index].clone());
    }

    return password.join("");
}
