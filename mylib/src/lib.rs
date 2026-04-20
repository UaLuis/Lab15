use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum GameAction {
    Move { x: i32, y: i32 },
    PlaceBlock { x: i32, y: i32, block: String },
    BreakBlock { x: i32, y: i32, block: String },
    Inventory { block_items: Vec<String> },
}