use crate::suffix_node::Node;
use crate::tree_item::TreeItem;
use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::option::Option;


#[derive(Debug)]
pub struct KGST<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    // num_nodes: usize,
    root: usize,
    nodes: HashMap<usize, Node<T>>,
    // active_node: &Rc<Node<T, U>>,
    // active_edge: Option<T>,
    // active_edge_index: usize,
    // active_length: usize,
    // remainder: usize,
    // need_suffix_link: Option<Rc<Node<T, U>>>,
    // string_leaves: Vec<Rc<Node<T, U>>>,
    terminal_character: T,
    // terminal_er3: bool,
    strings: HashMap<usize, TreeItem<T, U>>,
    // start_idx: usize,
    leaves: Vec<usize>,
    // _main_strings: HashMap<U, Vec<T>>,
}


impl<T, U> KGST<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub fn new(terminal_character: T)->KGST<T, U>{
        KGST{
            // num_nodes: 1,
            nodes: HashMap::from([(0, Node{
                children: HashMap::new(),
                suffix_link: None,
                string_id: None,
                data: HashMap::new(),
                parent: Some(0),
                end: None,
                start: 0
            })]),
            root: 0,
            terminal_character: terminal_character,
            strings: HashMap::new(),
            // start_idx: 0,
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
            parent: Some(0),
            end: None,
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

    pub fn get_strings(&self)->&HashMap<usize, TreeItem<T, U>>{
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
            Some(treeitem) => Some(treeitem.get_id())
        }
    }

    pub fn get_string(&self, treeitem_id: &usize)->Option<&Vec<T>>{
        match self.strings.get(treeitem_id){
            None => None,
            Some(treeitem) => Some(treeitem.get_string())
        }
    }

    pub fn get_treeitem(&self, treeitem_id: &usize)->Option<&TreeItem<T, U>>{
        self.strings.get(treeitem_id)
    }

    fn get_pattern_node(&self, q_string:&Vec<T>)->Option<&usize>{
        let node_id: &usize = &self.root;
        let mut c: &T = &q_string[0];
        let mut i = 0;
        loop {
            match self.nodes.get(node_id).unwrap().get_child(c){
                None => return None,
                Some(n) => {
                    if i==q_string.len()-1{
                        return Some(n);
                    }
                    i +=1;
                    c = &q_string[i];
                    let mut j = 1;
                    // println!("{}", n);
                    while i < q_string.len() && j < self.get_node(n)?.edge_length(0){
                        // println!("node_id: {}, edge_len:{}", n, self.get_node(n)?.edge_length(0));
                        if c != &self.get_string(self.get_node(n)?.get_string_id()?)?[self.nodes.get(n)?.get_start() + j]{
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
                        ids_and_indexes.push((self.get_treeitem(&treeitem_id).unwrap(), idx.into_iter().map(|(start, _end)| start.clone()).collect()));
                    }
                }
                ids_and_indexes
            }
        }
    }

    fn add_suffix_link(&mut self, node_id: usize, need_suffix_link: Option<usize>) -> Option<usize>{
        match need_suffix_link{
            None => (),
            Some(i) => self.get_node_mut(&i)?.set_suffix_link(node_id),
        };
        Some(node_id)
    }

    fn walk_down(&mut self, next_node_id:usize, string:&Vec<T>, leaf_end:usize, mut active_length: usize, mut active_edge_index: usize, mut active_edge: Option<T>, mut active_node: usize)->(bool, usize, usize, Option<T>, usize){
        let edge_length = self.get_node_mut(&next_node_id).unwrap().edge_length(leaf_end);
        if active_length >= edge_length{
            active_length -= edge_length;
            active_edge_index += edge_length;
            active_edge = Some(string[active_edge_index].clone());
            active_node = next_node_id.clone();
            return (true, active_length, active_edge_index, active_edge, active_node);
        }
        (false, active_length, active_edge_index, active_edge, active_node)
    }

    pub fn add_string(&mut self, mut seq: Vec<T>, seq_id: U){
        seq.push(self.terminal_character.clone());
        // let string_ids_num: usize = self._strings.len() + 1;
        let new_string: TreeItem<T, U> = TreeItem::new(seq_id, seq.clone());
        let new_string_id: usize = self.strings.len()+1;
        self.strings.insert(new_string_id.clone(), new_string);
        // self.strings.insert(string_ids_num, TreeItem::new(seq.clone().into(), seq_id.clone()));
        let string: &Vec<T> = &seq;
        let string_len: usize = seq.len()-1;
        let mut i: usize = 0;
        let mut start_idx: usize = 0;
        let mut terminal_er3: bool = false;
        let mut need_suffix_link: Option<usize>;
        let mut remainder: usize = 0;
        let mut active_length: usize = 0;
        let mut active_edge_index: usize = 0;
        let mut active_edge: Option<T> = None;
        let mut active_node: usize = self.root.clone();
        let mut string_leaves: Vec<usize> = Vec::new();
        while i <= string_len {
            let leaf_end = i;
            need_suffix_link = None;
            remainder += 1;
            while remainder > 0{

                if active_length == 0{
                    active_edge_index = i;
                    active_edge = Some(string[i].clone());
                }
                let next_node = self.get_node(&active_node).unwrap().get_child(active_edge.as_ref().unwrap()).cloned();
                match next_node{
                    None => {
                        let new_node: Node<T> = Node{
                            children: HashMap::new(),
                            suffix_link: None,
                            string_id: Some(new_string_id.clone()),
                            data: HashMap::from([(new_string_id.clone(), vec![(start_idx.clone(), None)])]),
                            parent: Some(active_node.clone()),
                            end: None,
                            start: start_idx.clone(),
                        };
                        let new_node_id = self.nodes.len();
                        self.nodes.insert(new_node_id.clone(), new_node);
                        self.get_node_mut(&active_node).unwrap().set_child(active_edge.clone().unwrap(), new_node_id);
                        string_leaves.push(new_node_id.clone());
                        start_idx += 1;
                        need_suffix_link = self.add_suffix_link(active_node.clone(), need_suffix_link);
                    },
                    Some(next_node_id) => {
                        let walk_down = self.walk_down(next_node_id.clone(), string, leaf_end, active_length, active_edge_index, active_edge, active_node);
                        (active_length, active_edge_index, active_edge, active_node) = (walk_down.1, walk_down.2, walk_down.3, walk_down.4);
                        if walk_down.0{
                            continue;
                        }
                        else if self.get_string(self.get_node(&next_node_id).unwrap().get_string_id().unwrap()).unwrap()[self.get_node(&next_node_id).unwrap().get_start() + active_length] == string[i]{
                            if string[i] == self.terminal_character{
                                self.get_node_mut(&next_node_id).unwrap().add_seq(new_string_id.clone(), start_idx.clone());
                                start_idx += 1;
                                if !terminal_er3{
                                    need_suffix_link = self.add_suffix_link(active_node.clone(), need_suffix_link);
                                    terminal_er3 = true;
                                }
                            }
                            else{
                                active_length += 1;
                                self.add_suffix_link(active_node.clone(), need_suffix_link);
                                break;
                            }
                        }
                        else{
                            let leaf_node: Node<T> = Node{
                                                        children: HashMap::new(),
                                                        suffix_link: None,
                                                        parent:None,
                                                        data: HashMap::from([(new_string_id.clone(), vec![(start_idx.clone(), None)])]),
                                                        string_id: Some(new_string_id.clone()),
                                                        end: None,
                                                        start: 0,
                                                    };
                            let leaf_node_id = self.nodes.len();
                            self.nodes.insert(leaf_node_id.clone(), leaf_node);
                            string_leaves.push(leaf_node_id);
                            let split_node:Node<T> = Node{
                                children: HashMap::from([
                                    (string[i].clone(), leaf_node_id.clone())
                                    ]),
                                suffix_link: None,
                                string_id: self.get_node(&next_node_id).unwrap().get_string_id().cloned(),
                                data: HashMap::from([(self.get_node(&next_node_id).unwrap().get_string_id().cloned().unwrap(), vec![(start_idx.clone(), None)])]),
                                parent: Some(active_node.clone()),
                                end: Some(self.get_node(&next_node_id).unwrap().get_start().clone() + active_length - 1),
                                start: self.get_node(&next_node_id).unwrap().get_start().clone(),
                            };
                            let split_node_id = self.nodes.len();
                            // println!("split_node: {}, start: {}, end: {:?}", split_node_id, &split_node.get_start(), &split_node.get_end(0));
                            self.nodes.insert(split_node_id.clone(), split_node);
                            self.get_node_mut(&active_node).unwrap().set_child(active_edge.clone().unwrap(), split_node_id);
                            start_idx += 1;
                            self.get_node_mut(&self.get_node(&split_node_id).unwrap().get_child(&string[i]).unwrap().clone()).unwrap().set_parent(split_node_id.clone());
                            let tmp_start = self.get_node(&next_node_id).unwrap().get_start() + active_length;
                            self.get_node_mut(&next_node_id).unwrap().set_start(tmp_start);
                            let tmp_char = self.get_string(self.get_node(&next_node_id).unwrap().get_string_id().unwrap()).unwrap()[self.get_node(&next_node_id).unwrap().get_start() + 0].clone();
                            self.get_node_mut(&split_node_id).unwrap().set_child(tmp_char, next_node_id.clone());
                            need_suffix_link = self.add_suffix_link(split_node_id, need_suffix_link);
                        }
                    },
                };
                if active_node.clone() == self.root && active_length > 0{
                    active_edge_index += 1;
                    active_edge = Some(string[active_edge_index].clone());
                    active_length -= 1;
                }
                else if active_node.clone() != self.root{
                    active_node = self.get_node(&active_node).unwrap().get_suffix_link().unwrap();
                }
                    
                remainder -= 1
            }
            i +=1;
        }

        for leaf in string_leaves.iter(){
            self.get_node_mut(leaf).unwrap().set_end(string.len() - 1);
        }
        // println!("{:?}", string_leaves);
        string_leaves.clear();
    }



    // pub fn contains(&self, string_id:&U)->bool{
    //     self.strings.contains(string_id)
    // }

    
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
