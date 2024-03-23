use rand::Rng;
use std::io;

const PASSWORD_LENGTH: usize = 10;

fn main() {
    let mut command: String;

    loop {
        command = String::new();
        // read the command
        println!("Enter a command: ");
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        // trim the new line character from command
        command = command.trim().to_string();
        match command.as_str() {
            "generate" => println!("{}", generate_password(PASSWORD_LENGTH)),
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}

fn generate_password(length: usize) -> String {
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
    for _ in 0..length {
        random_index = rng.gen_range(0..valid_chars.len());
        password.push(valid_chars[random_index].clone());
    }

    return password.join("");
}
