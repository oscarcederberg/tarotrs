use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spread {
    pub name: String,
    pub positions: Vec<Position>,
}