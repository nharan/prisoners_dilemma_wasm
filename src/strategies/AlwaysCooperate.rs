use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct AlwaysCooperate {}

impl AlwaysCooperate {
    pub fn new() -> Self {
        AlwaysCooperate {}
    }
}

impl Strategy for AlwaysCooperate {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        Action::Cooperate
    }

    fn update_history(&mut self, _opponent_action: Action) {}

    fn name(&self) -> String {
        "Always Cooperate".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
