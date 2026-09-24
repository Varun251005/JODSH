pub enum BuiltinStatus {
    Handled,
    Exit,
    NotHandled,
}

pub fn execute(program: &str, arguments: &[&str]) -> BuiltinStatus {
    match program {
        "exit" => BuiltinStatus::Exit,
        "cd" => {
            if arguments.is_empty() {
                eprintln!("jodsh: cd: missing directory");
            } else {
                let target_dir = arguments[0];
                if let Err(e) = std::env::set_current_dir(target_dir) {
                    eprintln!("jodsh: cd: {}", e);
                }
            }
            BuiltinStatus::Handled
        }
        "pwd" => {
            match std::env::current_dir() {
                Ok(dir) => println!("{}", dir.display()),
                Err(e) => eprintln!("jodsh: pwd: {}", e),
            }
            BuiltinStatus::Handled
        }
        "env" => {
            for (key, value) in std::env::vars() {
                println!("{}={}", key, value);
            }
            BuiltinStatus::Handled
        }
        _ => BuiltinStatus::NotHandled,
    }
}

