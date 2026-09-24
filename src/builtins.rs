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
        "export" => {
            if arguments.is_empty() {
                eprintln!("jodsh: export: missing argument");
            } else {
                let arg = arguments[0];
                if let Some(pos) = arg.find('=') {
                    let key = &arg[..pos];
                    let value = &arg[pos + 1..];
                    unsafe { std::env::set_var(key, value) };
                } else {
                    // Fallback if no '=' is provided
                    unsafe { std::env::set_var(arg, "") };
                }
            }
            BuiltinStatus::Handled
        }
        "unset" => {
            if arguments.is_empty() {
                eprintln!("jodsh: unset: missing variable name");
            } else {
                unsafe { std::env::remove_var(arguments[0]) };
            }
            BuiltinStatus::Handled
        }
        _ => BuiltinStatus::NotHandled,
    }
}
