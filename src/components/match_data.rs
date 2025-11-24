#[derive(Debug, Default, Clone)]
pub struct MatchData {
    //try my best not always operate on these values directly
    round_count: usize,
    //represents the players
    turn: Option<usize>,
    player_turn: Option<u8>,
}

impl MatchData {

    pub fn new() -> Self {
        MatchData {
            round_count: 1,
            turn: None,
            player_turn: None,
        }
    }

    pub fn round_count(&self) -> usize {
        self.round_count
    }

    pub fn next_round(&mut self) {
        self.round_count += 1;
    }

    pub fn update_turn(&mut self, new_turn: Option<u8>) {
        self.player_turn = new_turn;
    }
}
