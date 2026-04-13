mod blocks_items;
mod logick;
mod network;

use logick::{base_logick, generate_world};
use blocks_items::{base_items, base_blocks};
use std::{collections::HashMap, fs::File, io::Error};


fn main() {
    let mut list_of_items_blocks: Vec<&str> = vec!["ground", "stone", "prapor", "axe", "pick", "lighter"];
    let create_file: Result<File, Error> = File::create("./World/world.json");
    let create_file: Result<File, Error> = File::create("./World/players.json");

    let mut items_of_world: HashMap<&str, Vec<Vec<i32>>> = generate_world::generate_world(list_of_items_blocks);

    if let Some(item) = items_of_world.get("pick") {
        println!("ground coords: {:?}", item);
    }




}
