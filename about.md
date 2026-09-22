# JODSH

### A Modern Linux Shell Built From Scratch in Rust

JODSH is a Linux shell built from the ground up to understand and implement how a Unix/Linux shell works internally.

The project starts with a minimal command-line interpreter and progressively evolves into a feature-rich shell with process management, pipes, redirection, command history, autocomplete, customizable prompts, Git integration, job control, scripting, and package-based distribution.

The primary goal of JODSH is not to clone an existing shell such as Bash or Zsh, but to **understand and implement the core concepts behind Linux shells while building a practical, modern developer-focused shell.**

---

## 🚀 Project Vision

The vision of JODSH is to create a lightweight, customizable, developer-friendly Linux shell with:

* ⚡ Fast command execution
* 🧠 Intelligent command parsing
* 🔎 Powerful command and path completion
* 📜 Persistent command history
* 🎨 Customizable prompts and themes
* 🌿 Git-aware prompt
* 🔀 Pipes and I/O redirection
* ⚙️ Process and job management
* 🛠️ Shell scripting
* 🔌 Future plugin support
* 📦 Native Linux package distribution

JODSH will be developed incrementally, starting from a minimal shell and eventually becoming a complete interactive Unix shell.

---

# 🏗️ Architecture

The high-level architecture of JODSH is planned as:

```text
                         ┌──────────────────┐
                         │      JODSH       │
                         │    Shell Core    │
                         └────────┬─────────┘
                                  │
                                  ▼
                         ┌──────────────────┐
                         │       REPL       │
                         │ Read → Process   │
                         └────────┬─────────┘
                                  │
                                  ▼
                         ┌──────────────────┐
                         │      Lexer       │
                         │  Input → Tokens  │
                         └────────┬─────────┘
                                  │
                                  ▼
                         ┌──────────────────┐
                         │      Parser      │
                         │ Tokens → AST     │
                         └────────┬─────────┘
                                  │
                                  ▼
                         ┌──────────────────┐
                         │     Executor     │
                         └────────┬─────────┘
                                  │
              ┌───────────────────┼───────────────────┐
              ▼                   ▼                   ▼
        ┌──────────┐        ┌──────────┐        ┌──────────┐
        │ Processes│        │   Pipes  │        │Redirects │
        └──────────┘        └──────────┘        └──────────┘
              │
              ▼
        ┌──────────────┐
        │ Job Control  │
        │   Signals    │
        └──────────────┘
```

---

# 🛠️ Tech Stack

## Core Language

| Technology                | Purpose                                 |
| ------------------------- | --------------------------------------- |
| **Rust**                  | Core shell implementation               |
| **Cargo**                 | Build system and package manager        |
| **Rust Standard Library** | Filesystem, processes, environment, I/O |

## Linux / System Programming

| Technology           | Purpose                       |
| -------------------- | ----------------------------- |
| **Linux**            | Primary operating system      |
| **POSIX APIs**       | Unix/Linux system interaction |
| **Processes**        | External command execution    |
| **File Descriptors** | stdin, stdout, stderr         |
| **Pipes**            | Inter-process communication   |
| **Signals**          | Process and terminal control  |
| **Process Groups**   | Job control                   |
| **TTY / PTY**        | Interactive terminal handling |

## Shell Components

| Component           | Purpose                           |
| ------------------- | --------------------------------- |
| REPL                | Interactive shell loop            |
| Lexer               | Tokenizes user input              |
| Parser              | Converts tokens into commands/AST |
| Executor            | Executes commands                 |
| Built-ins           | Commands implemented inside JODSH |
| Environment Manager | Environment variables             |
| History Manager     | Persistent command history        |
| Completion Engine   | Tab completion                    |
| Prompt Engine       | Dynamic shell prompt              |
| Job Manager         | Background/foreground processes   |
| Config Manager      | User configuration                |

## Configuration

Planned configuration format:

```text
TOML
```

Example:

```toml
[prompt]
show_git = true
show_time = false

[history]
enabled = true
max_entries = 10000

[completion]
enabled = true
```

## Development Tools

* VS Code
* Rust Analyzer
* Git
* GitHub
* Linux terminal
* Cargo
* Rust compiler

## Testing

Planned testing stack:

* Rust unit tests
* Rust integration tests
* Shell behavior tests
* Process execution tests
* Parser tests
* Lexer tests
* Pipe tests
* Redirection tests
* Completion tests

## Distribution

Planned Linux distribution:

* Fedora RPM
* Fedora COPR
* GitHub Releases
* Installation script

Future targets:

* Debian/Ubuntu packages
* Arch Linux packages
* Other Linux distributions

---

# ✨ Planned Features

## Phase 1 — Basic Shell

* [ ] Interactive prompt
* [ ] Read user input
* [ ] Execute external commands
* [ ] Exit shell
* [ ] Exit status handling

Example:

```bash
jodsh> ls
jodsh> pwd
jodsh> whoami
jodsh> date
```

---

## Phase 2 — Built-in Commands

* [ ] `cd`
* [ ] `pwd`
* [ ] `exit`
* [ ] `export`
* [ ] `unset`
* [ ] `env`
* [ ] `history`

Example:

```bash
jodsh> cd ~/Projects
jodsh> pwd
```

---

## Phase 3 — Environment Variables

Support:

```bash
echo $HOME
echo $USER
echo $PATH
```

and:

```bash
export NAME=Varun
echo $NAME
```

---

## Phase 4 — Pipes

Support:

```bash
ls | grep rs
```

Multiple pipes:

```bash
cat file.txt | sort | uniq
```

Architecture:

```text
Command A
    │
    │ stdout
    ▼
   PIPE
    │
    │ stdin
    ▼
Command B
```

---

## Phase 5 — I/O Redirection

### Output

```bash
ls > output.txt
```

### Append

```bash
echo hello >> output.txt
```

### Input

```bash
cat < input.txt
```

### Error output

```bash
command 2> error.txt
```

JODSH will manage:

```text
0 → stdin
1 → stdout
2 → stderr
```

---

## Phase 6 — Shell Parser

JODSH will implement its own lexer and parser.

Input:

```bash
ls -la | grep ".rs" > result.txt
```

Processing:

```text
Input
 ↓
Lexer
 ↓
Tokens
 ↓
Parser
 ↓
AST
 ↓
Executor
```

The parser will eventually support:

* Arguments
* Quotes
* Escaping
* Variables
* Pipes
* Redirection
* Command separators
* `&&`
* `||`
* Background execution

---

## Phase 7 — Command History

Persistent history:

```text
~/.config/jodsh/history
```

Features:

* [ ] Save commands
* [ ] Load history
* [ ] Arrow-key navigation
* [ ] History search
* [ ] History size configuration

Example:

```text
jodsh> git status
jodsh> git add .
jodsh> git commit
```

Press:

```text
↑
```

to retrieve previous commands.

---

## Phase 8 — Tab Completion

JODSH will provide:

### Command completion

```text
jodsh> git ch<TAB>
```

Possible results:

```text
checkout
cherry-pick
```

### Path completion

```text
jodsh> cd ~/Pro<TAB>
```

Result:

```text
~/Projects/
```

Planned completion types:

* Commands
* Files
* Directories
* Environment variables
* Built-ins
* Git commands
* Shell-specific commands

---

## Phase 9 — Custom Prompt

JODSH will provide a customizable prompt.

Example:

```text
╭─ jod@fedora ~/Projects/GitPulse
│  git:(main) ✓
╰─❯
```

The prompt can display:

* Username
* Hostname
* Current directory
* Git branch
* Git status
* Exit status
* Execution time
* Custom symbols

---

# 🌿 Git Integration

JODSH will provide optional Git information directly in the prompt.

Example:

```text
╭─ jod@fedora ~/Projects/GitPulse
│   main ✓
╰─❯
```

When changes exist:

```text
╭─ jod@fedora ~/Projects/GitPulse
│   main ●
╰─❯
```

Planned information:

* Current branch
* Repository status
* Modified files
* Staged changes
* Ahead/behind state
* Git operation state

---

# ⚙️ Job Control

JODSH will eventually support Unix job control.

Example:

```bash
sleep 100 &
```

List jobs:

```bash
jobs
```

Bring a job to foreground:

```bash
fg
```

Continue in background:

```bash
bg
```

Signals:

```text
SIGINT
SIGTERM
SIGTSTP
SIGCONT
```

Terminal controls:

```text
Ctrl+C
Ctrl+Z
Ctrl+D
```

This component will manage process groups and terminal ownership.

---

# 📜 Shell Scripting

Future versions will support JODSH scripts.

Example:

```bash
#!/usr/bin/env jodsh

name="Varun"

echo "Hello $name"
```

Potential scripting features:

* Variables
* Conditions
* Loops
* Functions
* Command substitution
* Exit status
* Arguments
* Pipelines

Example:

```bash
if command -v git
then
    echo "Git is installed"
fi
```

---

# 🔌 Future Plugin System

A future plugin architecture may allow developers to extend JODSH.

Possible plugin capabilities:

```text
Custom commands
Prompt extensions
Completion providers
Git integrations
Developer tools
Themes
Aliases
Hooks
```

Potential structure:

```text
~/.config/jodsh/
├── config.toml
├── aliases.toml
├── history
└── plugins/
```

---

# 🎨 Themes

JODSH will eventually support customizable themes.

Planned examples:

```text
Default
Minimal
Cyber
Matrix
Developer
Classic
```

Users may be able to configure:

```toml
[theme]
name = "cyber"
```

---

# 📦 Installation

## Development Installation

Clone the repository:

```bash
git clone https://github.com/YOUR_USERNAME/jodsh.git
cd jodsh
```

Build:

```bash
cargo build
```

Run:

```bash
cargo run
```

---

## Release Build

```bash
cargo build --release
```

Binary:

```text
target/release/jodsh
```

---

## Manual System Installation

```bash
sudo install -m 755 target/release/jodsh /usr/local/bin/jodsh
```

Verify:

```bash
which jodsh
```

Run:

```bash
jodsh
```

---

# 📦 Linux Package Distribution

The long-term goal is to allow users to install JODSH using the native Linux package manager.

For Fedora:

```bash
sudo dnf install jodsh
```

The planned distribution pipeline:

```text
GitHub Repository
       │
       ▼
Source Code
       │
       ▼
RPM Build
       │
       ▼
Fedora COPR
       │
       ▼
DNF Repository
       │
       ▼
sudo dnf install jodsh
```

---

# 🧩 Project Structure

The planned project structure:

```text
jodsh/
│
├── src/
│   ├── main.rs
│   │
│   ├── shell/
│   │   ├── mod.rs
│   │   └── repl.rs
│   │
│   ├── lexer/
│   │   ├── mod.rs
│   │   └── lexer.rs
│   │
│   ├── parser/
│   │   ├── mod.rs
│   │   └── parser.rs
│   │
│   ├── executor/
│   │   ├── mod.rs
│   │   └── executor.rs
│   │
│   ├── builtins/
│   │   ├── mod.rs
│   │   ├── cd.rs
│   │   ├── pwd.rs
│   │   ├── export.rs
│   │   └── history.rs
│   │
│   ├── process/
│   │   ├── mod.rs
│   │   └── jobs.rs
│   │
│   ├── completion/
│   ├── history/
│   ├── prompt/
│   ├── terminal/
│   └── config/
│
├── tests/
│
├── docs/
│
├── packaging/
│   └── fedora/
│
├── Cargo.toml
├── Cargo.lock
├── README.md
└── LICENSE
```

---

# 🗺️ Development Roadmap

```text
                    JODSH
                      │
                      ▼
              ┌───────────────┐
              │   v0.1 Basic   │
              │     Shell      │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.2         │
              │   Built-ins    │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.3         │
              │ Environment    │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.4         │
              │ Pipes/Redirect │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.5         │
              │ Lexer/Parser   │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.6         │
              │ History        │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.7         │
              │ Completion     │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.8         │
              │ Prompt + Git   │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v0.9         │
              │ Job Control    │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │   v1.0         │
              │ Stable Shell   │
              └───────┬───────┘
                      │
                      ▼
              ┌───────────────┐
              │ Distribution   │
              │ RPM / COPR     │
              └───────────────┘
```

---

# 🎯 Version 0.1 Goal

The first version intentionally has very few features.

JODSH v0.1 will support:

* [x] Start shell
* [x] Display prompt
* [x] Read user input
* [x] Execute external Linux commands
* [x] Wait for command completion
* [x] Handle invalid commands
* [x] Exit shell

Example:

```text
JODSH v0.1

jodsh> pwd
/home/user/Projects/jodsh

jodsh> ls
Cargo.toml
Cargo.lock
src

jodsh> echo Hello JODSH
Hello JODSH

jodsh> exit
```

---

# 🧠 Learning Objectives

Building JODSH will provide practical experience with:

### Linux

* Processes
* Process creation
* Process execution
* Environment variables
* File descriptors
* Pipes
* Signals
* Process groups
* TTYs
* Job control
* Filesystem APIs

### Rust

* Ownership and borrowing
* Structs and enums
* Traits
* Error handling
* Modules
* Iterators
* Collections
* Process APIs
* Filesystem APIs
* Concurrency
* Testing

### Computer Science

* Lexical analysis
* Parsing
* Abstract syntax trees
* Inter-process communication
* Operating-system concepts
* Process scheduling concepts
* State machines
* Compiler/interpreter fundamentals

---

# 🔐 Security Considerations

JODSH executes commands with the permissions of the current user.

The project will avoid silently performing privileged operations.

Security considerations will include:

* Safe command parsing
* Correct argument handling
* Avoiding unintended shell expansion
* Environment handling
* Configuration file permissions
* Plugin isolation considerations
* Safe package installation
* Avoiding unnecessary root privileges

---

# 📊 Project Status

Current status:

```text
🚧 Early Development
```

Development target:

```text
Linux
└── Fedora
    └── x86_64
```

Future targets:

```text
Linux
├── x86_64
├── aarch64
└── other architectures
```

---

# 🤝 Development Philosophy

JODSH is being developed incrementally.

The project follows:

```text
Understand
    ↓
Design
    ↓
Implement
    ↓
Test
    ↓
Document
    ↓
Refactor
    ↓
Release
```

Each major shell feature should be understood at the Linux/system level before being implemented.

The project prioritizes **learning, correctness, maintainability, and practical usability** over simply accumulating features.

---

# 📄 License

License: TBD

---

# 👨‍💻 Author

**Varun Kumar R**

JODSH is an independent open-source learning and systems-programming project.

---

## ⭐ JODSH

> **Build the shell. Understand the system. Control the terminal.**
