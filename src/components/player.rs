use crate::components::items::Items;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    pub id: u32,
    pub health: u8,
    items: Vec<Items>
}

impl Player {

    pub fn new() -> Self {
	let mut rng = rand::thread_rng();
	let id: u32 = rng.gen_range(1..=3511);
        Player {
            name: String::new(),
            id,
            health: 5,
            items: Vec::new(),
        }
    }

    pub fn use_item(&mut self, item: Items) {
        if let Some(pos) = self.items.iter().position(|it| *it == item) {
	    self.items.remove(pos);
	}
    }

    pub fn items(&self) -> &Vec<Items> {
	&self.items
    }

    pub fn name_player(&mut self, name: String) {
	self.name = name;
    }
}
