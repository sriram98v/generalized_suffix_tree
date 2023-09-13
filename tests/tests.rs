use std::collections::{HashSet, HashMap};
use generalized_suffix_tree::suffix_tree::*;

#[test]
fn create_tree() {
    let _tree: KGST<char, String> = KGST::new('$');
}

#[test]
fn add_string_no_repeats(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Helo".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone(), 0);
    let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string);
    assert_eq!(sstring, HashMap::from([(item_id.clone(), HashSet::from([0]))]));
    for i in 0..item_string.len(){
        let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string[i..].to_vec());
        assert_eq!(sstring, HashMap::from([(item_id.clone(), HashSet::from([i]))]));
    }
}

#[test]
fn add_string_repeats(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "GATTAXYZ".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone(), 0);
    let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string);
    assert_eq!(sstring, HashMap::from([(item_id.clone(), HashSet::from([0]))]));
    for i in 0..item_string.len(){
        let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string[i..].to_vec());
        assert_eq!(sstring, HashMap::from([(item_id.clone(), HashSet::from([i]))]));
    }
}

#[test]
fn add_string_set(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "XYZTTATAGCXYZCGTACAGACCGAA".to_string().chars().collect()];
    let id_set:Vec<String> = vec!["first".to_string(),"second".to_string()];
    let it = string_set.iter().zip(id_set.iter());
    for (string,id) in it{
        tree.add_string(string.clone(), id.clone(), 0);
    }
    let sstring: HashMap<String, HashSet<usize>> = tree.find(&"XYZ".chars().collect());
    assert_eq!(sstring, HashMap::from([("first".to_string(), HashSet::from([14])), ("second".to_string(), HashSet::from([0, 10]))]));
}

#[test]
fn serialize_deserialize_tree(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "GATTACAGATTACAXYZGATTACAGATTACA".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone(), 0);
    let serialized = serde_json::to_string(&tree).unwrap();
    let tree_2: KGST<char, String> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(tree.get_nodes(), tree_2.get_nodes());
}


#[test]
fn add_string_no_repeats_trunc(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "abcdefghi".chars().collect();
    let item_id:String = "World".to_string();
    let max_depth: usize = 3;
    tree.add_string(item_string.clone(), item_id.clone(), max_depth);
    let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string);
    assert_ne!(sstring, HashMap::from([(item_id.clone(), HashSet::from([0]))]));
    for j in 1..max_depth+1{
        for i in 0..(item_string.len()-j){
            let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string[i..i+j].to_vec());
            assert_eq!(sstring, HashMap::from([(item_id.clone(), HashSet::from([i]))]));
        }
    }
}

#[test]
fn add_string_repeats_trunc(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "GATTACAGATTACAXYZGATTACAGATTACA".chars().collect();
    let item_id:String = "World".to_string();
    let max_depth: usize = 3;
    tree.add_string(item_string.clone(), item_id.clone(), max_depth);
    let sstring: HashMap<String, HashSet<usize>> = tree.find(&item_string);
    assert_ne!(sstring, HashMap::from([(item_id.clone(), HashSet::from([0]))]));
    let sstring: HashMap<String, HashSet<usize>> = tree.find(&"XYZ".chars().collect());
    assert_eq!(sstring, HashMap::from([(item_id.clone(), HashSet::from([14]))]));
}

// #[test]
// fn add_string_set_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "XYZTTATAGCXYZCGTACAGACCGAA".to_string().chars().collect()];
//     let id_set:Vec<String> = vec!["first".to_string(),"second".to_string()];
//     let it = string_set.iter().zip(id_set.iter());
//     let max_depth: usize = 3;
//     for (string,id) in it{
//         tree.add_string(string.clone(), id.clone(), max_depth);
//     }
//     let sstring: HashMap<String, HashSet<usize>> = tree.find(&"XYZ".chars().collect());
//     assert_eq!(sstring, HashMap::from([("first".to_string(), HashSet::from([14])), ("second".to_string(), HashSet::from([0, 10]))]));
// }