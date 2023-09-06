use generalized_suffix_tree::suffix_tree::*;
use std::rc::Rc;

#[test]
fn create_tree() {
    let _tree: KGST<char, String> = KGST::new('$');
}

#[test]
fn add_string_no_repeats(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Helo".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone());
    let sstring: Vec<(String, Vec<usize>)> = tree.find(&item_string)
                                                    .into_iter()
                                                    .map(|(treeitem, pos_vec)|{
                                                        (treeitem.get_id().clone(), pos_vec)
                                                    })
                                                    .collect();
    assert_eq!(sstring, vec![(item_id, vec![0 as usize])]);
}

#[test]
fn add_string_repeats(){
    let mut tree: KGST<char, String> = KGST::new('$');
    let item_string:Vec<char> = "Hello".chars().collect();
    let item_id:String = "World".to_string();
    tree.add_string(item_string.clone(), item_id.clone());
    let sstring: Vec<(String, Vec<usize>)> = tree.find(&item_string)
                                                    .into_iter()
                                                    .map(|(treeitem, pos_vec)|{
                                                        (treeitem.get_id().clone(), pos_vec)
                                                    })
                                                    .collect();
    assert_eq!(sstring, vec![(item_id, vec![0 as usize])]);
}

// #[test]
// fn add_string_set(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let string_set: Vec<Vec<char>> = vec!["GATTACAGATTACAXYZGATTACAGATTACA".to_string().chars().collect(), "TTATAGCCGTACAGACCGAA".to_string().chars().collect(), "ATCTTAAGTCATATCACGCGACTAG".to_string().chars().collect()];
//     let id_set:Vec<String> = vec!["first".to_string(),"second".to_string(),"third".to_string()];
//     let it = string_set.iter().zip(id_set.iter());
//     for (string,id) in it{
//         tree.add_string(string.clone(), id.clone());
//     }
//     assert_eq!(tree.find(&"XYZ".to_string().chars().collect()).into_iter().map(|(treeitem, idxs)| {(treeitem.get_id(), idxs)}).collect::<Vec<(&String, Vec<usize>)>>(), vec![(&"first".to_string(), vec![14])]);
// }

// #[test]
// fn add_string_trunc(){
//     let mut tree: KGST<char, String> = KGST::new('$');
//     let item_string:Vec<char> = "Hello".chars().collect();
//     let item_id:String = "World".to_string();
//     tree.add_string(item_string.clone(), item_id, 3);
//     assert_ne!(tree.find(item_string), vec![(&"World".to_string(), &(0 as u32))]);
// }