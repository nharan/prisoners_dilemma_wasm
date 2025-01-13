use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct Extortionary {
    last_opponent_action: Action, // Tracks the opponent's last action
    p0: f64, // Probability of cooperating if both cooperated last round
    p1: f64, // Probability of cooperating if I cooperated and opponent defected
    p2: f64, // Probability of cooperating if I defected and opponent cooperated
    p3: f64, // Probability of cooperating if both defected last round
}

impl Extortionary {
    /// Create a new Extortionary strategy with Extort-2 parameters
    pub fn new() -> Self {
        Extortionary {
            last_opponent_action: Action::Cooperate, // Start by assuming cooperation
            p0: 7.0 / 8.0,  // Probability of cooperating if both cooperated last round
            p1: 7.0 / 16.0, // Probability of cooperating if I cooperated and opponent defected
            p2: 3.0 / 8.0,  // Probability of cooperating if I defected and opponent cooperated
            p3: 0.0,        // Probability of cooperating if both defected last round
        }
    }

    /// Determine the probability of cooperating based on the last round's outcome
    fn cooperation_probability(&self, my_last_action: Action, opponent_last_action: Action) -> f64 {
        match (my_last_action, opponent_last_action) {
            (Action::Cooperate, Action::Cooperate) => self.p0,
            (Action::Cooperate, Action::Defect) => self.p1,
            (Action::Defect, Action::Cooperate) => self.p2,
            (Action::Defect, Action::Defect) => self.p3,
        }
    }
}

impl Strategy for Extortionary {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        // Determine the probability of cooperating based on the last round's outcome
        let prob = self.cooperation_probability(self.last_opponent_action, self.last_opponent_action);

        // Cooperate with the calculated probability
        if rand::random::<f64>() < prob {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }

    fn update_history(&mut self, opponent_action: Action) {
        // Update the opponent's last action
        self.last_opponent_action = opponent_action;
    }

    fn name(&self) -> String {
        "Extortionary (Extort-2)".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}