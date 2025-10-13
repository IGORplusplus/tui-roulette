//data.rs

use crate::components::shotgun::{Shotgun};
use crate::components::match_data::{MatchData};

//need to implement things which would allow default and clone
#[derive(Debug, Default, Clone)]
pub struct Data {
    pub shotgun: Shotgun,
    pub match_data: MatchData,
}

impl Data {
    pub fn new() -> Self {
        Self {
            shotgun: Shotgun::new(),
            match_data: MatchData::new(),
        }
    }
}
