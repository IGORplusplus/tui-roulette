#[derive(Default, Debug, Clone)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    Settings,
    Confirmation(ConfirmationType),
}

#[derive(Debug, Clone)]
pub enum ConfirmationType {
    Quit,
    Item(String),
    Generic(String),
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ReloadAmount {
    #[default]
    One = 3,
    Two = 5,
    Three = 8,
    Four = 9,
    Five = 13,
}

impl ReloadAmount {
    pub fn as_usize(&self) -> usize {
        self.clone() as usize
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Copy)]
pub enum ShotgunCycleView {
    #[default]
    Ready,
    Reloading,
    Shooting,
    Blanking,
    Tasing,
    Imposter,
    SelfDestruct,
    Poison
}
