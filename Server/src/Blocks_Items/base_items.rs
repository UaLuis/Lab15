struct Item <'a> {
    name: &'a str,
    texture: &'a str,
    description: &'a str,
}

fn main() {
    let block = get_item_data("axe");
    println!("{:?}", block);

}
pub fn get_item_data(item: &str) -> Vec<&str> {
    let list_items = create_items();

    let mut data_items: Vec<&str> = Vec::new();

    if let Some(b) = list_items.iter().find(|b| b.name == item) {
        data_items.push(b.name);
        data_items.push(b.texture);
        data_items.push(b.description);
    } else {
        println!("Error! Block {} not found!", item);
    }

    data_items
}

pub fn get_items_list() -> Vec<Item<'static>> {
    let list_items = create_items();
    
    list_items
}

fn create_items() -> Vec<Item<'static>> {
    vec![
        Item { name: "axe", texture: "axe.png", description: "Для деревини" },
        Item { name: "pickaxe",  texture: "pickaxe.png",  description: "For stones"   },
        Item { name: "lighter", texture: "lighter.png", description: "For... Boooooo:)"   },
    ]
}