mod builtins;

use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        // 1. Print shell prompt
        print!("jodsh> ");
        io::stdout().flush().unwrap();

        // 2. Read user input
        let mut input = String::new();
        let mut raw_input = String::new();
        io::stdin().read_line(&mut raw_input).unwrap();

        io::stdin()
            .read_line(&mut input)
            .unwrap();

        // 3. Remove newline
        let input = input.trim();

        // 4. Ignore empty input
        if input.is_empty() {
        let raw_input = raw_input.trim();
        if raw_input.is_empty() {
            continue;
        }

        // 5. Exit the shell
        if input == "exit" {
            break;
        // 3. Variable expansion
        let mut expanded_input = String::new();
        let mut chars = raw_input.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '$' {
                let mut var_name = String::new();
                while let Some(&next_c) = chars.peek() {
                    // Collect valid variable name characters
                    if next_c.is_alphanumeric() || next_c == '_' {
                        var_name.push(next_c);
                        chars.next(); // Consume the character
                    } else {
                        break;
                    }
                }
                
                if !var_name.is_empty() {
                    // Expand the variable if it exists; otherwise replace with empty string
                    if let Ok(val) = std::env::var(&var_name) {
                        expanded_input.push_str(&val);
                    }
                } else {
                    // Just a solitary '$'
                    expanded_input.push('$');
                }
            } else {
                expanded_input.push(c);
            }
        }

        // 6. Split command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();

        // 4. Split command and arguments from the expanded input
        let parts: Vec<&str> = expanded_input.split_whitespace().collect();
        if parts.is_empty() {
            continue; // Could be empty after variable expansion
        }
        
        let program = parts[0];
        let arguments = &parts[1..];

        // Built-in command: cd
        if program == "cd" {
            if arguments.is_empty() {
                eprintln!("jodsh: cd: missing directory");
            } else {
                let target_dir = arguments[0];
                if let Err(e) = std::env::set_current_dir(target_dir) {
                    eprintln!("jodsh: cd: {}", e);
                }
            }
            continue;
        // 5. Handle built-in commands
        match builtins::execute(program, arguments) {
            builtins::BuiltinStatus::Handled => continue,
            builtins::BuiltinStatus::Exit => break,
            builtins::BuiltinStatus::NotHandled => {} // Fall through to external execution
        }

        // Built-in command: pwd
        if program == "pwd" {
            match std::env::current_dir() {
                Ok(dir) => println!("{}", dir.display()),
                Err(e) => eprintln!("jodsh: pwd: {}", e),
            }
            continue;
        }

        // 7. Execute the command
        // 6. Execute external command
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