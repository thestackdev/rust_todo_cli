use tabled::Tabled;

#[derive(Debug, Clone, Tabled)]
pub struct Todo {
    pub id: u32,
    pub item: String,
    pub is_done: u8,
}
