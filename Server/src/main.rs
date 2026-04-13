mod Network;
mod Logick;
mod Blocks_Items;

use std::collections::HashMap;


fn main() {
    let list_of_items: Vec<&str> = vec!["Grass", "Stone", "Iron_Grind", "Iron"];
    let items_of_world: HashMap<&str, Vec<Vec<i32>>> = Logick::generate_world::generate_world(list_of_items);

    if let Some(item) = items_of_world.get("Iron") {
        println!("Iron coords: {:?}", item);
    }

     
}
