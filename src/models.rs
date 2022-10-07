use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Params {
    pub a: f64, 
    pub b: f64,
}

#[derive(Serialize)]
pub struct CalculatorResult {
    pub result: f64,
}

impl CalculatorResult {
    pub fn new(result: f64) -> CalculatorResult {
        CalculatorResult { result }
    }
}