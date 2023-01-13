use generalized_suffix_tree::suffix_tree::*;

#[test]
fn create_tree() {
    let tree: KGST<char, String> = KGST::new('$');
}

#[test]
fn add_string_full(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Hello".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id, 20);
    assert_eq!(tree.find(item_string), vec![(&"World".to_string(), &(0 as i32))]);
}

#[test]
fn add_string_trunc(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Hello".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id, 3);
    assert_ne!(tree.find(item_string), vec![(&"World".to_string(), &(0 as i32))]);
}