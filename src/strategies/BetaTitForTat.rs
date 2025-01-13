use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct BetaTitForTat {
    history: Vec<Action>,          // Tracks the opponent's actions
    cooperation_rate: f64,         // Tracks the opponent's cooperation rate
    consecutive_defections: usize, // Tracks consecutive defections by the opponent
    punishment_length: usize,      // How long to punish the opponent
    is_punishing: bool,            // Whether we're currently punishing the opponent
}

impl BetaTitForTat {
    pub fn new() -> Self {
        BetaTitForTat {
            history: Vec::new(),
            cooperation_rate: 1.0, // Start by assuming cooperation
            consecutive_defections: 0,
            punishment_length: 1,  // Start with a minimal punishment length
            is_punishing: false,
        }
    }

    /// Update the cooperation rate based on the opponent's action
    fn update_cooperation_rate(&mut self, opponent_action: Action) {
        let total_rounds = self.history.len() as f64;
        if opponent_action == Action::Cooperate {
            self.cooperation_rate = (self.cooperation_rate * total_rounds + 1.0) / (total_rounds + 1.0);
        } else {
            self.cooperation_rate = (self.cooperation_rate * total_rounds) / (total_rounds + 1.0);
        }
    }

    /// Determine the punishment length based on the opponent's behavior
    fn update_punishment_length(&mut self) {
        if self.consecutive_defections > 0 {
            // Increase punishment length based on consecutive defections
            self.punishment_length = self.consecutive_defections.min(10); // Cap at 10 rounds
        } else {
            // Reset punishment length if the opponent cooperates
            self.punishment_length = 1;
        }
    }
}

impl Strategy for BetaTitForTat {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        if self.history.is_empty() {
            // Always cooperate in the first round
            Action::Cooperate
        } else if self.is_punishing {
            // Continue punishing if we're in a punishment phase
            Action::Defect
        } else {
            // Cooperate if the opponent has a high cooperation rate, else defect
            if self.cooperation_rate >= 0.5 {
                Action::Cooperate
            } else {
                Action::Defect
            }
        }
    }

    fn update_history(&mut self, opponent_action: Action) {
        // Update the opponent's action history
        self.history.push(opponent_action);

        // Update the cooperation rate
        self.update_cooperation_rate(opponent_action);

        // Track consecutive defections
        if opponent_action == Action::Defect {
            self.consecutive_defections += 1;
        } else {
            self.consecutive_defections = 0;
        }

        // Update punishment length
        self.update_punishment_length();

        // Start or continue punishing if the opponent defects
        if opponent_action == Action::Defect {
            self.is_punishing = true;
        } else if self.consecutive_defections == 0 {
            // Stop punishing if the opponent cooperates
            self.is_punishing = false;
        }
    }

    fn name(&self) -> String {
        "Beta Tit for Tat".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}