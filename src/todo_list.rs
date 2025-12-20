use std::collections::HashMap;

use crate::{Todo, TodoMenu, flush_output, read};

#[derive(Debug, Default)]
pub struct TodoList {
    todos: HashMap<String, Todo>,
}

impl TodoList {
    fn get_todo_item(&self) -> Option<Todo> {
        let mut input = String::new();
        read(&mut input);
        self.todos.get(input.trim()).cloned()
    }

    pub fn print_menu(&self) {
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

    pub fn read_menu_option(&self) -> Result<TodoMenu, &'static str> {
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

    pub fn add(&mut self) {
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

    pub fn delete(&mut self) {
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

    pub fn update_todo(&mut self) {
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

    pub fn list(&self) {
        for todo in self.todos.values() {
            println!("{}", todo);
        }
    }
}
