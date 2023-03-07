use crate::suffix_node::Node;
use crate::tree_item::TreeItem;
use std::collections::HashMap;
use std::option::Option;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct KGST<T, U>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
    U: std::cmp::Eq + std::hash::Hash + Clone
{
    num_nodes: usize,
    nodes: HashMap<usize, Node<T>>,
    _root: usize,
    _active_node: usize,
    _active_edge: Option<T>,
    _active_edge_index: usize,
    _active_length: usize,
    _remainder: usize,
    _need_suffix_link: Option<usize>,
    _string_leaves: Vec<usize>,
    _terminal_character: T,
    _terminal_er3: bool,
    _strings: HashMap<usize, TreeItem<T, U>>,
    _start_idx: isize,
    leaves: Vec<isize>,
    _main_strings: HashMap<U, Arc<[T]>>,
}


impl<'a, T, U> KGST<T, U> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone + std::marker::Copy, 
    U: std::cmp::Eq + std::hash::Hash + Clone 
{
    pub fn new(terminal_character: T)->KGST<T, U>{
        KGST{
            num_nodes: 1,
            nodes: HashMap::from([
                (0, Node::new(0, Some(0))),
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
            _main_strings: HashMap::new(),
        }
    }

    pub fn clear(&mut self){
        self.num_nodes= 1;
        self.nodes= HashMap::from([(0, Node::new(0, Some(0))),]);
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
    }

    pub fn add_string(&mut self, mut seq: Vec<T>, seq_id: U){
        seq.push(self._terminal_character);
        let string_ids_num: usize = self._strings.len() + 1;
        self._strings.insert(string_ids_num, TreeItem::new(seq.clone(), seq_id.clone()));
        let string = &seq;
        let string_len = seq.len()-1;
        let mut i = 0;
        self._start_idx = 0;
        self._terminal_er3 = false;
        while i <= string_len {
            let leaf_end = i as usize;
            self._need_suffix_link = None;
            self._remainder += 1;
            while self._remainder > 0{

                if self._active_length == 0{
                    self._active_edge_index = i as usize;
                    self._active_edge = Some(string[i]);
                }
                let next_node_id = self.nodes.get(&self._active_node).unwrap().get_child(self._active_edge);
                match next_node_id{
                    None => {

                        let mut new_node = Node::new(i.try_into().unwrap(), None);
                        new_node.add_seq(string_ids_num, self._start_idx as usize);
                        new_node.set_string_id(string_ids_num);
                        // new_node.add_parent(self._active_node);
                        self.nodes.insert(self.num_nodes, new_node);
                        self.num_nodes+=1;
                        self._string_leaves.push(self.num_nodes-1);
                        self._start_idx += 1;
                        self.nodes.get_mut(&self._active_node).unwrap().set_child(self._active_edge.unwrap(), self.num_nodes-1);
                        self._add_suffix_link(self._active_node);
                    },
                    Some(node_id) => {

                        if self._walk_down(node_id, string, leaf_end){
                            continue;
                        }
                        else if self._strings.get(&(*self.nodes.get(&node_id).unwrap()).get_string_id().unwrap()).unwrap().get_string()[(self.nodes.get(&node_id).unwrap().get_start() + self._active_length) as usize] == string[i]{
                            if string[i] == self._terminal_character as T{
                                self.nodes.get_mut(&node_id).unwrap().add_seq(string_ids_num, self._start_idx as usize);
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

                            let mut split_node:Node<T> = Node::new(self.nodes.get(&node_id).unwrap().get_start(), Some(self.nodes.get(&node_id).unwrap().get_start() + self._active_length - 1));
                            split_node.set_string_id(self.nodes.get(&node_id).unwrap().get_string_id().unwrap());
                            split_node.add_seq(self.nodes.get(&node_id).unwrap().get_string_id().unwrap(), self._start_idx as usize);
                            // split_node.add_parent(self._active_node);
                            self.nodes.insert(self.num_nodes, split_node);

                            self.num_nodes += 1;
                            self.nodes.get_mut(&self._active_node).unwrap().set_child(self._active_edge.unwrap(), self.num_nodes-1);
                            // self.nodes.get_mut(&(self.num_nodes-1)).unwrap().add_parent(self._active_node);

                            let mut leaf_node = Node::new(i as isize, None);
                            println!("{}", i);
                            leaf_node.set_string_id(string_ids_num);
                            leaf_node.add_seq(string_ids_num, self._start_idx as usize);
                            // leaf_node.add_parent(self.num_nodes-1);
                            self.nodes.insert(self.num_nodes, leaf_node);
                            println!("{}, {}", self.num_nodes, self.nodes.get(&self.num_nodes).unwrap().get_start());

                            self.num_nodes += 1;
                            self._string_leaves.push(self.num_nodes-1);
                            self._start_idx += 1;
                            self.nodes.get_mut(&(self.num_nodes-2)).unwrap().set_child(string[i], self.num_nodes-1);
                            let tmp_start = self.nodes.get(&node_id).unwrap().get_start() + self._active_length;
                            self.nodes.get_mut(&node_id).unwrap().set_start(tmp_start);
                            let tmp_char = self._strings.get(&(*self.nodes.get(&node_id).unwrap()).get_string_id().unwrap()).unwrap().get_string()[self.nodes.get(&node_id).unwrap().get_start() as usize];
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
            self.nodes.get_mut(leaf).unwrap().set_end((string.len() - 1) as usize);
        }     
        self._string_leaves.clear()
         
    }

    fn _add_suffix_link(&mut self, node_id: usize){
        match self._need_suffix_link{
            None => (),
            Some(i) => self.nodes.get_mut(&i).unwrap().set_suffix_link(node_id),
        };
        self._need_suffix_link = Some(node_id);
    }

    fn _walk_down(&mut self, next_node_id:usize, string:Vec<T>, leaf_end:usize)->bool{
        let edge_length = self.nodes.get(&next_node_id).unwrap().edge_length(leaf_end);
        if self._active_length >= edge_length{
            self._active_length -= edge_length;
            self._active_edge_index += edge_length;
            self._active_edge = Some(string[self._active_edge_index as usize]);
            self._active_node = next_node_id;
            return true;
        }
        false
    }

    pub fn find(&self, s:&Vec<T>) -> Vec<(&U, &usize)>{
        let node = self._find_node(s);
        let mut leaves:Vec<usize> = Vec::new();
        match node{
            None => Vec::new(),
            Some(i) => {
                self._leaves_of_node(i, &mut leaves);
                let mut ids_and_indexes = Vec::new();
                for leaf in &leaves{
                    for (id, idx) in self.nodes.get(leaf).unwrap().get_data(){
                        ids_and_indexes.push((self._strings.get(id).unwrap().get_id(), idx));
                    }
                }
                ids_and_indexes
            }
        }
    }

    fn _find_node(&self, q_string:Vec<T>)->Option<usize>{
        let mut node: Option<usize> = Some(self._root);
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
                    while i < q_string.len() && j < self.nodes.get(&n).unwrap().edge_length(0){
                        if c != self._strings.get(&(*self.nodes.get(&n).unwrap()).get_string_id().unwrap()).unwrap().get_string()[(self.nodes.get(&n).unwrap().get_start() + j) as usize]{
                            return None;
                        }
                        if i==q_string.len()-1{
                            return node;
                        }
                        i += 1;
                        j += 1;
                        c = q_string[i];
                    }
                },
            }
        }
    }

    fn _leaves_of_node(&self, node:usize, leaves:&mut Vec<usize>){
        if !self.nodes.get(&node).unwrap().has_children(){
            leaves.push(node);
        }

        for child in self.nodes.get(&node).unwrap().get_children().values(){
            self._leaves_of_node(*child, leaves);
        }   
    }

    pub fn get_string(&self, string_id: &U)->&Arc<[T]>{
        self._main_strings.get(string_id).unwrap()
    }

    pub fn get_strings(&self)->&HashMap<U, Arc<[T]>>{
        &self._main_strings
    }

    pub fn set_strings(&mut self, strings:HashMap<U, Arc<[T]>>){
        self._main_strings = strings;

    }

    pub fn contains_key(&self, string_id:&U)->bool{
        self._main_strings.contains_key(string_id)
    }

    pub fn to_newick(&self){
        let _newick_string: Vec<String> = Vec::new();
    }

        
}