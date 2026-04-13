use crate::network::network;
use std::{collections::HashMap, fs::File, io::{Read, Error}};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
enum GameAction {
    Move { x: i32, y: i32 },
    Place_Block { x: i32, y: i32, block: &str },
    Break_Block { x: i32, y: i32, block: &str },
    Inventory { Block_Items: Vec<&str> }
}

struct Data_World {
    Item
}

pub fn break_block(block_data: Vec<&str>, coord: Vec<i32, i32>) {

}

pub fn place_block(block_data: Vec<&str>, coord: Vec<i32, i32>) {

}

pub fn inventory(inventory_data: Vec<&str>) {

}

pub fn send_world_data(stream: &mut TcpStream, items_of_world: HashMap<&str, Vec<Vec<i32>>>) {
    let buffer: Vec<u8> = serde_json::to_vec(&items_of_world).unwrap();
    network::send_data(stream, &buffer);
}

pub fn get_data_player(stream: &mut TcpStream) -> Vec<&str> {

}

pub fn save_world(items_of_world: HashMap<&str, Vec<Vec<i32>>>, player_data: Vec<&str>) {
    //let create_file: Result<File, Error> = File::create("./world.json");



}