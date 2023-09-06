use crate::suffix_node::Node;
use crate::tree_item::TreeItem;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::rc::Rc;
use std::option::Option;


pub struct KGST<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    // num_nodes: usize,
    root: Rc<RefCell<Node<T, U>>>,
    // active_node: &Rc<Node<T, U>>,
    // active_edge: Option<T>,
    // active_edge_index: usize,
    // active_length: usize,
    // remainder: usize,
    // need_suffix_link: Option<Rc<Node<T, U>>>,
    // string_leaves: Vec<Rc<Node<T, U>>>,
    terminal_character: T,
    // terminal_er3: bool,
    strings: HashSet<Rc<TreeItem<T, U>>>,
    start_idx: usize,
    leaves: Vec<Rc<RefCell<Node<T, U>>>>,
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
            root: Rc::new(RefCell::new(Node::new(0, Some(0)))),
            terminal_character: terminal_character,
            strings: HashSet::new(),
            start_idx: 0,
            leaves: Vec::new(),
        }
    }

    pub fn clear(&mut self){
        // self.num_nodes = 1;
        self.root = Rc::new(RefCell::new(Node::new(0, Some(0))));
        self.strings = HashSet::new();
        self.start_idx = 0;
        self.leaves = Vec::new();
    }

    fn leaves_of_node(&self, node:&Rc<RefCell<Node<T, U>>>, leaves:&mut Vec<Rc<RefCell<Node<T, U>>>>){
        if node.borrow().has_children(){
            leaves.push(node.clone());
        }

        for child in node.borrow().get_children().values(){
            self.leaves_of_node(child, leaves);
        }   
    }

    fn get_node(&self, q_string:&Vec<T>)->Option<Rc<RefCell<Node<T, U>>>>{
        let node: Option<&Rc<RefCell<Node<T, U>>>> = Some(&self.root);
        let mut c: &T = &q_string[0];
        let mut i = 0;
        loop {
            match node.unwrap().borrow().get_child(c){
                None => return None,
                Some(n) => {
                    if i==q_string.len()-1{
                        return Some(n.clone());
                    }
                    i +=1;
                    c = &q_string[i];
                    let mut j = 1;
                    while i < q_string.len() && j < n.borrow().edge_length(0){
                        if c != &n.borrow().get_string_id().unwrap().get_string()[(n.borrow().get_start() + j) as usize]{
                            return None;
                        }
                        if i==q_string.len()-1{
                            return Some(n.clone());
                        }
                        i += 1;
                        j += 1;
                        c = &q_string[i];
                    }
                },
            }
        }
    }

    pub fn find(&self, s:&Vec<T>) -> Vec<(Rc<TreeItem<T, U>>, Vec<usize>)>{
        let node = self.get_node(s);
        let mut leaves:Vec<Rc<RefCell<Node<T, U>>>> = Vec::new();
        match node{
            None => Vec::new(),
            Some(i) => {
                self.leaves_of_node(&i, &mut leaves);
                let mut ids_and_indexes: Vec<(Rc<TreeItem<T, U>>, Vec<usize>)> = Vec::new();
                for leaf in leaves{
                    for (treeitem, idx) in leaf.borrow().get_data(){
                        ids_and_indexes.push((treeitem.clone(), idx.into_iter().map(|(start, _end)| start.clone()).collect()));
                    }
                }
                ids_and_indexes
            }
        }
    }

    pub fn get_strings(&self)->&HashSet<Rc<TreeItem<T, U>>>{
        &self.strings
    }

    fn add_suffix_link(&self, node: Rc<RefCell<Node<T, U>>>, need_suffix_link: Option<Rc<RefCell<Node<T, U>>>>) -> Option<Rc<RefCell<Node<T, U>>>>{
        match need_suffix_link{
            None => (),
            Some(i) => i.clone().borrow_mut().set_suffix_link(node.clone()),
        };
        Some(node)
    }

    fn walk_down(next_node:Rc<RefCell<Node<T, U>>>, string:&Vec<T>, leaf_end:usize, mut active_length: usize, mut active_edge_index: usize, mut active_edge: Option<T>, mut active_node: Rc<RefCell<Node<T, U>>>)->(bool, usize, usize, Option<T>, Rc<RefCell<Node<T, U>>>){
        let edge_length = next_node.borrow().edge_length(leaf_end);
        if active_length >= edge_length{
            active_length -= edge_length;
            active_edge_index += edge_length;
            active_edge = Some(string[active_edge_index].clone());
            active_node = next_node;
            return (true, active_length, active_edge_index, active_edge, active_node);
        }
        (false, active_length, active_edge_index, active_edge, active_node)
    }


    pub fn add_string(&mut self, mut seq: Vec<T>, seq_id: U){
        seq.push(self.terminal_character.clone());
        // let string_ids_num: usize = self._strings.len() + 1;
        let new_string: Rc<TreeItem<T, U>> = Rc::new(TreeItem::new(seq_id, seq.clone()));
        self.strings.insert(new_string.clone());
        // self.strings.insert(string_ids_num, TreeItem::new(seq.clone().into(), seq_id.clone()));
        let string = &seq;
        let string_len = seq.len()-1;
        let mut i = 0;
        self.start_idx = 0;
        let mut terminal_er3 = false;
        let mut need_suffix_link: Option<Rc<RefCell<Node<T, U>>>>;
        let mut remainder: usize = 0;
        let mut active_length: usize = 0;
        let mut active_edge_index: usize = 0;
        let mut active_edge: Option<T> = None;
        let mut active_node: Rc<RefCell<Node<T, U>>> = self.root.clone();
        let mut string_leaves: Vec<Rc<RefCell<Node<T, U>>>> = Vec::new();
        while i <= string_len {
            let leaf_end = i;
            need_suffix_link = None;
            remainder += 1;
            while remainder > 0{

                if active_length == 0{
                    active_edge_index = i;
                    active_edge = Some(string[i].clone());
                }
                let next_node = active_node.borrow().get_child(active_edge.as_ref().unwrap()).clone();
                match next_node{
                    None => {
                        let new_node: Rc<RefCell<Node<T, U>>> = Rc::new(RefCell::new(Node::new(i.try_into().unwrap(), None)));
                        new_node.borrow_mut().add_seq(new_string.clone(), self.start_idx.clone());
                        new_node.borrow_mut().set_string_id(new_string.clone());
                        // new_node.add_parent(self._active_node);
                        // self.nodes.insert(self.num_nodes, new_node);
                        // self.num_nodes+=1;
                        string_leaves.push(new_node.clone());
                        self.start_idx += 1;
                        active_node.borrow_mut().set_child(active_edge.clone().unwrap(), new_node.clone());
                        need_suffix_link = self.add_suffix_link(active_node.clone(), need_suffix_link);
                    },
                    Some(node) => {
                        let walk_down = Self::walk_down(node.clone(), string, leaf_end, active_length, active_edge_index, active_edge, active_node);
                        (active_length, active_edge_index, active_edge, active_node) = (walk_down.1, walk_down.2, walk_down.3, walk_down.4);
                        if walk_down.0{
                            continue;
                        }
                        else if node.borrow().get_string_id().unwrap().get_string()[node.borrow().get_start() + active_length] == string[i]{
                            if string[i] == self.terminal_character{
                                node.borrow_mut().add_seq(new_string.clone(), self.start_idx.clone());
                                self.start_idx += 1;
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
                            print!("running here");
                            let split_node:Rc<RefCell<Node<T, U>>> = Rc::new(RefCell::new(Node::new(node.borrow().get_start().clone(), Some(node.borrow().get_start().clone() + active_length - 1))));
                            split_node.borrow_mut().set_string_id(node.borrow().get_string_id().unwrap());
                            split_node.borrow_mut().add_seq(node.borrow().get_string_id().unwrap(), self.start_idx.clone());
                            // self.nodes.insert(self.num_nodes, split_node);

                            // self.num_nodes += 1;
                            active_node.borrow_mut().set_child(active_edge.clone().unwrap(), split_node.clone());

                            let leaf_node = Rc::new(RefCell::new(Node::new(i, None)));
                            leaf_node.borrow_mut().set_string_id(new_string.clone());
                            leaf_node.borrow_mut().add_seq(new_string.clone(), self.start_idx.clone());
                            // self.nodes.insert(self.num_nodes, leaf_node);

                            // self.num_nodes += 1;
                            string_leaves.push(leaf_node.clone());
                            self.start_idx += 1;
                            split_node.borrow_mut().set_child(string[i].clone(), leaf_node);
                            let tmp_start = node.borrow().get_start() + active_length;
                            node.borrow_mut().set_start(tmp_start);
                            let tmp_char = node.borrow().get_string_id().unwrap().get_string()[node.borrow().get_start() + 0].clone();
                            split_node.borrow_mut().set_child(tmp_char, node.clone());
                            need_suffix_link = self.add_suffix_link(split_node, need_suffix_link);
                        }
                    },
                };
                if active_node.clone() == self.root && active_length > 0{
                    active_edge_index += 1;
                    active_edge = Some(string[active_edge_index].clone());
                    active_length -= 1;
                }
                else if active_node.clone() != self.root{
                    active_node = active_node.clone().borrow().get_suffix_link().unwrap();
                }
                    
                remainder -= 1
            }
            i +=1;
        }

        for leaf in string_leaves.iter(){
            leaf.borrow_mut().set_end(string.len() - 1);
        }     
        string_leaves.clear()
         
    }



    // pub fn contains(&self, string_id:&U)->bool{
    //     self.strings.contains(string_id)
    // }

    
}




//     pub fn to_newick(&self){
//         let _newick_string: Vec<String> = Vec::new();
//     }

        
// }

// // impl<'a, T, U> Serialize for KGST<T, U> 
// // where
// //     T: std::cmp::Eq + std::hash::Hash + Clone + std::marker::Copy + Serialize, 
// //     U: std::cmp::Eq + std::hash::Hash + Clone + Serialize
// // {
// //     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// //     where
// //         S: Serializer,
// //     {
// //         let mut state = serializer.serialize_struct("KGST", 6)?;
// //         state.serialize_field("num_nodes", &self.num_nodes)?;
// //         state.serialize_field("nodes", &self.nodes)?;
// //         state.serialize_field("terminal_character", &self.terminal_character)?;
// //         state.serialize_field("_strings", &self._strings)?;
// //         state.serialize_field("_start_idx", &self._start_idx)?;
// //         let new_main_strings: HashMap<U, Vec<T>> = self._main_strings.clone().into_iter().map(|(key, value)| (key, value.to_vec())).collect();
// //         state.serialize_field("_main_strings", &new_main_strings)?;
// //         state.end()
// //     }
// // }
