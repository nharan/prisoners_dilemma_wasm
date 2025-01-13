use crate::simulation::{Action, Strategy};
use rand::Rng;

#[derive(Clone)]
pub struct GenerousTitForTat {
    last_opponent_action: Option<Action>, // Tracks the opponent's last action
    cooperation_probability: f64,        // Probability of cooperating after a defection
}

impl GenerousTitForTat {
    pub fn new() -> Self {
        // Default payoffs for the Prisoner's Dilemma
        let reward = 3;      // R: Reward for mutual cooperation
        let punishment = 1;  // P: Punishment for mutual defection
        let temptation = 5;  // T: Temptation to defect
        let sucker = 0;      // S: Sucker's payoff

        // Calculate the cooperation probability g(R, P, T, S)
        let g1 = 1.0 - (temptation as f64 - reward as f64) / (reward as f64 - sucker as f64);
        let g2 = (reward as f64 - punishment as f64) / (temptation as f64 - punishment as f64);
        let cooperation_probability = g1.min(g2);

        GenerousTitForTat {
            last_opponent_action: None, // No history initially
            cooperation_probability,
        }
    }
}

impl Strategy for GenerousTitForTat {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        if self.last_opponent_action.is_none() {
            // Cooperate on the first round
            Action::Cooperate
        } else {
            match self.last_opponent_action.unwrap() {
                Action::Cooperate => {
                    // Cooperate if the opponent cooperated last round
                    Action::Cooperate
                }
                Action::Defect => {
                    // Cooperate with probability g(R, P, T, S) after a defection
                    if rand::thread_rng().gen::<f64>() < self.cooperation_probability {
                        Action::Cooperate
                    } else {
                        Action::Defect
                    }
                }
            }
        }
    }

    fn update_history(&mut self, opponent_action: Action) {
        // Update the opponent's last action
        self.last_opponent_action = Some(opponent_action);
    }

    fn name(&self) -> String {
        "Generous Tit for Tat".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}