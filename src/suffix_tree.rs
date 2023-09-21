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

    fn get_node(&self)->&usize{
        &self.active_node
    }

    fn set_node(&mut self, node_id: &usize){
        self.active_node = node_id.clone()
    }

    fn get_edge(&self)->&T{
        self.active_edge.as_ref().expect("Active edge is None!")
    }

    fn set_edge(&mut self, edge_label: T){
        self.active_edge = Some(edge_label);
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
                0,
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
            0,
            0
        ))]);
        self.strings = HashMap::new();
        self.leaves = Vec::new();
    }

    fn leaves_of_node(&self, node_id:&usize, leaves:&mut Vec<usize>){
        if !self.get_node(node_id).has_children(){
            leaves.push(node_id.clone());
        }

        for child_node_id in self.get_node(node_id).get_children().values(){
            self.leaves_of_node(child_node_id, leaves);
        }   
    }

    pub fn get_strings(&self)->&HashMap<usize, (TreeItem<T, U>, usize)>{
        &self.strings
    }

    pub fn get_nodes(&self)->&HashMap<usize, Node<T>>{
        &self.nodes
    }

    pub fn get_node(&self, node_id: &usize)->&Node<T>{
        self.nodes.get(node_id).expect("Node ID does not exist!")
    }

    fn get_suffix_link(&self, node_id: &usize) -> &usize{
        self.nodes.get(node_id).expect("Node ID does not exist!").get_suffix_link().unwrap_or(&0)
    }

    fn set_node_suffix_link(&mut self, node_id: &usize, suffix_link_node_id: &usize){
        self.get_node_mut(node_id).set_suffix_link(suffix_link_node_id.clone())
    }

    // fn get_string_id_by_treeitem_id(&self, treeitem_id: &usize)->Option<&U>{
    //     match self.strings.get(treeitem_id){
    //         None => None,
    //         Some(treeitem) => Some(treeitem.0.get_id())
    //     }
    // }

    fn get_string_by_treeitem_id(&self, treeitem_id: &usize)->&Vec<T>{
        self.strings.get(treeitem_id).expect("TreeItem ID does not exist!").0.get_string()
    }

    // fn is_leaf(&self, node_id: &usize)->bool{
    //     (!self.get_node(node_id).has_children()) && (self.get_node_parent_id(node_id)!=None)
    // }

    fn get_node_edge_length(&self, node_id: &usize)->usize{
        self.get_node(node_id).get_edge_length()
    }

    pub fn get_node_string(&self, node_id: &usize)->&Vec<T>{
        self.get_string_by_treeitem_id(self.get_node_string_id(node_id))
    }

    pub fn get_node_string_id(&self, node_id: &usize)->&usize{
        self.get_node(node_id).get_string_id().expect("Node ID is root node")
    }

    fn get_node_mut(&mut self, node_id: &usize)->&mut Node<T>{
        self.nodes.get_mut(node_id).expect("Node ID does not exist!")
    }

    pub fn get_root(&self)->&Node<T>{
        self.get_node(&0)
    }

    fn get_treeitem_by_treeitem_id(&self, treeitem_id: &usize)->&(TreeItem<T, U>, usize){
        self.strings.get(treeitem_id).expect("TreeItem ID does not exist!")
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

    pub fn is_suffix(&self, s:&[T])->bool{
        let mut query_string: Vec<T> = s.clone().to_vec();
        query_string.push(self.terminal_character.clone());
        match self.get_pattern_node(&query_string){
            None => false,
            Some(_) => true
        }
    }

    pub fn suffix_match(&self, s:&[T])-> HashMap<U, HashSet<usize>>{
        let mut query_string: Vec<T> = s.clone().to_vec();
        query_string.push(self.terminal_character.clone());
        return self.substring_match(&query_string);
    }

    pub fn substring_match(&self, s:&[T]) -> HashMap<U, HashSet<usize>>{
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
            for (treeitem_id, idx) in self.get_node(&leaf).get_data(){
                match ids_and_indexes.get_mut(treeitem_id){
                    None => {
                        if self.get_treeitem_by_treeitem_id(treeitem_id).1>=s.len(){
                            ids_and_indexes.insert(treeitem_id.clone(), idx.clone());
                        }   
                    },
                    Some(idxs) => {
                        if self.get_treeitem_by_treeitem_id(treeitem_id).1>=s.len(){
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

    pub fn get_node_children(&self, node_id: &usize)-> &HashMap<T, usize>{
        self.get_node(node_id).get_children()
    }

    pub fn get_node_child_id(&self, node_id: &usize, edge_label: &T)->Option<&usize>{
        self.get_node(node_id).get_child(edge_label)
    }

    pub fn set_node_child_id(&mut self, edge_label: &T, parent_node_id: &usize, child_node_id: &usize){
        self.get_node_mut(parent_node_id).set_child(edge_label.clone(), child_node_id.clone())
    }

    pub fn get_node_child(&self, node_id: &usize, edge_label: &T)->&Node<T>{
        self.get_node(self.get_node_child_id(node_id, edge_label).expect("No child!"))
    }

    pub fn get_node_parent(&self, node_id: &usize)->Option<&Node<T>>{
        match self.get_node(node_id).get_parent(){
            None => None,
            Some(parent_id) => Some(self.get_node(parent_id))
        }
    }

    pub fn get_node_parent_id(&self, node_id: &usize)->Option<&usize>{
        self.get_node(node_id).get_parent()
    }

    pub fn set_node_parent_id(&mut self, node_id: &usize, parent_id: &usize){
        self.get_node_mut(node_id).set_parent(parent_id.clone())
    }

    fn node_depth(&self, node_id: &usize, depth: usize)->usize{
        match self.get_node(node_id).get_parent(){
            None => return depth,
            Some(i) => return self.node_depth(i, depth+self.get_node_edge_length(node_id))
        };
    }

    fn get_node_depth(&self, node_id: &usize)->usize{
        self.node_depth(node_id, 0)
    }

    fn get_node_start(&self, node_id: &usize)->&usize{
        self.get_node(&node_id).get_start()
    }

    fn set_node_start(&mut self, node_id: &usize, start: usize){
        self.get_node_mut(&node_id).set_start(start)
    }

    fn add_suffix_link(&mut self, node_id: &usize, need_suffix_link: &mut Option<usize>){
        match need_suffix_link{
            None => (),
            Some(i) => self.set_node_suffix_link(i, node_id),
        };
        *need_suffix_link = Some(node_id.clone())
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
        while curr_pos <= seq.len()-1 {
            need_suffix_link = None;
            remainder += 1;
            while remainder > 0{
                if active_point.active_length == 0{
                    active_point.active_edge_index = curr_pos;
                    active_point.active_edge = Some(seq[curr_pos].clone());
                }
                let next_node = self.get_node(&active_point.active_node).get_child(active_point.active_edge.as_ref().unwrap()).cloned();
                match next_node{
                    None => {
                        if self.get_node_depth(&active_point.active_node)+active_point.active_length<max_depth{
                            let new_leaf_node: Node<T> = Node::new(
                                HashMap::new(),
                                Some(0),
                                Some(new_string_id),
                                HashMap::from([(new_string_id, HashSet::from([(start_idx.clone())]))]),
                                Some(active_point.active_node.clone()),
                                cmp::min(seq.len()-curr_pos,max_depth-self.get_node_depth(&active_point.active_node)),
                                curr_pos.clone(),
                            );
                            let new_leaf_node_id = self.nodes.len();
                            self.nodes.insert(new_leaf_node_id.clone(), new_leaf_node);
                            self.set_node_child_id(active_point.active_edge.as_ref().unwrap(), &active_point.active_node, &new_leaf_node_id);
                            self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                            let active_node_data = self.get_node(&active_point.active_node).get_data().clone();
                            self.get_node_mut(&new_leaf_node_id).add_data(active_node_data);
                            
                            if self.get_node(&active_point.active_node).get_children().len()-1==0 && active_point.active_node!=0{
                                active_point.active_length = curr_pos-(start_idx+1);
                                active_point.active_edge_index = start_idx+1;
                                active_point.active_edge = Some(seq[active_point.active_edge_index].clone());
                            }
                        }
                        else if self.get_node_depth(&active_point.active_node)+active_point.active_length==max_depth{
                            if !self.get_node(&active_point.active_node).has_children(){
                                self.get_node_mut(&active_point.active_node).add_seq(&new_string_id, &start_idx);
                            }
                            else{
                                let active_node_children = self.get_node_children(&active_point.active_node).clone();
                                for child in active_node_children.values(){
                                    self.get_node_mut(child).add_seq(&new_string_id, &start_idx)
                                }
                            }
                            let old_active_node = self.get_node_parent_id(&active_point.active_node).unwrap();
                            active_point.active_length += self.get_node_edge_length(&active_point.active_node);
                            active_point.active_edge_index -= self.get_node_edge_length(&active_point.active_node);
                            active_point.active_edge = Some(seq[active_point.active_edge_index].clone());
                            active_point.active_node = old_active_node.clone();
                        }
                        else{
                            self.get_node_mut(&active_point.active_node).add_seq(&new_string_id, &start_idx);
                            active_point.active_length = curr_pos-start_idx-1;
                            active_point.active_edge_index = start_idx +1;
                            active_point.active_edge = Some(seq[active_point.active_edge_index].clone());
                        }
                        start_idx += 1;
                    },
                    Some(next_node_id) => {
                        if self.get_node_edge_length(&next_node_id)<=active_point.active_length{
                            active_point.active_length -= self.get_node_edge_length(&next_node_id);
                            active_point.active_edge_index += self.get_node_edge_length(&next_node_id);
                            active_point.active_edge = Some(seq[active_point.active_edge_index].clone());
                            active_point.active_node = next_node_id.clone();
                            continue;
                        }
                        else if self.get_node_string(&next_node_id)[self.get_node_start(&next_node_id) + active_point.active_length] == seq[curr_pos]{
                            if seq[curr_pos] == self.terminal_character{
                                self.get_node_mut(&next_node_id).add_seq(&new_string_id, &start_idx);
                                start_idx += 1;
                                self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                            }
                            else{
                                active_point.active_length += 1;
                                self.add_suffix_link(&active_point.active_node, &mut need_suffix_link);
                                break;
                            }
                        }
                        else{
                            if self.get_node_depth(&active_point.active_node)+active_point.active_length<=max_depth{
                                let split_node:Node<T> = Node::new(
                                        HashMap::from([
                                            (self.get_node_string(&next_node_id)[self.get_node_start(&next_node_id) + active_point.active_length].clone(), next_node_id.clone())
                                            ]),
                                        None,
                                        Some(self.get_node_string_id(&next_node_id).clone()),
                                        HashMap::from([(new_string_id, HashSet::from([(start_idx.clone())]))]),
                                        Some(active_point.active_node.clone()),
                                        active_point.active_length,
                                        self.get_node_start(&next_node_id).clone(),
                                        );
                                let split_node_id = self.nodes.len();
                                self.nodes.insert(split_node_id.clone(), split_node);
                                self.set_node_child_id(active_point.active_edge.as_ref().unwrap(), &active_point.active_node, &split_node_id);
                                let next_node_new_start = self.get_node_start(&next_node_id) + active_point.active_length;
                                self.set_node_start(&next_node_id, next_node_new_start);
                                self.set_node_parent_id(&next_node_id, &split_node_id);
                                let leaf_node: Node<T> = Node::new(
                                    HashMap::new(),
                                    Some(0),
                                    Some(new_string_id),
                                    HashMap::from([(new_string_id, HashSet::from([(start_idx.clone())]))]),
                                    Some(split_node_id.clone()),
                                    cmp::min(seq.len()-curr_pos, max_depth-self.get_node_depth(&split_node_id)),
                                    curr_pos.clone(),
                                );
                                let leaf_node_id = self.nodes.len();
                                self.nodes.insert(leaf_node_id.clone(), leaf_node);
                                self.set_node_child_id(&seq[curr_pos], &split_node_id, &leaf_node_id);

                                if self.get_node_depth(&split_node_id)<max_depth && need_suffix_link!=Some(active_point.active_node){// last node inserted in curr phase must not be a child of the active node
                                    self.add_suffix_link(&split_node_id, &mut need_suffix_link);
                                }
                                else{
                                    self.add_suffix_link(&0, &mut need_suffix_link);
                                    need_suffix_link = None;
                                }
                            }
                            else{// the split node would have to be placed past the max depth. Here we instead insert the data to the next node
                                self.get_node_mut(&next_node_id).add_seq(&new_string_id, &start_idx);
                            }
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
                    active_point.active_node = self.get_suffix_link(&active_point.active_node).clone();
                }
                remainder -= 1
            }
            curr_pos +=1;
        }
        
    }

    pub fn contains(&self, string_id: &U)->bool{
        let string_ids: HashSet<&U> = self.strings.values().map(|x| x.0.get_id()).collect();
        string_ids.contains(string_id)
    }

    pub fn print_tree(&self){
        todo!()
    }
    
}
