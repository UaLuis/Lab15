struct Block <'a> {
    name: &'a str,
    texture: &'a str,
    item_for_break: &'a str,
}

fn main() {
    let block = get_block_data("stone");
    println!("{:?}", block);

}
pub fn get_block_data(block: &str) -> Vec<&str> {
    let list_blocks = create_block();

    let mut data_block: Vec<&str> = Vec::new();

    if let Some(b) = list_blocks.iter().find(|b| b.name == block) {
        data_block.push(b.name);
        data_block.push(b.texture);
        data_block.push(b.item_for_break);
    } else {
        println!("Error! Block {} not found!", block);
    }

    data_block
}


pub fn get_bloks_list() -> Vec<Block<'static>> {
    let list_blocks = create_block();

    list_blocks
}

fn create_block() -> Vec<Block<'static>> {
    vec![
        Block { name: "ground", texture: "Base_blocks/ground.png", item_for_break: "shovel" },
        Block { name: "stone",  texture: "Base_blocks/stone.png",  item_for_break: "pick"   },
        Block { name: "prapor", texture: "Base_blocks/prapor.png", item_for_break: "hand"   },
    ]
}