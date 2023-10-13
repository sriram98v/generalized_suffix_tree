pub mod tree;

use crate::suffix_tree::tree::*;
use crate::suffix_node::node::*;
use crate::suffix_node::*;
use crate::tree_item::TreeItem;
use crate::iter::node_iter::*;
use crate::iter::edge_iter::*;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::cmp;
use std::option::Option;
use serde::ser::{Serialize, Serializer, SerializeStruct};
    

/// A Generalized Truncated Suffix Tree implemented with a variation of Ukkonen's Algorithm.  

#[derive(Debug)]
pub struct KGST<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    root: usize,
    nodes: HashMap<NodeID, Node<T>>,
    terminal_character: T,
    strings: HashMap<StringID, (TreeItem<T, U>, usize)>,
    leaves: Vec<NodeID>,
    suffix_links: HashMap<NodeID, NodeID>,
    node_data: HashMap<NodeID, HashMap<StringID, HashSet<usize>>>
}

impl<T, U> Serialize for KGST<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
    U: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("KGST", 7)?;
        state.serialize_field("root", &self.root)?;
        state.serialize_field("nodes", &self.nodes)?;
        state.serialize_field("terminal_character", &self.terminal_character)?;
        state.serialize_field("strings", &self.strings)?;
        state.serialize_field("leaves", &self.leaves)?;
        state.serialize_field("suffix_links", &self.suffix_links)?;
        state.serialize_field("node_data", &self.node_data)?;
        state.end()
    }
}


impl<T, U> KGST<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
    U: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
{
    /// Creates a new empty K-Truncated Generalized Suffix tree, with a constant end symbol. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// use generalized_suffix_tree::suffix_tree::KGST;
    /// 
    /// let tree: KGST<char, String> = KGST::new('$');
    /// ```
    pub fn new(terminal_character: T)->Self{
        Self {
            nodes: HashMap::from([(0, Node::new(
                HashMap::new(),
                None,
                None,
                0,
                0
            ))]),
            root: 0,
            terminal_character,
            strings: HashMap::new(),
            leaves: Vec::new(),
            suffix_links: HashMap::from([(0,0)]),
            node_data: HashMap::from([(0, HashMap::new())]),
        }
    }

    /// Empties the tree of all strings and nodes.
    pub fn clear(&mut self){
        self.root = 0;
        self.nodes = HashMap::from([(0, Node::new(
            HashMap::new(),
            None,
            None,
            0,
            0
        ))]);
        self.strings = HashMap::new();
        self.leaves = Vec::new();
        self.node_data = HashMap::new();
        self.suffix_links = HashMap::new();
    }

    fn leaves_of_node(&self, node_id:&NodeID, leaves:&mut Vec<NodeID>){
        if !self.get_node(node_id).has_children(){
            leaves.push(*node_id);
        }

        for child_node_id in self.get_node_children(node_id).values(){
            self.leaves_of_node(child_node_id, leaves);
        }   
    }

    pub fn num_nodes(&self)->usize{
        self.nodes.len()
    }

    /// Returns a Hashmap of all the strings present in the tree along with their respective tree depth.
    pub fn get_strings(&self)->&HashMap<StringID, (TreeItem<T, U>, usize)>{
        &self.strings
    }

    pub fn get_nodes(&self)->&HashMap<NodeID, Node<T>>{
        &self.nodes
    }

    /// Retrieves a node from the tree by node id
    pub fn get_node(&self, node_id: &NodeID)->&Node<T>{
        self.nodes.get(node_id).expect("Node ID does not exist!")
    }

    /// Returns the string represented by the incoming edge of the node.
    pub fn get_node_label(&self, node_id: &NodeID)->&[T]{
        return &self.get_node_string(node_id)[*self.get_node_start(node_id)..self.get_node_start(node_id)+(self.get_node_edge_length(node_id))]
    }

    fn create_node(&mut self, children: HashMap<T, usize>,
            string_id: Option<usize>,
            data: HashMap<StringID, HashSet<usize>>,
            parent: Option<usize>,
            edge_length: usize,
            start: usize) -> usize{
                let node_id: usize = self.nodes.len();
                let node: Node<T> = Node::new(
                    children,
                    string_id,
                    parent,
                    edge_length,
                    start
                );
                self.suffix_links.insert(node_id, 0);
                self.nodes.insert(node_id, node);
                self.node_data.insert(node_id, data);
                node_id
            }

    fn set_node_suffix_link(&mut self, node_id: &NodeID, suffix_link_node_id: &NodeID){
        self.suffix_links.entry(*node_id).and_modify(|e| *e *= suffix_link_node_id).or_insert(*suffix_link_node_id);
    }

    fn get_string_by_treeitem_id(&self, treeitem_id: &StringID)->&Vec<T>{
        self.strings.get(treeitem_id).expect("TreeItem ID does not exist!").0.get_string()
    }

    fn get_node_edge_length(&self, node_id: &NodeID)->usize{
        self.get_node(node_id).get_edge_length()
    }

    fn get_node_string(&self, node_id: &NodeID)->&Vec<T>{
        self.get_string_by_treeitem_id(self.get_node_string_id(node_id))
    }

    fn get_node_string_id(&self, node_id: &NodeID)->&usize{
        self.get_node(node_id).get_string_id().expect("Node ID is root node")
    }

    fn get_node_mut(&mut self, node_id: &NodeID)->&mut Node<T>{
        self.nodes.get_mut(node_id).expect("Node ID does not exist!")
    }

    fn add_seq_to_leaves(&mut self, node_id: &NodeID, string_id: &StringID, start: &usize){
        let mut leaves:Vec<NodeID> = vec![];
        self.leaves_of_node(node_id, &mut leaves);
        for leaf in leaves.iter(){
            self.add_seq_to_node(leaf, string_id, start);
        }
    }

    fn get_treeitem_by_treeitem_id(&self, treeitem_id: &StringID)->&(TreeItem<T, U>, usize){
        self.strings.get(treeitem_id).expect("TreeItem ID does not exist!")
    }

    fn get_pattern_node(&self, q_string:&[T])->Option<&NodeID>{
        let mut node_id: Option<&NodeID> = Some(&self.root);
        let mut c: &T = &q_string[0];
        let mut i = 0;
        loop {
            node_id = self.get_node_child(node_id.unwrap(), c);
            match node_id{
                None => return None,
                Some(n) => {
                    if q_string.len()>self.get_node_depth(n){
                        i += self.get_node_edge_length(n);
                        c = &q_string[i];
                        node_id = Some(n);
                    }
                    else{
                        return Some(n);
                    }
                },
            }
        }
    }

    /// Retrieves all strings that the input slice is a suffix of.
    pub fn suffix_match(&self, s:&[T])-> HashMap<U, HashSet<usize>>{
        let mut query_string: Vec<T> = s.to_vec();
        query_string.push(self.terminal_character.clone());
        self.substring_match(&query_string)
    }

    /// Retrieves all strings that contain the input slice as some substring.
    pub fn substring_match(&self, s:&[T]) -> HashMap<U, HashSet<usize>>{
        let node = self.get_pattern_node(s);
        let mut leaves:Vec<usize> = vec![];
        let mut ids_and_indexes: HashMap<StringID, HashSet<usize>> = HashMap::new();
        match node{
            None => {},
            Some(i) => {
                if self.get_node_depth(i)<s.len(){
                    match self.get_node_parent(i){
                        None => {},
                        Some(_parent_id) => {self.leaves_of_node(i, &mut leaves);}
                    }
                }
                else{
                    self.leaves_of_node(i, &mut leaves);
                }
            }
        }
        for leaf in leaves{
            for (treeitem_id, idx) in self.get_node_data(&leaf){
                match ids_and_indexes.get_mut(treeitem_id){
                    None => {
                        if self.get_treeitem_by_treeitem_id(treeitem_id).1>=s.len(){
                            ids_and_indexes.insert(*treeitem_id, idx.clone());
                        }
                    },
                    Some(idxs) => {
                        if self.get_treeitem_by_treeitem_id(treeitem_id).1>=s.len(){
                            for i in idx.iter(){
                                idxs.insert(*i);
                            }
                        }
                    },
                }
            }
        }
        ids_and_indexes.into_iter().map(|(k, v)| (self.strings.get(&k).cloned().unwrap().0.get_id().clone(), v)).collect::<HashMap<U, HashSet<usize>>>()
    }

    fn get_node_children(&self, node_id: &NodeID)-> &HashMap<T, usize>{
        self.get_node(node_id).get_children()
    }

    fn set_node_child_id(&mut self, edge_label: &T, parent_node_id: &NodeID, child_node_id: &NodeID){
        self.get_node_mut(parent_node_id).set_child(edge_label.clone(), *child_node_id)
    }

    fn add_seq_to_node(&mut self, node_id: &NodeID , seq_id: &StringID, start: &usize){
        self.node_data.entry(*node_id).or_default().entry(*seq_id).or_default().insert(*start);
    }

    fn add_data_to_node(&mut self, node_id: &NodeID, data: HashMap<StringID, HashSet<usize>>){
        for (seq_id, starts) in data.iter(){
            for start in starts.iter(){
                self.add_seq_to_node(node_id, seq_id, start);
            }
        }
    }

    fn get_node_data(&self, node_id: &NodeID)->&HashMap<StringID, HashSet<usize>>{
        self.node_data.get(node_id).expect("Node ID does not exist!")
    }

    fn set_node_parent_id(&mut self, node_id: &NodeID, parent_id: &NodeID){
        self.get_node_mut(node_id).set_parent(*parent_id)
    }

    fn node_depth(&self, node_id: &NodeID, depth: usize)->usize{
        match self.get_node(node_id).get_parent(){
            None => depth,
            Some(i) => self.node_depth(i, depth+self.get_node_edge_length(node_id))
        }
    }

    fn get_node_start(&self, node_id: &NodeID)->&usize{
        self.get_node(node_id).get_start()
    }

    fn set_node_start(&mut self, node_id: &NodeID, start: usize){
        self.get_node_mut(node_id).set_start(start)
    }

    fn add_suffix_link(&mut self, node_id: &NodeID, need_suffix_link: &mut Option<usize>){
        match need_suffix_link{
            None => (),
            Some(i) => self.set_node_suffix_link(i, node_id),
        };
        *need_suffix_link = Some(*node_id)
    }

    /// inserts all suffixes of a string into the tree. If max_depth>0, all substrings of length==max_depth are inserted. 
    pub fn insert(&mut self, k: U, v: Vec<T>, max_depth: &usize){
        let seq_id: U = k.clone();
        let mut seq: Vec<T> = v.clone();

        seq.push(self.terminal_character.clone());

        let max_depth: usize = match max_depth {
            &0 => seq.len(),
            _ => cmp::min(*max_depth, seq.len()),
        };
        
        let new_string: TreeItem<T, U> = TreeItem::new(seq_id, seq.clone());
        let new_string_id: StringID = self.strings.len();
        self.strings.insert(new_string_id, (new_string, max_depth));

        let mut curr_pos: usize = 0;
        let mut start_idx: usize = 0;
        let mut need_suffix_link: Option<NodeID>;
        let mut remainder: usize = 0;
        let mut active_node: NodeID = 0;
        while curr_pos < seq.len() {
            need_suffix_link = None;
            remainder += 1;
            while remainder > 0{
                let active_edge = &seq[start_idx+self.get_node_depth(&active_node)].clone();
                let next_node = self.get_node(&active_node).get_child(active_edge).cloned();
                match next_node{
                    None => {
                        let new_leaf_node_id: usize = self.create_node(
                            HashMap::new(),
                            Some(new_string_id),
                            HashMap::from([(new_string_id, HashSet::from([start_idx]))]),
                            Some(active_node),
                            cmp::min(seq.len()-curr_pos,max_depth-self.get_node_depth(&active_node)),
                            curr_pos,
                        );
                        self.set_node_child_id(active_edge, &active_node, &new_leaf_node_id);
                        self.add_suffix_link(&active_node, &mut need_suffix_link);
                        let active_node_data = self.get_node_data(&active_node).clone();
                        self.add_data_to_node(&new_leaf_node_id, active_node_data);
                        start_idx += 1;
                    },
                    Some(next_node_id) => {
                        if self.get_node_edge_length(&next_node_id)<=curr_pos-start_idx-self.get_node_depth(&active_node){
                            // Walk down to next node (skip count trick)
                            active_node = next_node_id;
                            continue;
                        }
                        else if self.get_node_string(&next_node_id)[self.get_node_start(&next_node_id) + curr_pos-start_idx-self.get_node_depth(&active_node)] == seq[curr_pos]{   
                            self.add_seq_to_leaves(&next_node_id, &new_string_id, &start_idx);
                            if curr_pos==seq.len()-1{
                                start_idx+=1;
                            }
                            else{
                                self.add_suffix_link(&active_node, &mut need_suffix_link);
                                break;    
                            }
                        }
                        else{
                            let split_node_id: usize = self.create_node(
                                HashMap::from([
                                                (self.get_node_string(&next_node_id)[self.get_node_start(&next_node_id) + curr_pos-start_idx-self.get_node_depth(&active_node)].clone(), next_node_id)
                                                ]),
                                            Some(*self.get_node_string_id(&next_node_id)),
                                            HashMap::from([(new_string_id, HashSet::from([start_idx]))]),
                                            Some(active_node),
                                            curr_pos-start_idx-self.get_node_depth(&active_node),
                                            *self.get_node_start(&next_node_id),
                            );
                            self.set_node_child_id(active_edge, &active_node, &split_node_id);
                            let next_node_new_start = self.get_node_start(&next_node_id) + curr_pos-start_idx-self.get_node_depth(&active_node);
                            self.set_node_start(&next_node_id, next_node_new_start);
                            self.set_node_parent_id(&next_node_id, &split_node_id);
                            let leaf_node_id: usize = self.create_node(
                                HashMap::new(),
                                Some(new_string_id),
                                HashMap::from([(new_string_id, HashSet::from([start_idx]))]),
                                Some(split_node_id),
                                cmp::min(seq.len()-curr_pos, max_depth-self.get_node_depth(&split_node_id)),
                                curr_pos,
                            );
                            self.set_node_child_id(&seq[curr_pos], &split_node_id, &leaf_node_id);
                            self.add_suffix_link(&split_node_id, &mut need_suffix_link);
                            start_idx += 1;
                        }
                    },
                };
                if active_node != self.root{
                    active_node = *self.get_suffix_link(&active_node);
                }
                remainder -= 1
            }
            curr_pos +=1;
        }
        
    }

    //Checks if a string with string_id already exists in tree.
    pub fn contains(&self, string_id: &U)->bool{
        let string_ids: HashSet<&U> = self.strings.values().map(|x| x.0.get_id()).collect();
        string_ids.contains(string_id)
    }

    /// Prints tree as a string.
    pub fn print_tree(&self){
        todo!()
    }

    /// Returns a preorder node iterator of the tree
    pub fn iter_nodes_pre(&self)->PreOrdNodes<T>{
        PreOrdNodes::new(&self.root, &self.nodes)
    }
    
    /// Returns a preorder node iterator of the tree
    pub fn iter_nodes_post(&self)->PostOrdNodes<T>{
        PostOrdNodes::new(&self.root, &self.nodes)
    }

    /// Returns a postorder edge iterator of the tree
    pub fn iter_edges_post(&self)->PostOrdEdges<T>{
        PostOrdEdges::new(&self.root, &self.nodes, self.suffix_links.clone(), self.nodes.iter()
        .map(|(k, v)| (*k, *v.get_parent().unwrap_or(&0)))
        .collect()
        )

    }
}

impl<T, U> SuffixTree<T> for KGST<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
    U: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
{
    fn root(&self)->&NodeID{
        &self.root
    }

    fn is_leaf(&self, node_id: &NodeID)->bool{
        self.get_node(node_id).is_leaf()
    }

    fn get_node_child(&self, node_id: &NodeID, edge_label: &T)->Option<&NodeID>{
        self.get_node(node_id).get_child(edge_label)
    }
    fn get_node_parent(&self, node_id: &NodeID)->Option<&NodeID>{
        self.get_node(node_id).get_parent()
    }
    fn get_node_depth(&self, node_id: &NodeID)->usize{
        self.node_depth(node_id, 0)
    }
    fn get_suffix_link(&self, node_id: &NodeID) -> &usize{
        self.suffix_links.get(node_id).expect("Node id does not exist!")
    }
    fn get_node_label(&self, _node_id: &NodeID)->&[T]{
        todo!();
    }
    fn get_node_path_label(&self, _node_id: &NodeID)->&[T]{
        todo!();
    }
    fn get_node_path(&self, _node_id: &NodeID)->LinkedList<NodeID>{
        todo!();
    }

    fn is_suffix(&self, s:&[T])->bool{
        let mut query_string: Vec<T> = s.to_vec();
        query_string.push(self.terminal_character.clone());
        self.get_pattern_node(&query_string).is_some()
    }
}