# Todo App

A simple command-line todo list application written in Rust.

## Features

- Add new todo items
- Delete todos by name
- List all todos with their status
- Mark todos as done/not done
- Fast lookups using HashMap storage

## Project Structure

```
src/
├── lib.rs        # Public API exports
├── main.rs       # Binary entry point
├── menu.rs       # TodoMenu enum
├── todo.rs       # Todo struct
├── todo_list.rs  # TodoList operations
└── utils.rs      # I/O helpers
```

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

## As a Library

```rust
use todo_app::{TodoList, Todo};

let mut list = TodoList::default();
```

## Building

```bash
cargo build --release
```

The binary will be available at `target/release/todo_app`.
