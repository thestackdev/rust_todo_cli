use std::fmt;

#[derive(Debug, Clone)]
pub struct Todo {
    pub item: String,
    pub is_done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\nTask: {}\nis_done: {}", self.item, self.is_done)
    }
}
