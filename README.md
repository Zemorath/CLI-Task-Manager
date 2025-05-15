# Tasker: CLI Task Manager
A simple command-line task manager written in Rust. Manage tasks with add, list, update, delete, and complete commands. Tasks are stored in `tasks.json`.

## Setup
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Clone this repo and run: `cargo build`
3. Use commands like: `cargo run -- add "Do homework"`

## Features
- Add tasks with descriptions.
- List all tasks with status.
- Update or delete tasks by ID.
- Mark tasks as complete.

## Dependencies
- `clap`: CLI parsing.
- `serde`: JSON serialization.