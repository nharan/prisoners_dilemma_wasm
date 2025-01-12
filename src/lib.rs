use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json;

// Declare the `strategies` module
pub mod strategies;

// Declare the `simulation` module
pub mod simulation;

#[wasm_bindgen]
pub fn run_simulation() -> Result<String, JsValue> {
    let leaderboard = simulation::run_simulation();
    match serde_json::to_string(&leaderboard) {
        Ok(json_string) => Ok(json_string),
        Err(e) => Err(JsValue::from_str(&format!("Error serializing leaderboard: {}", e))),
    }
}