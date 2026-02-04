use crate::components::player::Player;

#[derive(Debug, Default, Clone)]
pub struct MatchData {
    //try my best not always operate on these values directly
    pub round_count: usize,
    //represents the players
    pub player_turn: Option<usize>,
    pub players: Vec<Player>,
}

impl MatchData {

    pub fn new() -> Self {
        MatchData {
            round_count: 1,
            player_turn: None,
            players: vec![Player::new(), Player::new()],
        }
    }

    pub fn round_count(&self) -> usize {
        self.round_count
    }

    pub fn next_round(&mut self) {
        self.round_count += 1;
    }

    pub fn update_turn(&mut self, new_turn: Option<usize>) {
        self.player_turn = new_turn;
    }

    pub fn current_player(&self) -> Option<&Player> {
        self.player_turn
            .and_then(|turn| self.players.get(turn as usize))
    }

    pub fn first_player_id(&self) -> Option<usize> {
        let id = self.players[0].id;
        Some(id)
    }

    pub fn second_player_id(&self) -> Option<usize> {
        let id = self.players[1].id;
        Some(id)
    }

}
