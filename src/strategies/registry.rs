// src/strategies/registry.rs
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::simulation::Strategy;

lazy_static! {
    pub static ref STRATEGIES: Mutex<Vec<Box<dyn Strategy>>> = Mutex::new(Vec::new());
}

pub fn register_strategy(strategy: Box<dyn Strategy>) {
    STRATEGIES.lock().unwrap().push(strategy);
}

pub fn get_strategies() -> Vec<Box<dyn Strategy>> {
    STRATEGIES.lock().unwrap().clone()
}

pub trait StrategyRegistration {
    fn register();
}