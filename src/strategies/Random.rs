use crate::simulation::{Action, Strategy};
use rand::Rng;

#[derive(Clone)]
pub struct Random {}

impl Random {
    pub fn new() -> Self {
        Random {}
    }
}

impl Strategy for Random {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        if rand::thread_rng().gen_bool(0.5) {
            Action::Cooperate
        } else {
            Action::Defect
        }
    }

    fn update_history(&mut self, _opponent_action: Action) {}

    fn name(&self) -> String {
        "Random".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
