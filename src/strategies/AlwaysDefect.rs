use crate::simulation::{Action, Strategy};

#[derive(Clone)]
pub struct AlwaysDefect {}

impl AlwaysDefect {
    pub fn new() -> Self {
        AlwaysDefect {}
    }
}

impl Strategy for AlwaysDefect {
    fn play(&mut self, _opponent: &dyn Strategy) -> Action {
        Action::Defect
    }

    fn update_history(&mut self, _opponent_action: Action) {}

    fn name(&self) -> String {
        "Always Defect".to_string()
    }

    fn clone_box(&self) -> Box<dyn Strategy> {
        Box::new(self.clone())
    }
}
