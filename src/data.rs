//data.rs

use crate::components::shotgun::{Shotgun};

#[derive(Debug, Default, Clone)]
pub struct Data {
    pub shotgun: Shotgun,
    pub player_turn: bool,
    pub round_count: usize,
}

impl Data {
    pub fn new() -> Self {
        Self {
            shotgun: Shotgun::new(),
            player_turn: false,
            round_count: 1,
        }
    }
}
