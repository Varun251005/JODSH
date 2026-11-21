mod builtins;
mod lexer;
mod parser;

use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};
use std::process::Command;

enum ExecResult {
    Success,
    Failure,
    Exit,
}

fn execute_pipeline(pipeline: parser::Pipeline) -> Result<Vec<Child>, ExecResult> {
    let commands = pipeline.commands;
    if commands.is_empty() {
        return Err(ExecResult::Success);
    }

    if commands.len() == 1 {
        let cmd = &commands[0];
        let program = &cmd.program;
        let arguments: Vec<&str> = cmd.arguments.iter().map(|s| s.as_str()).collect();

        match builtins::execute(program, &arguments) {
            builtins::BuiltinStatus::Handled => return Err(ExecResult::Success), 
            builtins::BuiltinStatus::Exit => return Err(ExecResult::Exit),
            builtins::BuiltinStatus::NotHandled => {} 
        }
    } else {
        let mut has_builtin = false;
        for cmd in &commands {
            match cmd.program.as_str() {
                "cd" | "pwd" | "exit" | "env" | "export" | "unset" => {
                    has_builtin = true;
                    break;
                }
                _ => {}
            }
        }
        if has_builtin {
            eprintln!("jodsh: built-in commands are not currently supported in pipelines");
            return Err(ExecResult::Failure);
        }
    }

    let mut children = Vec::new();
    let mut previous_stdout = None;
    let num_commands = commands.len();

    for (i, cmd) in commands.iter().enumerate() {
        let mut command = Command::new(&cmd.program);
        command.args(&cmd.arguments);

        if let Some(ref stdin_file) = cmd.stdin {
            match File::open(stdin_file) {
                Ok(file) => { command.stdin(file); },
                Err(e) => {
                    eprintln!("jodsh: {}: {}", stdin_file, e);
                    return Err(ExecResult::Failure);
                }
            }
        } else if let Some(stdout) = previous_stdout {
            command.stdin(stdout);
        }

        let mut explicit_stdout = false;
        if let Some(ref stdout_redir) = cmd.stdout {
            let file_res = match stdout_redir {
                parser::OutputRedirect::Truncate(path) => File::create(path),
                parser::OutputRedirect::Append(path) => OpenOptions::new().create(true).append(true).open(path),
            };
            match file_res {
                Ok(file) => { 
                    command.stdout(file); 
                    explicit_stdout = true;
                },
                Err(e) => {
                    eprintln!("jodsh: {}", e);
                    return Err(ExecResult::Failure);
                }
            }
            previous_stdout = None; 
        } else if i < num_commands - 1 {
            command.stdout(Stdio::piped());
        }

        if let Some(ref stderr_file) = cmd.stderr {
            match File::create(stderr_file) {
                Ok(file) => { command.stderr(file); },
                Err(e) => {
                    eprintln!("jodsh: {}: {}", stderr_file, e);
                    return Err(ExecResult::Failure);
                }
            }
        }

        match command.spawn() {
            Ok(mut child) => {
                if !explicit_stdout && i < num_commands - 1 {
                    previous_stdout = child.stdout.take().map(Stdio::from);
                } else {
                    previous_stdout = None;
                }
                children.push(child);
            }
            Err(e) => {
                eprintln!("jodsh: {}: {}", cmd.program, e);
                return Err(ExecResult::Failure);
            }
        }
    }

    Ok(children)
}

fn main() {
    let mut background_jobs: Vec<Child> = Vec::new();

    loop {
        // Clean up finished background jobs to avoid zombie processes
        background_jobs.retain_mut(|child| {
            match child.try_wait() {
                Ok(Some(status)) => {
                    println!("[Job {} finished with {}]", child.id(), status);
                    false
                }
                Ok(None) => true,
                Err(_) => false,
            }
        });

        // 1. Print shell prompt
        print!("jodsh> ");
        io::stdout().flush().unwrap();

        // 2. Read user input
        let mut raw_input = String::new();
        io::stdin().read_line(&mut raw_input).unwrap();

        let raw_input = raw_input.trim();
        if raw_input.is_empty() {
            continue;
        }

        // 3. Variable expansion
        let mut expanded_input = String::new();
        let mut chars = raw_input.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '$' {
                let mut var_name = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c.is_alphanumeric() || next_c == '_' {
                        var_name.push(next_c);
                        chars.next(); 
                    } else {
                        break;
                    }
                }
                
                if !var_name.is_empty() {
                    if let Ok(val) = std::env::var(&var_name) {
                        expanded_input.push_str(&val);
                    }
                } else {
                    expanded_input.push('$');
                }
            } else {
                expanded_input.push(c);
            }
        }

        // 4. Lexical Analysis (Characters -> Tokens)
        let tokens = match lexer::tokenize(&expanded_input) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("jodsh: lexer error: {:?}", e);
                continue;
            }
        };

        let command_list = match parser::parse(tokens) {
            Ok(cl) => cl,
            Err(parser::ParserError::NoCommand) => continue,
            Err(parser::ParserError::SyntaxError(msg)) => {
                eprintln!("jodsh: syntax error: {}", msg);
                continue;
        // 5. Parsing (Tokens -> AST)
        let cmd = match parser::parse(tokens) {
            Ok(c) => c,
            Err(parser::ParserError::NoCommand) => {
                continue; // Safe handling of empty commands (e.g. from whitespace)
            }
        };

        let mut last_success = true;
        let mut preceding_op = parser::ControlOperator::None;
        let program = &cmd.program;
        let arguments: Vec<&str> = cmd.arguments.iter().map(|s| s.as_str()).collect();

        for (pipeline, following_op) in command_list.pipelines {
            let mut should_run = true;
            match preceding_op {
                parser::ControlOperator::And if !last_success => should_run = false,
                parser::ControlOperator::Or if last_success => should_run = false,
                _ => {}
            }
        // 6. Execute Built-in commands
        match builtins::execute(program, &arguments) {
            builtins::BuiltinStatus::Handled => continue,
            builtins::BuiltinStatus::Exit => break,
            builtins::BuiltinStatus::NotHandled => {} // Fall through to external execution
        }

            if should_run {
                let is_background = following_op == parser::ControlOperator::Background;
                
                match execute_pipeline(pipeline) {
                    Ok(children) => {
                        if is_background {
                            if let Some(last_child) = children.last() {
                                println!("[{}] {}", background_jobs.len() + 1, last_child.id());
                            }
                            background_jobs.extend(children);
                            last_success = true; 
                        } else {
                            let mut success = true;
                            for mut child in children {
                                match child.wait() {
                                    Ok(status) => success = status.success(),
                                    Err(_) => success = false,
                                }
                            }
                            last_success = success;
                        }
                    }
                    Err(ExecResult::Success) => last_success = true,
                    Err(ExecResult::Failure) => last_success = false,
                    Err(ExecResult::Exit) => return,
                }
        // 7. Execute External commands
        match Command::new(program)
            .args(&arguments)
            .status()
        {
            Ok(status) => {
                println!("Process exited with: {}", status);
            }
            
            preceding_op = following_op;
            Err(error) => {
                eprintln!("jodsh: {}", error);
            }
        }
    }
}
}