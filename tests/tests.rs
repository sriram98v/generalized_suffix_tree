use std::collections::HashSet;

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
    tree.add_string(item_string.clone(), item_id.clone(), None);
    let sstring: Vec<(String, Vec<usize>)> = tree.find(&item_string)
                                                    .into_iter()
                                                    .map(|(treeitem, pos_vec)|{
                                                        (treeitem.get_id().clone(), pos_vec)
                                                    })
                                                    .collect();
    assert_eq!(sstring, vec![(item_id.clone(), vec![0 as usize])]);
    for i in 0..item_string.len(){
        let sstring: Vec<(String, Vec<usize>)> = tree.find(&item_string[i..].to_vec())
                                                    .into_iter()
                                                    .map(|(treeitem, pos_vec)|{
                                                        (treeitem.get_id().clone(), pos_vec)
                                                    })
                                                    .collect();
        assert_eq!(sstring, vec![(item_id.clone(), vec![i])]);
    }
}

#[test]
fn add_string_repeats(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "GATTAXYZ".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone(), None);
    let sstring: Vec<(String, Vec<usize>)> = tree.find(&item_string)
                                                    .into_iter()
                                                    .map(|(treeitem, pos_vec)|{
                                                        (treeitem.get_id().clone(), pos_vec)
                                                    })
                                                    .collect();
    assert_eq!(sstring, vec![(item_id.clone(), vec![0 as usize])]);
    for i in 0..item_string.len(){
        let sstring: Vec<(String, Vec<usize>)> = tree.find(&item_string[i..].to_vec())
                                                    .into_iter()
                                                    .map(|(treeitem, pos_vec)|{
                                                        (treeitem.get_id().clone(), pos_vec)
                                                    })
                                                    .collect();
        assert_eq!(sstring, vec![(item_id.clone(), vec![i])]);
    }
}

#[test]
fn add_string_set(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "XYZTTATAGCCGTACAGACCGAA".to_string().chars().collect()];
    let id_set:Vec<String> = vec!["first".to_string(),"second".to_string()];
    let it = string_set.iter().zip(id_set.iter());
    for (string,id) in it{
        tree.add_string(string.clone(), id.clone(), None);
    }
    assert_eq!(tree.find(&"XYZ".to_string().chars().collect()).into_iter().map(|(treeitem, idxs)| {(treeitem.get_id(), idxs)}).collect::<HashSet<(&String, Vec<usize>)>>(), HashSet::from([(&"first".to_string(), vec![14]), (&"second".to_string(), vec![0])]));
}

#[test]
fn serialize_deserialize_tree(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "GATTACAGATTACAXYZGATTACAGATTACA".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone(), None);
    let serialized = serde_json::to_string(&tree).unwrap();
    let tree_2: KGST<char, String> = serde_json::from_str(&serialized).unwrap();
    assert_eq!(tree.get_nodes(), tree_2.get_nodes());
}


// #[test]
// fn add_string_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let item_string:Vec<char> = "Hello".chars().collect();
//     let item_id:String = "World".to_string();
//     tree.add_string(item_string.clone(), item_id, 3);
//     assert_ne!(tree.find(item_string), vec![(&"World".to_string(), &(0 as u32))]);
// }