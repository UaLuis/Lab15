mod blocks_items;
mod logick;
mod network;

use logick::{base_logick, generate_world};
use blocks_items::{base_items, base_blocks};
use std::collections::HashMap;


fn main() {
    let mut list_of_items_blocks: Vec<&str> = vec!["ground", "stone", "prapor", "axe", "pick", "lighter"];
    let items_of_world: HashMap<&str, Vec<Vec<i32>>> = generate_world::generate_world(list_of_items_blocks);

    if let Some(item) = items_of_world.get("pick") {
        println!("ground coords: {:?}", item);
    }

     
}
