use generalized_suffix_tree::suffix_tree::*;

#[test]
fn create_tree() {
    let _tree: KGST<char, String> = KGST::new('$');
}

#[test]
fn add_string_full(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Hello".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id);
    let sstring = tree.find(item_string);
    assert_eq!(sstring, vec![(&"World".to_string(), &(0 as usize))]);
}

#[test]
fn add_string_set(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "TTATAGCCGTACAGACCGAA".to_string().chars().collect(), "ATCTTAAGTCATATCACGCGACTAG".to_string().chars().collect()];
    let id_set:Vec<String> = vec!["first".to_string(),"second".to_string(),"third".to_string()];
    let it = string_set.iter().zip(id_set.iter());
    for (string,id) in it{
        tree.add_string(string.clone(), id.clone(), 0);
    }
    assert_eq!(tree.find("XYZ".to_string().chars().collect()), vec![(&"first".to_string(), &(14 as usize))]);
}

// #[test]
// fn add_string_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let item_string:Vec<char> = "Hello".chars().collect();
//     let item_id:String = "World".to_string();
//     tree.add_string(item_string.clone(), item_id, 3);
//     assert_ne!(tree.find(item_string), vec![(&"World".to_string(), &(0 as u32))]);
// }