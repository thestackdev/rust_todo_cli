use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

enum TodoMenu {
    List,
    Add,
    Delete,
    Update,
}

#[derive(Debug, Clone)]
struct Todo {
    item: String,
    is_done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\nTask: {}\nis_done: {}", self.item, self.is_done)
    }
}

#[derive(Debug)]
struct TodoList {
    todos: HashMap<String, Todo>,
}

impl TodoList {
    fn new() -> Self {
        Self {
            todos: HashMap::new(),
        }
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
        self.todos.insert(
            input.trim().to_string(),
            Todo {
                item: input.trim().to_string(),
                is_done: false,
            },
        );
    }

    fn get_todo_item(&self) -> Option<Todo> {
        let mut input = String::new();
        read(&mut input);
        self.todos.get(input.trim()).cloned()
    }

    fn delete(&mut self) {
        print!("Please enter the name of the todo to delete: ");
        flush_output();

        match self.get_todo_item() {
            Some(item) => {
                self.todos.remove(&item.item);
            }
            _ => {
                println!("Todo not found");
            }
        };
    }

    fn update_todo(&mut self) {
        print!("Enter the name of the todo to update: ");
        flush_output();

        match self.get_todo_item() {
            Some(item) => {
                print!("Enter y/n to update the todo");
                flush_output();
                let mut input = String::new();
                read(&mut input);

                match input.trim() {
                    "y" | "n" => {
                        let response = self.todos.insert(
                            item.item.to_string(),
                            Todo {
                                item: item.item,
                                is_done: input.trim() == "y",
                            },
                        );
                        match response {
                            Some(inserted) => {
                                println!(
                                    "Todo: {}, Update to {}",
                                    inserted.item,
                                    input.trim() == "y"
                                )
                            }
                            _ => {
                                println!("Failed to insert item");
                            }
                        }
                    }
                    _ => {
                        println!("Please enter the y/n");
                    }
                }
            }
            _ => println!("Todo not found"),
        }
    }

    fn list(&self) {
        for todo in self.todos.values() {
            println!("{}", todo);
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
