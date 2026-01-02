use rusqlite::{Connection, Result};
use todo_app::{Todo, TodoList, TodoMenu, flush_output};

fn main() -> Result<()> {
    let connection = Connection::open("todos.db")?;

    connection.execute(
        "
            create table if not exists todos (
            id integer primary key autoincrement,
            item text not null,
            is_done not null default 0
        )",
        (),
    )?;

    let rows;

    {
        let mut stmt = connection.prepare("select * from todos")?;
        rows = stmt
            .query_map([], |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    item: row.get(1)?,
                    is_done: row.get(2)?,
                })
            })?
            .map(|row| row.unwrap())
            .collect();
    }

    let mut todolist = TodoList::new(connection, rows);

    loop {
        todolist.print_menu();

        flush_output();
        match todolist.read_menu_option() {
            Ok(TodoMenu::List) => todolist.list(),
            Ok(TodoMenu::Add) => todolist.add(),
            Ok(TodoMenu::Delete) => todolist.delete(),
            Ok(TodoMenu::Update) => todolist.update_todo(),
            Ok(TodoMenu::Quit) => {
                break;
            }
            Err(e) => println!("{}", e),
        };
    }
    Ok(())
}
