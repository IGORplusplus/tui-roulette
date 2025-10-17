#[derive(Default, Debug)]
pub enum Menu {
    #[default]
    Off,
    Log,
    Help,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ReloadAmount {
    #[default]
    One = 3,
    Two = 5,
    Three = 6,
    Four = 8,
    Five = 10,
}

impl ReloadAmount {
    pub fn as_usize(&self) -> usize {
        self.clone() as usize
    }
}
