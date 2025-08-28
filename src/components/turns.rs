//turns.rs
pub struct TurnSystem {
    pub current_turn: usize,
}

impl TurnSystem {
    pub fn new() -> Self {
        Self { current_turn: 0 }
    }
    
    /// Advances the turn and triggers reload if needed
    pub fn advance_turn(&mut self, data: &mut Data) {
        self.current_turn += 1;
        println!("Turn {}", self.current_turn);
        
        // Reload shotgun every 3 turns
        if self.current_turn % 3 == 0 {
            println!("Reloading shotgun...");
            data.shotgun.load_random_shells(5);
        }
    }
}
