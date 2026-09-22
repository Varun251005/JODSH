use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        // 1. Print shell prompt
        print!("jodsh> ");
        io::stdout().flush().unwrap();

        // 2. Read user input
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        // 3. Remove newline
        let input = input.trim();

        // 4. Ignore empty input
        if input.is_empty() {
            continue;
        }

        // 5. Exit the shell
        if input == "exit" {
            break;
        }

        // 6. Split command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();

        let program = parts[0];
        let arguments = &parts[1..];

        // 7. Execute the command
        match Command::new(program)
            .args(arguments)
            .status()
        {
            Ok(status) => {
                println!("Process exited with: {}", status);
            }

            Err(error) => {
                eprintln!("jodsh: {}", error);
            }
        }
    }
}