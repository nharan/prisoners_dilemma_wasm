use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// Import the strategies module and its contents
use crate::strategies::{
    tit_for_tat::TitForTat,
    always_cooperate::AlwaysCooperate,
    always_defect::AlwaysDefect,
    random::Random,
    delayed_tit_for_tat::DelayedTitForTat,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrategyResult {
    pub strategy: String,
    pub score: i32,
}

pub fn run_simulation() -> Vec<StrategyResult> {
    // Create a vector of boxed strategies
    let strategies: Vec<Box<dyn Strategy>> = vec![
        Box::new(TitForTat::new()),
        Box::new(AlwaysCooperate::new()),
        Box::new(AlwaysDefect::new()),
        Box::new(Random::new()),
        Box::new(DelayedTitForTat::new()),
    ];

    let mut scores: HashMap<String, i32> = HashMap::new();
    let num_rounds = 100;

    // Run the simulation for each pair of strategies
    for (i, strategy1) in strategies.iter().enumerate() {
        for (j, strategy2) in strategies.iter().enumerate() {
            if i == j {
                continue; // Skip self-play if desired
            }
            let mut s1 = strategy1.clone_box();
            let mut s2 = strategy2.clone_box();
            for _ in 0..num_rounds {
                let action1 = s1.play(&*s2); // Dereference s2 to get &dyn Strategy
                let action2 = s2.play(&*s1); // Dereference s1 to get &dyn Strategy

                let (score1, score2) = calculate_scores(action1, action2);
                *scores.entry(s1.name()).or_insert(0) += score1;
                *scores.entry(s2.name()).or_insert(0) += score2;
                s1.update_history(action2);
                s2.update_history(action1);
            }
        }
    }

    // Convert scores to a sorted leaderboard
    let mut results: Vec<StrategyResult> = scores
        .into_iter()
        .map(|(strategy, score)| StrategyResult { strategy, score })
        .collect();

    results.sort_by(|a, b| b.score.cmp(&a.score)); // Sort by score in descending order
    results
}

// Calculate scores based on the Prisoner's Dilemma payoff matrix
fn calculate_scores(action1: Action, action2: Action) -> (i32, i32) {
    match (action1, action2) {
        (Action::Cooperate, Action::Cooperate) => (3, 3),
        (Action::Defect, Action::Defect) => (1, 1),
        (Action::Cooperate, Action::Defect) => (0, 5),
        (Action::Defect, Action::Cooperate) => (5, 0),
    }
}

// Define the possible actions in the Prisoner's Dilemma
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Action {
    Cooperate,
    Defect,
}

// Define the Strategy trait
pub trait Strategy: Send + Sync {
    fn play(&mut self, opponent: &dyn Strategy) -> Action;
    fn update_history(&mut self, opponent_action: Action);
    fn name(&self) -> String;
    fn clone_box(&self) -> Box<dyn Strategy>;
}

// Implement Clone for Box<dyn Strategy>
impl Clone for Box<dyn Strategy> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}