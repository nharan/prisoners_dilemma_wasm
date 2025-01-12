use crate::simulation::{Action, Strategy};

    #[derive(Clone)]
    pub struct DelayedTitForTat {
        history: Vec<Action>,
        delay: usize,
    }

    impl DelayedTitForTat {
        pub fn new() -> Self {
            DelayedTitForTat {
                history: Vec::new(),
                delay: 2,
            }
        }
    }

    impl Strategy for DelayedTitForTat {
        fn play(&mut self, opponent: &dyn Strategy) -> Action {
            if self.history.len() < self.delay {
                Action::Cooperate
            } else {
                *self.history.get(self.history.len() - self.delay).unwrap()
            }
        }

        fn update_history(&mut self, opponent_action: Action) {
            self.history.push(opponent_action);
        }

        fn name(&self) -> String {
            "Delayed Tit for Tat".to_string()
        }

        fn clone_box(&self) -> Box<dyn Strategy> {
            Box::new(self.clone())
        }
    }
