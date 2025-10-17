#[derive(Debug, Default, Clone)]
pub struct MatchData {
    //round count
    count: u8,
    //represents the players
    turn: Option<u16>,
}

impl MatchData {
    pub fn new() -> Self {
        MatchData {
            count: 1,
            turn: None,
        }
    }

    pub fn incr_round(&mut self) {
        self.count += 1;
    }

    pub fn count(&self) -> u8 {
        self.count
    }

    pub fn reset_count(&mut self) {
        self.count = 0;
    }
}
