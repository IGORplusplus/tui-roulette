use crate::components::items::Items;
use rand::Rng;

const PLAYER_ART: &str = include_str!("../assets/player_icon.txt");

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub id: usize,
    pub health: u8,
    pub items: Vec<Items>,
    pub art: &'static str,
}

impl Player {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen_range(1..=3511);
        Player {
            name: String::new(),
            id,
            health: 5,
            items: Vec::new(),
            art: PLAYER_ART,
        }
    }

    pub fn new_art(art: &'static str) -> Self {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen_range(1..=3511);
        Player {
            name: String::new(),
            id,
            health: 5,
            items: Vec::new(),
            art,
        }
    }

    pub fn use_item(&mut self, item: Items) {
        if let Some(pos) = self.items.iter().position(|it| *it == item) {
            self.items.remove(pos);
        }
    }

    pub fn name_player(&mut self, name: String) {
        self.name = name;
    }
}
