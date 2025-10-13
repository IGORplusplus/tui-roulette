#[derive(Debug, Default, Clone)]
pub struct MatchData {
    count: i8,
    turn: Option<i8>,
}

impl MatchData {
    pub fn new() -> Self {
        MatchData {
            count: 0,
            turn: None,
        }
    }
}
