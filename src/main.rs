use todo_app::{TodoList, TodoMenu, flush_output};

fn main() {
    let mut todolist = TodoList::default();

    loop {
        todolist.print_menu();

        flush_output();
        match todolist.read_menu_option() {
            Ok(TodoMenu::List) => todolist.list(),
            Ok(TodoMenu::Add) => todolist.add(),
            Ok(TodoMenu::Delete) => todolist.delete(),
            Ok(TodoMenu::Update) => todolist.update_todo(),
            Ok(TodoMenu::Quit) => {
                todolist.save();
                break;
            }
            Err(e) => println!("{}", e),
        };
    }
}
