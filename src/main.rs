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

/*
    tests
*/

#[cfg(test)]
mod main_test {
    use crate::handle_command;

    #[test]
    fn test_handle_command_generate() {
        let execution_result = handle_command("generate");
        assert!(execution_result.is_some());
        if let Some(result_val) = execution_result {
            assert!(!result_val.starts_with("Error"));
        }
    }

    #[test]
    fn test_handle_command_exit() {
        let execution_result = handle_command("exit");
        assert!(execution_result.is_none())
    }

    #[test]
    fn test_unknown_command() {
        let execution_result = handle_command("test_command");
        assert!(execution_result.is_some());
        if let Some(result_val) = execution_result {
            assert!(result_val.starts_with("Error"));
        }
    }
}
