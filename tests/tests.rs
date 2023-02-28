use std::collections::HashSet;

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
<<<<<<< HEAD
    let sstring = tree.find(&item_string);
    assert_eq!(sstring, vec![(&"World".to_string(), &(0 as u32))]);
=======
    assert_eq!(tree.find(item_string), vec![(&"World".to_string(), &(0 as u32))]);
>>>>>>> 1f5d44f (switched from i32 to isize)
}

#[test]
fn add_string_set(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "TTATAGCCGTACAGACCGAA".to_string().chars().collect(), "ATCTTAAGTCATATCACGCGACTAG".to_string().chars().collect()];
    let id_set:Vec<String> = vec!["first".to_string(),"second".to_string(),"third".to_string()];
    let it = string_set.iter().zip(id_set.iter());
    for (string,id) in it{
        tree.add_string(string.clone(), id.clone());
    }
<<<<<<< HEAD
    assert_eq!(tree.find(&"XYZ".to_string().chars().collect()), vec![(&"first".to_string(), &(14 as u32))]);
=======
    assert_eq!(tree.find("XYZ".to_string().chars().collect()), vec![(&"first".to_string(), &(14 as u32))]);
>>>>>>> 1f5d44f (switched from i32 to isize)
}

#[test]
fn serialize_tree(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Hello".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id);
    println!("{}", serde_json::to_string(&tree).unwrap());
}

#[test]
fn deserialize_tree(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Hello".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id);
    let json_str:String = serde_json::to_string(&tree).unwrap();
    let _new_tree: KGST<char, String> = serde_json::from_str(&json_str).unwrap();
    // assert_eq!(tree, new_tree);
}

#[test]
fn exact_pattern_match(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "GATTACAGATTACAXYZGATTACAGATTACA".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id);
    let item_string:Vec<char> = "BANATA".chars().collect();
    let item_id:String = "World2".to_string();
    tree.add_string(item_string.clone(), item_id);
    println!("{}", serde_json::to_string(&tree).unwrap());
    let matches: HashSet<(&String, &u32)> = tree.find(&"XYZ".to_string().chars().collect()).into_iter().collect();
    assert_eq!(matches, HashSet::from([(&"World".to_string(), &(14 as u32))]));
    let matches: HashSet<(&String, &u32)> = tree.find(&"GATTA".to_string().chars().collect()).into_iter().collect();
    assert_eq!(matches, HashSet::from([(&"World".to_string(), &(0 as u32)), (&"World".to_string(), &(7 as u32)), (&"World".to_string(), &(17 as u32)), (&"World".to_string(), &(24 as u32))]));
    let matches: HashSet<(&String, &u32)> = tree.find(&"ANA".to_string().chars().collect()).into_iter().collect();
    assert_eq!(matches, HashSet::from([(&"World2".to_string(), &(1 as u32))]));
    let matches: HashSet<(&String, &u32)> = tree.find(&"AT".to_string().chars().collect()).into_iter().collect();
    assert_eq!(matches, HashSet::from([(&"World".to_string(), &(1 as u32)), (&"World".to_string(), &(8 as u32)), (&"World".to_string(), &(18 as u32)), (&"World".to_string(), &(25 as u32)), (&"World2".to_string(), &(3 as u32))]));
}
