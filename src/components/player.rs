use crate::components::items::Items;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    id: u8,
    health: u8,
    items: Vec<Items>
}
