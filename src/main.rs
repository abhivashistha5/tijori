use rand::Rng;

fn main() {
    // Initialize the characters that are valid for the passwords
    let valid_chars_string =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890_-!@#$%&(){};?";
    let mut valid_chars: Vec<String> = Vec::new();
    for chars in valid_chars_string.chars() {
        valid_chars.push(chars.to_string());
    }

    const PASSWORD_LENGTH: usize = 10;

    // Randomly select characters
    let mut rng = rand::thread_rng();
    let mut password: Vec<String> = Vec::new();
    let mut random_index: usize;
    for _ in 0..PASSWORD_LENGTH {
        random_index = rng.gen_range(0..valid_chars.len());
        password.push(valid_chars[random_index].clone());
    }

    println!("{}", password.join(""));
}
