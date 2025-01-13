use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct NPavlov {
    last_action: Action,           // Tracks the last action taken by this strategy
    memory: Vec<Outcome>,          // Tracks the outcomes of the last N rounds
    memory_size: usize,            // Number of rounds to remember (N)
    cooperation_threshold: f64,    // Threshold for cooperation (e.g., 0.5 for majority)
}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Win,  // Favorable outcome (e.g., mutual cooperation or successful defection)
    Lose, // Unfavorable outcome (e.g., sucker's payoff or mutual defection)
}

impl NPavlov {
    pub fn new() -> Self {
        NPavlov {
            last_action: Action::Cooperate, // Start by cooperating
            memory: Vec::with_capacity(5),  // Default memory size: 5 rounds
            memory_size: 5,                 // Default memory size: 5 rounds
            cooperation_threshold: 0.6,     // Default threshold: 60% cooperation
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

    /// Calculate the proportion of favorable outcomes in memory
    fn cooperation_rate(&self) -> f64 {
        let wins = self.memory.iter().filter(|&o| *o == Outcome::Win).count();
        wins as f64 / self.memory.len() as f64
    }
}

impl Strategy for NPavlov {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        if self.memory.len() < self.memory_size {
            // Not enough history; default to cooperating
            Action::Cooperate
        } else {
            // Decide based on the proportion of favorable outcomes
            if self.cooperation_rate() >= self.cooperation_threshold {
                self.last_action // Win-Stay: Repeat the last action
            } else {
                // Lose-Shift: Switch the last action
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
        let outcome = self.determine_outcome(opponent_action);

        // Add the outcome to memory
        if self.memory.len() == self.memory_size {
            self.memory.remove(0); // Remove the oldest outcome if memory is full
        }
        self.memory.push(outcome);
    }

    fn name(&self) -> String {
        format!("N-Pavlov (Memory: {}, Threshold: {})", self.memory_size, self.cooperation_threshold)
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}