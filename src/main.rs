use std::fmt;
use std::io::{self, Write};

enum TodoMenu {
    List,
    Add,
    Delete,
    Update,
}

#[derive(Debug)]
struct Todo {
    item: String,
    is_done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} is_done: {}", self.item, self.is_done)
    }
}

#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> Self {
        Self { todos: Vec::new() }
    }

    fn print_menu(&self) {
        println!(
            r"
1. To add a new Todo Item
2. To Delete a Todo Item
3. To List Todo Items
4. To Update the Todo Item
        "
        );

        print!("Please select an option: ");
    }

    fn read_menu_option(&self) -> Result<TodoMenu, &'static str> {
        let mut input = String::new();
        read(&mut input);

        match input.trim() {
            "1" => Ok(TodoMenu::Add),
            "2" => Ok(TodoMenu::Delete),
            "3" => Ok(TodoMenu::List),
            "4" => Ok(TodoMenu::Update),
            _ => Err("Not a valid menu option"),
        }
    }

    fn add(&mut self) {
        let mut input = String::new();
        print!("Please enter todo to add: ");

        flush_output();

        read(&mut input);
        self.todos.push(Todo {
            item: input.trim().to_string(),
            is_done: false,
        });
    }

    fn get_todo_position(&self) -> Option<usize> {
        let mut input = String::new();
        read(&mut input);

        match input.trim().parse::<usize>() {
            Ok(num) => {
                if num < self.todos.len() {
                    return Some(num);
                }
                println!("No todo found at the given position");
                None
            }
            Err(e) => {
                print!("Failed to parse position {}", e);
                None
            }
        }
    }

    fn delete(&mut self) {
        print!("Please enter the position of the todo to delete: ");
        flush_output();

        match self.get_todo_position() {
            Some(num) => {
                self.todos.remove(num);
            }
            _ => {
                println!("Unable to parse position");
            }
        };
    }

    fn update_todo(&mut self) {
        print!("Enter the todo position of the todo to update: ");
        flush_output();

        match self.get_todo_position() {
            Some(num) => {
                print!("Enter y/n to update the todo");
                flush_output();
                let mut input = String::new();
                read(&mut input);

                match input.trim() {
                    "y" => {
                        self.todos[num].is_done = true;
                        println!("{}", self.todos[num]);
                    }
                    "n" => {
                        self.todos[num].is_done = false;
                        println!("{}", self.todos[num]);
                    }
                    _ => {
                        println!("Please enter the y/n");
                    }
                }
            }
            _ => println!("Unable to parse position"),
        }
    }

    fn list(&self) {
        for (index, todo) in self.todos.iter().enumerate() {
            println!("{index}. Todo: {}; Status: {}", todo.item, todo.is_done);
        }
    }
}

fn read(input: &mut String) {
    io::stdin().read_line(input).expect("Failed to read input");
}

fn flush_output() {
    io::stdout().flush().expect("Failed to flush output");
}

fn main() {
    let mut todolist = TodoList::new();

    loop {
        todolist.print_menu();

        flush_output();
        match todolist.read_menu_option() {
            Ok(TodoMenu::List) => todolist.list(),
            Ok(TodoMenu::Add) => todolist.add(),
            Ok(TodoMenu::Delete) => todolist.delete(),
            Ok(TodoMenu::Update) => todolist.update_todo(),
            Err(e) => println!("{}", e),
        };
    }
}
