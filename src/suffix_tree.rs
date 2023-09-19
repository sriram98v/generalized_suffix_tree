use crate::suffix_node::Node;
use crate::tree_item::TreeItem;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::cmp;
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
    strings: HashMap<usize, (TreeItem<T, U>, usize)>,
    leaves: Vec<usize>,
}


impl<T, U> KGST<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub fn new(terminal_character: T)->KGST<T, U>{
        KGST{
            nodes: HashMap::from([(0, Node::new(
                HashMap::new(),
                None,
                None,
                HashMap::new(),
                None,
                Some(0),
                0
            ))]),
            root: 0,
            terminal_character: terminal_character,
            strings: HashMap::new(),
            leaves: Vec::new(),
        }
    }

    pub fn clear(&mut self){
        self.root = 0;
        self.nodes = HashMap::from([(0, Node::new(
            HashMap::new(),
            None,
            None,
            HashMap::new(),
            None,
            Some(0),
            0
        ))]);
        self.strings = HashMap::new();
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

    pub fn get_strings(&self)->&HashMap<usize, (TreeItem<T, U>, usize)>{
        &self.strings
    }

    pub fn get_nodes(&self)->&HashMap<usize, Node<T>>{
        &self.nodes
    }

    pub fn get_node(&self, node_id: &usize)->Option<&Node<T>>{
        self.nodes.get(node_id)
    }

    fn get_suffix_link(&self, node_id: &usize) -> Option<&usize>{
        match self.nodes.get(node_id){
            None => None,
            Some(node) => node.get_suffix_link(),
        }
    }

    fn set_suffix_link(&mut self, node_id: &usize, suffix_link_node_id: &usize){
        match self.get_node_mut(node_id){
            None => {},
            Some(node) => node.set_suffix_link(suffix_link_node_id.clone()),
        };
    }

    // fn get_string_id_by_treeitem_id(&self, treeitem_id: &usize)->Option<&U>{
    //     match self.strings.get(treeitem_id){
    //         None => None,
    //         Some(treeitem) => Some(treeitem.0.get_id())
    //     }
    // }

    fn get_string_by_treeitem_id(&self, treeitem_id: &usize)->Option<&Vec<T>>{
        match self.strings.get(treeitem_id){
            None => None,
            Some(treeitem) => Some(treeitem.0.get_string())
        }
    }

    // fn is_leaf(&self, node_id: &usize)->bool{
    //     (!self.get_node(node_id).unwrap().has_children()) && (self.get_node_parent_id(node_id)!=None)
    // }

    fn get_node_edge_length(&self, node_id: &usize, leaf_end:&usize)->usize{
        match self.get_node(node_id){
            None => leaf_end.clone(),
            Some(node) => node.get_edge_length(leaf_end)
        }
    }

    fn set_node_edge_length(&mut self, node_id: &usize, edge_length: usize){
        self.get_node_mut(node_id).unwrap().set_edge_length(edge_length)
    }

    pub fn get_node_string(&self, node_id: &usize)->Option<&Vec<T>>{
        match self.get_node_string_id(node_id){
            None => None,
            Some(i) => self.get_string_by_treeitem_id(i)
        }
    }

    pub fn get_node_string_id(&self, node_id: &usize)->Option<&usize>{
        self.get_node(node_id).unwrap().get_string_id()
    }

    fn get_node_mut(&mut self, node_id: &usize)->Option<&mut Node<T>>{
        self.nodes.get_mut(node_id)
    }

    pub fn get_root(&self)->&Node<T>{
        self.get_node(&0).unwrap()
    }

    fn get_treeitem_by_treeitem_id(&self, treeitem_id: &usize)->Option<&(TreeItem<T, U>, usize)>{
        self.strings.get(treeitem_id)
    }

    fn get_pattern_node(&self, q_string:&[T])->Option<&usize>{
        let mut node_id: Option<&usize> = Some(&self.root);
        let mut c: &T = &q_string[0];
        let mut i = 0;
        loop {
            node_id = self.get_node_child_id(node_id.unwrap(), c);
            match node_id{
                None => return None,
                Some(n) => {
                    if q_string.len()>self.get_node_depth(n){
                        i += self.get_node_edge_length(n, &0);
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

    pub fn is_suffix(&self, s:&[T])->bool{
        let mut query_string: Vec<T> = s.clone().to_vec();
        query_string.push(self.terminal_character.clone());
        match self.get_pattern_node(&query_string){
            None => false,
            Some(_i) => true
        }
    }

    pub fn find(&self, s:&[T]) -> HashMap<U, HashSet<usize>>{
        let node = self.get_pattern_node(s);
        let mut leaves:Vec<usize> = vec![];
        let mut ids_and_indexes: HashMap<usize, HashSet<usize>> = HashMap::new();
        match node{
            None => {},
            Some(i) => {
                if self.get_node_depth(i)<s.len(){
                    match self.get_node_parent_id(i){
                        None => {},
                        Some(_parent_id) => {self.leaves_of_node(&i, &mut leaves);}
                    }
                }
                else{
                    self.leaves_of_node(&i, &mut leaves);
                }
            }
        }
        for leaf in leaves{
            for (treeitem_id, idx) in self.get_node(&leaf).unwrap().get_data(){
                match ids_and_indexes.get_mut(treeitem_id){
                    None => {
                        if self.get_treeitem_by_treeitem_id(treeitem_id).unwrap().1>=s.len(){
                            ids_and_indexes.insert(treeitem_id.clone(), idx.clone());
                        }   
                    },
                    Some(idxs) => {
                        if self.get_treeitem_by_treeitem_id(treeitem_id).unwrap().1>=s.len(){
                            for i in idx.iter(){
                                idxs.insert(i.clone());
                            }
                        }
                    },
                }
            }
        }
        ids_and_indexes.into_iter().map(|(k, v)| (self.strings.get(&k).cloned().unwrap().0.get_id().clone(), v)).collect::<HashMap<U, HashSet<usize>>>()
    }

    pub fn get_node_children(&self, node_id: &usize)-> Option<Vec<usize>>{
        match self.get_node(node_id){
            None => None,
            Some(node) => Some(node.get_children().values().map(|x| x.clone()).collect())
        }
    }

    pub fn get_node_children_mut(&mut self, node_id: &usize)-> Option<Vec<&usize>>{
        match self.get_node(node_id){
            None => None,
            Some(node) => Some(node.get_children().values().collect())
        }
    }

    pub fn get_node_child_id(&self, node_id: &usize, edge_label: &T)->Option<&usize>{
        match self.get_node(node_id){
            None => None,
            Some(child_node) => child_node.get_child(edge_label)
        }
    }

    pub fn set_node_child_id(&mut self, edge_label: &T, parent_node_id: &usize, child_node_id: &usize){
        self.get_node_mut(parent_node_id).unwrap().set_child(edge_label.clone(), child_node_id.clone())
    }

    pub fn get_node_child(&self, node_id: &usize, edge_label: &T)->Option<&Node<T>>{
        match self.get_node(node_id){
            None => None,
            Some(child_node) => self.get_node(child_node.get_child(edge_label)?)
        }
    }

    pub fn get_node_parent(&self, node_id: &usize)->Option<&Node<T>>{
        match self.get_node(node_id).unwrap().get_parent(){
            None => None,
            Some(parent_id) => self.get_node(parent_id)
        }
    }

    pub fn get_node_parent_id(&self, node_id: &usize)->Option<&usize>{
        self.get_node(node_id).unwrap().get_parent()
    }

    pub fn set_node_parent_id(&mut self, node_id: &usize, parent_id: &usize){
        self.get_node_mut(node_id).unwrap().set_parent(parent_id.clone())
    }

    fn node_depth(&self, node_id: &usize, depth: usize)->usize{
        match self.get_node(node_id).unwrap().get_parent(){
            None => return depth,
            Some(i) => return self.node_depth(i, depth+self.get_node_edge_length(node_id, &0))
        };
    }

    fn get_node_depth(&self, node_id: &usize)->usize{
        self.node_depth(node_id, 0)
    }

    fn get_node_start(&self, node_id: &usize)->&usize{
        self.get_node(&node_id).unwrap().get_start()
    }

    fn set_node_start(&mut self, node_id: &usize, start: usize){
        self.get_node_mut(&node_id).unwrap().set_start(start)
    }

    fn add_suffix_link(&mut self, node_id: &usize, need_suffix_link: &mut Option<usize>){
        match need_suffix_link{
            None => (),
            Some(i) => self.set_suffix_link(i, node_id),
        };
        *need_suffix_link = Some(node_id.clone())
    }

    fn walk_down(&mut self, next_node_id:&usize, string:&Vec<T>, leaf_end:&usize, active_point: &mut ActivePoint<T>)->bool{
        let edge_length = self.get_node_edge_length(next_node_id, leaf_end);
        if active_point.active_length >= edge_length{
            (*active_point).active_length -= edge_length;
            (*active_point).active_edge_index += edge_length;
            (*active_point).active_edge = Some(string[active_point.active_edge_index].clone());
            (*active_point).active_node = next_node_id.clone();
            return true;
        }
        false
    }

    pub fn insert(&mut self, k: U, v: Vec<T>, max_depth: &usize){
        let seq_id: U = k.clone();
        let mut seq: Vec<T> = v.clone();

        seq.push(self.terminal_character.clone());

        let max_depth: usize = match max_depth {
            &0 => seq.len(),
            _ => cmp::min(max_depth.clone(), seq.len()),
        };
        
        let new_string: TreeItem<T, U> = TreeItem::new(seq_id, seq.clone());
        let new_string_id: usize = self.strings.len();
        self.strings.insert(new_string_id.clone(), (new_string, max_depth.clone()));

        let mut curr_pos: usize = 0;
        let mut start_idx: usize = 0;
        let mut need_suffix_link: Option<usize>;
        let mut remainder: usize = 0;
        let mut active_point: ActivePoint<T> = ActivePoint::new();
        let mut string_leaves: Vec<usize> = Vec::new();
        let mut terminal_er3: bool = false;
        while curr_pos <= seq.len()-1 {
            need_suffix_link = None;
            remainder += 1;
            while remainder > 0{
                if active_point.active_length == 0{
                    active_point.active_edge_index = curr_pos;
                    active_point.active_edge = Some(seq[curr_pos].clone());
                }
                let next_node = self.get_node(&active_point.active_node).unwrap().get_child(active_point.active_edge.as_ref().unwrap()).cloned();
                match next_node{
                    None => {
                        if self.get_node_depth(&active_point.active_node)<max_depth{
                            let new_leaf_node: Node<T> = Node::new(
                                HashMap::new(),
                                None,
                                Some(new_string_id.clone()),
                                HashMap::from([(new_string_id.clone(), HashSet::from([(start_idx.clone())]))]),
                                Some(active_point.active_node.clone()),
                                None,
                                curr_pos.clone(),
                            );
                            let new_leaf_node_id = self.nodes.len();
                            self.nodes.insert(new_leaf_node_id.clone(), new_leaf_node);
                            self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                            self.set_node_child_id(active_point.active_edge.as_ref().unwrap(), &active_point.active_node, &new_leaf_node_id);
                            string_leaves.push(new_leaf_node_id.clone());
                        }
                        start_idx += 1;
                    },
                    Some(next_node_id) => {
                        let walk_down = self.walk_down(&next_node_id, &seq, &curr_pos, &mut active_point);
                        if walk_down{
                            continue;
                        }
                        else if self.get_node_string(&next_node_id).unwrap()[self.get_node_start(&next_node_id) + active_point.active_length] == seq[curr_pos]{
                            if seq[curr_pos] == self.terminal_character{
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
                            let leaf_node: Node<T> = Node::new(
                                HashMap::new(),
                                None,
                                Some(new_string_id),
                                HashMap::from([(new_string_id, HashSet::from([(start_idx.clone())]))]),
                                Some(self.nodes.len()+1),
                                None,
                                curr_pos.clone(),
                            );
                            let leaf_node_id = self.nodes.len();
                            self.nodes.insert(leaf_node_id.clone(), leaf_node);
                            string_leaves.push(leaf_node_id);
                            let split_node:Node<T> = Node::new(
                                HashMap::from([
                                    (seq[curr_pos].clone(), leaf_node_id.clone()),
                                    (self.get_node_string(&next_node_id).unwrap()[self.get_node_start(&next_node_id) + active_point.active_length].clone(), next_node_id.clone())
                                    ]),
                                None,
                                self.get_node_string_id(&next_node_id).cloned(),
                                HashMap::from([(new_string_id, HashSet::from([(start_idx.clone())]))]),
                                Some(active_point.active_node.clone()),
                                Some(active_point.active_length),
                                self.get_node_start(&next_node_id).clone(),
                                );
                            let split_node_id = self.nodes.len();
                            self.nodes.insert(split_node_id.clone(), split_node);
                            self.set_node_child_id(active_point.active_edge.as_ref().unwrap(), &active_point.active_node, &split_node_id);
                            let tmp_start = self.get_node(&next_node_id).unwrap().get_start() + active_point.active_length;
                            self.set_node_start(&next_node_id, tmp_start);
                            self.set_node_parent_id(&next_node_id, &split_node_id);
                            self.add_suffix_link(&split_node_id, &mut need_suffix_link);
                            start_idx += 1;
                        }
                    },
                };
                if active_point.active_node == self.root && active_point.active_length > 0{
                    active_point.active_edge_index += 1;
                    active_point.active_edge = Some(seq[active_point.active_edge_index].clone());
                    active_point.active_length -= 1;
                }
                else if active_point.active_node != self.root{
                    active_point.active_node = self.get_suffix_link(&active_point.active_node).cloned().unwrap();
                }
                remainder -= 1
            }
            curr_pos +=1;
        }

        for leaf in string_leaves.iter(){
            let leaf_edge_length: usize = max_depth-self.get_node_depth(self.get_node_parent_id(leaf).unwrap());
            self.set_node_edge_length(leaf, leaf_edge_length);
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
