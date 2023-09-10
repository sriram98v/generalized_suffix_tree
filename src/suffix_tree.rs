use crate::suffix_node::Node;
use crate::tree_item::TreeItem;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::option::Option;
use serde::{Serialize, Deserialize};


#[derive(Debug)]
struct ActivePoint<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub active_length: usize,
    pub active_edge_index: usize,
    pub active_edge: Option<T>,
    pub active_node: usize,

}

impl<T> ActivePoint<T> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub fn new()->ActivePoint<T>{
        ActivePoint{
            active_length: 0,
            active_edge_index: 0,
            active_edge: None,
            active_node: 0,
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct KGST<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    root: usize,
    nodes: HashMap<usize, Node<T>>,
    terminal_character: T,
    strings: HashMap<usize, (TreeItem<T, U>, Option<usize>)>,
    leaves: Vec<usize>,
}


impl<T, U> KGST<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub fn new(terminal_character: T)->KGST<T, U>{
        KGST{
            nodes: HashMap::from([(0, Node{
                children: HashMap::new(),
                suffix_link: None,
                string_id: None,
                data: HashMap::new(),
                parent: None,
                end: Some(0),
                start: 0
            })]),
            root: 0,
            terminal_character: terminal_character,
            strings: HashMap::new(),
            leaves: Vec::new(),
        }
    }

    pub fn clear(&mut self){
        // self.num_nodes = 1;
        self.root = 0;
        // self.num_nodes = 1;
        self.nodes = HashMap::from([(0, Node{
            children: HashMap::new(),
            suffix_link: None,
            string_id: None,
            data: HashMap::new(),
            parent: None,
            end: Some(0),
            start: 0
        })]);
        self.strings = HashMap::new();
        // self.start_idx = 0;
        self.leaves = Vec::new();
    }

    fn leaves_of_node(&self, node_id:&usize, leaves:&mut Vec<usize>){
        if !self.nodes.get(node_id).unwrap().has_children(){
            leaves.push(node_id.clone());
        }

        for child_node_id in self.nodes.get(node_id).unwrap().get_children().values(){
            self.leaves_of_node(child_node_id, leaves);
        }   
    }

    pub fn get_strings(&self)->&HashMap<usize, (TreeItem<T, U>, Option<usize>)>{
        &self.strings
    }

    pub fn get_nodes(&self)->&HashMap<usize, Node<T>>{
        &self.nodes
    }

    pub fn get_node(&self, node_id: &usize)->Option<&Node<T>>{
        self.nodes.get(node_id)
    }

    pub fn get_node_mut(&mut self, node_id: &usize)->Option<&mut Node<T>>{
        self.nodes.get_mut(node_id)
    }

    pub fn get_string_id(&self, treeitem_id: &usize)->Option<&U>{
        match self.strings.get(treeitem_id){
            None => None,
            Some(treeitem) => Some(treeitem.0.get_id())
        }
    }

    pub fn get_string(&self, treeitem_id: &usize)->Option<&Vec<T>>{
        match self.strings.get(treeitem_id){
            None => None,
            Some(treeitem) => Some(treeitem.0.get_string())
        }
    }

    pub fn get_root(&self)->&Node<T>{
        self.get_node(&0).unwrap()
    }

    pub fn get_treeitem(&self, treeitem_id: &usize)->Option<&(TreeItem<T, U>, Option<usize>)>{
        self.strings.get(treeitem_id)
    }

    fn get_pattern_node(&self, q_string:&Vec<T>)->Option<&usize>{
        let mut node_id: Option<&usize> = Some(&self.root);
        let mut c: &T = &q_string[0];
        let mut i = 0;
        loop {
            node_id = self.nodes.get(node_id.unwrap()).unwrap().get_child(c);
            match node_id{
                None => return None,
                Some(n) => {
                    if i==q_string.len()-1{
                        return Some(n);
                    }
                    i +=1;
                    c = &q_string[i];
                    let mut j = 1;
                    while i < q_string.len() && j < self.get_node(n)?.edge_length(&0){
                        if c != &self.get_string(self.get_node(n)?.get_string_id()?)?[self.get_node(n)?.get_start() + j]{
                            return None;
                        }
                        if i==q_string.len()-1{
                            return Some(n);
                        }
                        i += 1;
                        j += 1;
                        c = &q_string[i];
                    }
                },
            }
        }
    }

    pub fn find(&self, s:&Vec<T>) -> Vec<(&TreeItem<T, U>, Vec<usize>)>{
        let node = self.get_pattern_node(s);
        let mut leaves:Vec<usize> = Vec::new();
        match node{
            None => Vec::new(),
            Some(i) => {
                self.leaves_of_node(&i, &mut leaves);
                let mut ids_and_indexes: Vec<(&TreeItem<T, U>, Vec<usize>)> = Vec::new();
                for leaf in leaves{
                    for (treeitem_id, idx) in self.get_node(&leaf).unwrap().get_data(){
                        ids_and_indexes.push((&self.get_treeitem(&treeitem_id).unwrap().0, idx.into_iter().map(|start| start.clone()).collect()));
                    }
                }
                ids_and_indexes
            }
        }
    }

    fn add_suffix_link(&mut self, node_id: &usize, need_suffix_link: &mut Option<usize>){
        match need_suffix_link{
            None => (),
            Some(i) => self.get_node_mut(&i).unwrap().set_suffix_link(node_id.clone()),
        };
        *need_suffix_link = Some(node_id.clone())
    }

    fn walk_down(&mut self, next_node_id:&usize, string:&Vec<T>, leaf_end:&usize, active_point: &mut ActivePoint<T>)->bool{
        let edge_length = self.get_node(next_node_id).unwrap().edge_length(leaf_end);
        if active_point.active_length.clone() >= edge_length{
            (*active_point).active_length -= edge_length;
            (*active_point).active_edge_index += edge_length;
            (*active_point).active_edge = Some(string[active_point.active_edge_index].clone());
            (*active_point).active_node = next_node_id.clone();
            return true;
        }
        false
    }

    pub fn add_string(&mut self, mut seq: Vec<T>, seq_id: U, max_depth: Option<usize>){
        seq.push(self.terminal_character.clone());

        let new_string: TreeItem<T, U> = TreeItem::new(seq_id, seq.clone());
        let new_string_id: usize = self.strings.len();
        self.strings.insert(new_string_id.clone(), (new_string, max_depth.clone()));

        let string: &Vec<T> = &seq;
        let string_len: usize = seq.len()-1;
        let mut curr_pos: usize = 0;
        let mut start_idx: usize = 0;
        let mut terminal_er3: bool = false;
        let mut need_suffix_link: Option<usize>;
        let mut remainder: usize = 0;
        let mut active_point: ActivePoint<T> = ActivePoint::new();
        let mut string_leaves: Vec<usize> = Vec::new();
        while curr_pos <= string_len {
            // dbg!(self.get_root());
            need_suffix_link = None;
            remainder += 1;
            while remainder > 0{
                if active_point.active_length == 0{
                    active_point.active_edge_index = curr_pos;
                    active_point.active_edge = Some(string[curr_pos].clone());
                }
                let next_node = self.get_node(&active_point.active_node).unwrap().get_child(active_point.active_edge.as_ref().unwrap()).cloned();
                match next_node{
                    None => {
                        let new_node: Node<T> = Node{
                            children: HashMap::new(),
                            suffix_link: None,
                            string_id: Some(new_string_id.clone()),
                            data: HashMap::from([(new_string_id.clone(), vec![(start_idx.clone())])]),
                            parent: Some(active_point.active_node.clone()),
                            end: None,
                            start: start_idx.clone(),
                        };
                        let new_node_id = self.nodes.len();
                        self.nodes.insert(new_node_id.clone(), new_node);
                        self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                        self.get_node_mut(&active_point.active_node).unwrap().set_child(active_point.active_edge.clone().unwrap(), new_node_id);
                        string_leaves.push(new_node_id.clone());
                        start_idx += 1;
                    },
                    Some(next_node_id) => {
                        let walk_down = self.walk_down(&next_node_id, string, &curr_pos, &mut active_point);
                        if walk_down{
                            continue;
                        }
                        else if self.get_string(self.get_node(&next_node_id).unwrap().get_string_id().unwrap()).unwrap()[self.get_node(&next_node_id).unwrap().get_start() + active_point.active_length] == string[curr_pos]{
                            if string[curr_pos] == self.terminal_character{
                                self.get_node_mut(&next_node_id).unwrap().add_seq(new_string_id.clone(), start_idx.clone());
                                start_idx += 1;
                                if !terminal_er3{
                                    self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                                    terminal_er3 = true;
                                }
                            }
                            else{
                                active_point.active_length += 1;
                                self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                                break;
                            }
                        }
                        else{
                            let leaf_node: Node<T> = Node{
                                                        children: HashMap::new(),
                                                        suffix_link: None,
                                                        parent:None,
                                                        data: HashMap::from([(new_string_id.clone(), vec![(start_idx.clone())])]),
                                                        string_id: Some(new_string_id.clone()),
                                                        end: None,
                                                        start: curr_pos.clone(),
                                                    };
                            let leaf_node_id = self.nodes.len();
                            self.nodes.insert(leaf_node_id.clone(), leaf_node);
                            string_leaves.push(leaf_node_id);
                            let split_node:Node<T> = Node{
                                children: HashMap::from([
                                    (string[curr_pos].clone(), leaf_node_id.clone()),
                                    (self.get_string(self.get_node(&next_node_id).unwrap().get_string_id().unwrap()).unwrap()[self.get_node(&next_node_id).unwrap().get_start().clone() + active_point.active_length].clone(), next_node_id.clone())
                                    ]),
                                suffix_link: None,
                                string_id: self.get_node(&next_node_id).unwrap().get_string_id().cloned(),
                                data: HashMap::from([(self.get_node(&next_node_id).unwrap().get_string_id().cloned().unwrap(), vec![(start_idx.clone())])]),
                                parent: Some(active_point.active_node.clone()),
                                end: Some(self.get_node(&next_node_id).unwrap().get_start().clone() + active_point.active_length - 1),
                                start: self.get_node(&next_node_id).unwrap().get_start().clone(),
                            };
                            let split_node_id = self.nodes.len();
                            self.nodes.insert(split_node_id.clone(), split_node);
                            self.get_node_mut(&active_point.active_node).unwrap().set_child(active_point.active_edge.clone().unwrap(), split_node_id);
                            start_idx += 1;
                            let tmp_start = self.get_node(&next_node_id).unwrap().get_start() + active_point.active_length;
                            self.get_node_mut(&next_node_id).unwrap().set_start(tmp_start);
                            self.add_suffix_link(&split_node_id, &mut need_suffix_link);
                        }
                    },
                };
                if active_point.active_node == self.root && active_point.active_length > 0{
                    active_point.active_edge_index += 1;
                    active_point.active_edge = Some(string[active_point.active_edge_index].clone());
                    active_point.active_length -= 1;
                }
                else if active_point.active_node != self.root{
                    active_point.active_node = self.get_node(&active_point.active_node).unwrap().get_suffix_link().unwrap();
                }
                remainder -= 1
            }
            curr_pos +=1;
        }

        for leaf in string_leaves.iter(){
            self.get_node_mut(leaf).unwrap().set_end(string.len() - 1);
        }
        string_leaves.clear();
        
    }

    pub fn contains(&self, string_id: &U)->bool{
        let string_ids: HashSet<&U> = self.strings.values().map(|x| x.0.get_id()).collect();
        string_ids.contains(string_id)
    }

    pub fn print_tree(&self){
        todo!()
    }
    
}




// //     pub fn to_newick(&self){
// //         let _newick_string: Vec<String> = Vec::new();
// //     }

        
// // }

// // // impl<'a, T, U> Serialize for KGST<T, U> 
// // // where
// // //     T: std::cmp::Eq + std::hash::Hash + Clone + std::marker::Copy + Serialize, 
// // //     U: std::cmp::Eq + std::hash::Hash + Clone + Serialize
// // // {
// // //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// // //     where
// // //         S: Serializer,
// // //     {
// // //         let mut state = serializer.serialize_struct("KGST", 6)?;
// // //         state.serialize_field("num_nodes", &self.num_nodes)?;
// // //         state.serialize_field("nodes", &self.nodes)?;
// // //         state.serialize_field("terminal_character", &self.terminal_character)?;
// // //         state.serialize_field("_strings", &self._strings)?;
// // //         state.serialize_field("_start_idx", &self._start_idx)?;
// // //         let new_main_strings: HashMap<U, Vec<T>> = self._main_strings.clone().into_iter().map(|(key, value)| (key, value.to_vec())).collect();
// // //         state.serialize_field("_main_strings", &new_main_strings)?;
// // //         state.end()
// // //     }
// // // }
