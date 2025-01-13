use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct TitForTat {
    history: Vec<Action>,
}

impl TitForTat {
    pub fn new() -> Self {
        TitForTat { history: Vec::new() }
    }
}

impl Strategy for TitForTat {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        if self.history.is_empty() {
            Action::Cooperate
        } else {
            *self.history.last().unwrap()
        }
    }

    fn update_history(&mut self, opponent_action: Action) {
        self.history.push(opponent_action);
    }

    fn name(&self) -> String {
        "Tit for Tat".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}