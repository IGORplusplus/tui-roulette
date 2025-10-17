//data.rs

use crate::components::shotgun::{Shotgun};

//need to implement things which would allow default and clone
#[derive(Debug, Default, Clone)]
pub struct Data {
    pub shotgun: Shotgun,
}

impl Data {
    pub fn new() -> Self {
        Self {
            shotgun: Shotgun::new(),
        }
    }
}
