use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct Pavlov {
    last_action: Action, // Tracks the last action taken by this strategy
    last_outcome: Outcome, // Tracks the outcome of the last round
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Win,  // Favorable outcome (e.g., mutual cooperation or successful defection)
    Lose, // Unfavorable outcome (e.g., sucker's payoff or mutual defection)
}

impl Pavlov {
    pub fn new() -> Self {
        Pavlov {
            last_action: Action::Cooperate, // Start by cooperating
            last_outcome: Outcome::Win,     // Assume the first round is favorable
        }
    }

    /// Determine the outcome of the last round based on the opponent's action
    fn determine_outcome(&self, opponent_action: Action) -> Outcome {
        match (self.last_action, opponent_action) {
            // Mutual cooperation or successful defection: Win
            (Action::Cooperate, Action::Cooperate) => Outcome::Win,
            (Action::Defect, Action::Cooperate) => Outcome::Win,

            // Sucker's payoff or mutual defection: Lose
            (Action::Cooperate, Action::Defect) => Outcome::Lose,
            (Action::Defect, Action::Defect) => Outcome::Lose,
        }
    }
}

impl Strategy for Pavlov {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        // Win-Stay, Lose-Shift logic
        match self.last_outcome {
            Outcome::Win => self.last_action, // Repeat the last action
            Outcome::Lose => {
                // Switch the last action
                self.last_action = match self.last_action {
                    Action::Cooperate => Action::Defect,
                    Action::Defect => Action::Cooperate,
                };
                self.last_action
            }
        }
    }

    fn update_history(&mut self, opponent_action: Action) {
        // Update the outcome of the last round
        self.last_outcome = self.determine_outcome(opponent_action);
    }

    fn name(&self) -> String {
        "Pavlov (Win-Stay, Lose-Shift)".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}