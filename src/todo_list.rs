use crate::{Todo, TodoMenu, flush_output, read};
use rusqlite::{Connection, Result};
use tabled::{Table, Tabled};

#[derive(Debug)]
pub struct TodoList {
    connection: Connection,
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new(connection: Connection, todos: Vec<Todo>) -> Self {
        Self { connection, todos }
    }

    pub fn print_menu(&self) {
        println!(
            r"
1. To add a new Todo Item
2. To Delete a Todo Item
3. To List Todo Items
4. To Update the Todo Item
5. Quit
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
            "5" => Ok(TodoMenu::Quit),
            _ => Err("Not a valid menu option"),
        }
    }

    fn get_todo_item(&self) -> Option<Todo> {
        let mut input = String::new();
        read(&mut input);

        let item = self
            .connection
            .query_row("select * from todos where id = ?1", [input], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    item: row.get(1)?,
                    is_done: row.get(2)?,
                })
            });

        match item {
            Result::Ok(todo) => Some(todo),
            Result::Err(e) => {
                println!("Todo not found {}", e);
                None
            }
        }
    }

    pub fn add(&mut self) {
        let mut input = String::new();
        print!("Please enter todo to add: ");

        flush_output();

        read(&mut input);

        let inserted = self
            .connection
            .execute("insert into todos (item) values (?1)", [input]);

        if inserted.is_err() {
            println!("Failed to add todo")
        }
    }

    pub fn delete(&mut self) {
        print!("Please enter the name of the todo to delete: ");
        flush_output();

        let mut input = String::new();
        read(&mut input);

        let deleted = self
            .connection
            .execute("delete from todos where id = ?1", [input]);

        if deleted.is_err() {
            println!("Failed to delete todo");
        }
    }

    pub fn update_todo(&mut self) {
        print!("Enter the name of the todo to update: ");
        flush_output();

        match self.get_todo_item() {
            Some(item) => {
                print!("Enter y/n to update the todo: ");
                flush_output();
                let mut input = String::new();
                read(&mut input);

                match input.trim() {
                    "y" => {
                        let stmt = self
                            .connection
                            .execute("update todos set is_done = 1 where id = ?1", [item.id]);

                        if stmt.is_err() {
                            println!("Failed to update todo");
                        }
                    }
                    "n" => {
                        let stmt = self
                            .connection
                            .execute("update todos set is_done = 0 where id = ?1", [item.id]);

                        if stmt.is_err() {
                            println!("Failed to update todo");
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
        let mut stmt = self.connection.prepare("select * from todos").unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    item: row.get(1)?,
                    is_done: row.get(2)?,
                })
            })
            .unwrap()
            .map(|row| row.unwrap())
            .collect::<Vec<Todo>>();

        println!("{}", Table::new(rows));
    }
}
