use std::io;

use tijori::commands::generate::PasswordGenerator;
use tijori::commands::Command;

fn main() {
    let mut command: String = String::new();

    loop {
        command.clear();
        // read the command
        println!("Enter a command: ");
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command");

        // trim the new line character from command
        let res = handle_command(command.trim());
        if let Some(output) = res {
            println!("{}", output);
        } else {
            break;
        }
    }
}

fn handle_command(cmd: &str) -> Option<String> {
    if cmd == "exit" {
        return None;
    }

    let execution_result: Result<String, String> = match cmd {
        "generate" => PasswordGenerator::execute(None),
        _ => Err(String::from("Unknown command")),
    };

    match execution_result {
        Ok(x) => Some(x),
        Err(x) => Some(format!("Error: {}", x)),
    }
}
