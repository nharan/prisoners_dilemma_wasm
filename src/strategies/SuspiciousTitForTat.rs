use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct SuspiciousTitForTat {
    last_opponent_action: Option<Action>, // Tracks the opponent's last action
}

impl SuspiciousTitForTat {
    pub fn new() -> Self {
        SuspiciousTitForTat {
            last_opponent_action: None, // No history initially
        }
    }
}

impl Strategy for SuspiciousTitForTat {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        if self.last_opponent_action.is_none() {
            // Defect on the first round
            Action::Defect
        } else {
            // Imitate the opponent's last action
            self.last_opponent_action.unwrap()
        }
    }

    fn update_history(&mut self, opponent_action: Action) {
        // Update the opponent's last action
        self.last_opponent_action = Some(opponent_action);
    }

    fn name(&self) -> String {
        "Suspicious Tit for Tat".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}