use rand::{random_range, seq::IndexedRandom};
use std::collections::HashMap;
pub fn generate_world(list_of_items: Vec<&str>) -> HashMap<&str, Vec<Vec<i32>>> {
    let mut grind: Vec<Vec<i32>> = Vec::new();

    for _ in 0..20 {
        let coords: Vec<i32> = vec![rand::random_range(1..99), rand::random_range(1..99)];
        grind.push(coords);
    };

    //println!("{:?}", grind); Debug

    let mut items_in_world:HashMap<&str, Vec<Vec<i32>>> = HashMap::new();

    for coord in grind {
        if let Some(item_name) = list_of_items.choose(&mut rand::rng()) {
            items_in_world
                .entry(item_name)
                .or_insert(Vec::new())
                .push(coord);
        }
    }

    items_in_world

}