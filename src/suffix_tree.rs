use crate::suffix_node::Node;
use std::collections::HashMap;
use std::option::Option;

// pub mod suffix_node;

#[derive(Debug)]
pub struct KGST<T, U>{
    num_nodes: i32,
    nodes: HashMap<i32, Node<T, U>>,
    _root: i32,
    _active_node: i32,
    _active_edge: Option<T>,
    _active_edge_index: i32,
    _active_length: i32,
    _remainder: i32,
    _need_suffix_link: Option<i32>,
    _string_leaves: Vec<i32>,
    _terminal_character: T,
    _terminal_er3: bool,
    _strings: HashMap<U, Vec<T>>,
    _start_idx: i32,
    leaves: Vec<i32>,
    depth: usize,
}


impl<T, U> KGST<T, U> where T: std::cmp::Eq + std::hash::Hash + Clone + std::marker::Copy, U:std::cmp::Eq + std::hash::Hash + Clone{
    pub fn new(terminal_character: T)->KGST<T, U>{
        KGST{
            num_nodes: 1,
            nodes: HashMap::from([
                (0, Node::new(-1, Some(-1))),
            ]),
            _root: 0,
            _active_node: 0,
            _active_edge: None,
            _active_edge_index: 0,
            _active_length: 0,
            _remainder: 0,
            _need_suffix_link: None,
            _string_leaves: Vec::new(),
            _terminal_character: terminal_character,
            _terminal_er3: false,
            _strings: HashMap::new(),
            _start_idx: 0,
            leaves: Vec::new(),
            depth: 0,
        }
    }

    pub fn clear(&mut self){
        self.num_nodes= 1;
        self.nodes= HashMap::from([(0, Node::new(-1, Some(-1))),]);
        self._root= 0;
        self._active_node= 0;
        self._active_edge= None;
        self._active_edge_index= 0;
        self._active_length= 0;
        self._remainder= 0;
        self._need_suffix_link= None;
        self._string_leaves= Vec::new();
        self._terminal_er3= false;
        self._strings= HashMap::new();
        self._start_idx= 0;
        self.leaves= Vec::new();
        self.depth = 0;
    }

    // pub fn save_tree(&self)

    pub fn add_string(&mut self, mut seq: Vec<T>, seq_id: U, max_depth: usize){
        seq.push(self._terminal_character);
        self._strings.insert(seq_id.clone(), seq.clone());
        let string = seq.clone();
        let string_len = seq.len()-1;
        let mut i = 0;
        self._start_idx = 0;
        self._terminal_er3 = false;
        self.depth = 0;
        while i <= string_len {
            let leaf_end = i as i32;
            self._need_suffix_link = None;
            self._remainder += 1;
            while self._remainder > 0{
                // println!("{}", self.depth);
                if self._active_length == 0{
                    self._active_edge_index = i as i32;
                    self._active_edge = Some(string[i]);
                }
                let next_node_id = self.nodes.get(&self._active_node).unwrap().get_child(self._active_edge);
                match next_node_id{
                    None => {
                        // println!("{}", self._active_node);
                        let mut new_node = Node::new(i.try_into().unwrap(), None);
                        new_node.add_seq(seq_id.clone(), i as i32);
                        new_node.set_string_id(seq_id.clone());
                        new_node.add_parent(self._active_node);
                        self.nodes.insert(self.num_nodes, new_node);
                        self.num_nodes+=1;
                        self.depth+=1;
                        self._string_leaves.push(self.num_nodes-1);
                        self._start_idx += 1;
                        self.nodes.get_mut(&self._active_node).unwrap().set_child(self._active_edge.unwrap(), self.num_nodes-1);
                        self._add_suffix_link(self._active_node);
                    },
                    Some(node_id) => {
                        // println!("{:?}", node_id);
                        // println!("{:?}", self.nodes);
                        if self._walk_down(node_id, string.clone(), leaf_end){
                            continue;
                        }
                        else if self._strings.get(&(*self.nodes.get(&node_id).unwrap()).get_string_id().unwrap()).unwrap()[(self.nodes.get(&node_id).unwrap().get_start() + self._active_length) as usize] == string[i]{
                            if string[i] == self._terminal_character as T{
                                self.nodes.get_mut(&node_id).unwrap().add_seq(seq_id.clone(), i as i32);
                                self._start_idx += 1;
                                if !self._terminal_er3{
                                    self._add_suffix_link(self._active_node);
                                    self._terminal_er3 = true;
                                }
                            }
                            else{
                                self._active_length += 1;
                                self._add_suffix_link(self._active_node);
                                break;
                            }
                        }
                        else{
                            let mut new_node:Node<T, U> = Node::new(self.nodes.get(&node_id).unwrap().get_start(), Some(self.nodes.get(&node_id).unwrap().get_start() + self._active_length - 1));
                            new_node.set_string_id(self.nodes.get(&node_id).unwrap().get_string_id().unwrap());
                            new_node.add_seq(self.nodes.get(&node_id).unwrap().get_string_id().unwrap(), i as i32);
                            new_node.add_parent(self._active_node);
                            self.nodes.insert(self.num_nodes, new_node);
                            self.num_nodes += 1;
                            self.nodes.get_mut(&self._active_node).unwrap().set_child(self._active_edge.unwrap(), self.num_nodes-1);
                            let mut new_node = Node::new(i as i32, None);
                            new_node.set_string_id(seq_id.clone());
                            new_node.add_seq(seq_id.clone(), i as i32);
                            self.nodes.insert(self.num_nodes, new_node);
                            self.num_nodes += 1;
                            self._string_leaves.push(self.num_nodes-1);
                            self._start_idx += 1;
                            self.nodes.get_mut(&(self.num_nodes-2)).unwrap().set_child(string[i], self.num_nodes-1);
                            let tmp_start = self.nodes.get(&node_id).unwrap().get_start() + self._active_length;
                            self.nodes.get_mut(&node_id).unwrap().set_start(tmp_start);
                            let tmp_char = self._strings.get(&(*self.nodes.get(&node_id).unwrap()).get_string_id().unwrap()).unwrap()[self.nodes.get(&node_id).unwrap().get_start() as usize];
                            self.nodes.get_mut(&(self.num_nodes-2)).unwrap().set_child(tmp_char, node_id);
                            self._add_suffix_link(self.num_nodes-2);
                        }
                    },
                };
                if self._active_node == self._root && self._active_length > 0{
                    self._active_edge_index += 1;
                    self._active_edge = Some(string[self._active_edge_index as usize]);
                    self._active_length -= 1;
                }
                else if self._active_node != self._root{
                    self._active_node = self.nodes.get(&self._active_node).unwrap().get_suffix_link().unwrap();
                }
                    
                self._remainder -= 1
            }
            i +=1;
        }

        for leaf in self._string_leaves.iter(){
            self.nodes.get_mut(leaf).unwrap().set_end((string.len() - 1) as i32);
        }     
        self._string_leaves.clear()
         
    }

    fn _add_suffix_link(&mut self, node_id: i32){
        match self._need_suffix_link{
            None => (),
            Some(i) => self.nodes.get_mut(&i).unwrap().set_suffix_link(node_id),
        };
        self._need_suffix_link = Some(node_id);
    }

    fn _walk_down(&mut self, next_node_id:i32, string:Vec<T>, leaf_end:i32)->bool{
        let edge_length = self.nodes.get(&next_node_id).unwrap().edge_length(leaf_end);
        if self._active_length >= edge_length{
            self._active_length -= edge_length;
            self.depth += edge_length as usize;
            self._active_edge_index += edge_length;
            self._active_edge = Some(string[self._active_edge_index as usize]);
            self._active_node = next_node_id;
            return true;
        }
        false
    }

    pub fn find(&mut self, s:Vec<T>) -> Vec<(&U, &i32)>{
        let node = self._find_node(s);
        self.leaves.clear();
        match node{
            None => Vec::new(),
            Some(i) => {
                self._leaves_of_node(i);
                let mut ids_and_indexes = Vec::new();
                for leaf in &self.leaves{
                    for (id, idx) in self.nodes.get(leaf).unwrap().get_data(){
                        ids_and_indexes.push((id, idx));
                    }
                }
                ids_and_indexes
            }
        }
    }

    fn _find_node(&mut self, q_string:Vec<T>)->Option<i32>{
        let mut node: Option<i32> = Some(self._root);
        let mut c: T = q_string[0];
        let mut i = 0;
        loop {
            node = self.nodes.get(&node.unwrap()).unwrap().get_child(Some(c));
            match node{
                None => return None,
                Some(n) => {
                    if i==q_string.len()-1{
                        return node;
                    }
                    i +=1;
                    c = q_string[i];
                    let mut j = 1;
                    while i < q_string.len() && j < self.nodes.get(&n).unwrap().edge_length(-1){
                        if c != self._strings.get(&(*self.nodes.get(&n).unwrap()).get_string_id().unwrap()).unwrap()[(self.nodes.get(&n).unwrap().get_start() + j) as usize]{
                            return None;
                        }
                        if i==q_string.len()-1{
                            return node;
                        }
                        i += 1;
                        j += 1;
                        c = q_string[i]
                    }
                },
            }
        }
        // None
    }

    fn _leaves_of_node(&mut self, node:i32){
        if !self.nodes.get(&node).unwrap().has_children(){
            self.leaves.push(node);
        }

        for child in self.nodes.get(&node).unwrap().get_children().values(){
            // println!("{}", child);
            self._leaves_of_node(*child);
        }   
    }

    // pub fn get_nodes(&self)->&HashMap<i32, Node>{
    //     &self.nodes
    // }

        
}