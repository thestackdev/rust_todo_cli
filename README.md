# Todo App

A simple command-line todo list application written in Rust.

## Features

- Add new todo items
- Delete todos by name
- List all todos with their status
- Mark todos as done/not done
- Fast lookups using HashMap storage

## Usage

```bash
cargo run
```

Then follow the interactive menu:

```
1. To add a new Todo Item
2. To Delete a Todo Item
3. To List Todo Items
4. To Update the Todo Item
```

Todos are identified by their name, so enter the exact todo text when deleting or updating.

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/todo_app`.
