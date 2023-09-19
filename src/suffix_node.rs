use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::option::Option;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    children: HashMap<T, usize>,
    suffix_link: Option<usize>,
    string_id: Option<usize>,
    data: HashMap<usize, HashSet<usize>>,
    parent: Option<usize>,
    edge_length: usize,
    start: usize,
}

impl<T> Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub fn new(children: HashMap<T, usize>,
                suffix_link: Option<usize>,
                string_id: Option<usize>,
                data: HashMap<usize, HashSet<usize>>,
                parent: Option<usize>,
                edge_length: usize,
                start: usize)->Node<T>{
                    Node {
                        children: children,
                        suffix_link: suffix_link,
                        string_id: string_id,
                        data: data,
                        parent: parent,
                        edge_length: edge_length,
                        start: start,
                    }
                }

    pub fn set_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }

    pub fn get_parent(&self)->Option<&usize>{
        self.parent.as_ref()
    }

    pub fn set_suffix_link(&mut self, link_node:usize){
        self.suffix_link = Some(link_node);
    }

    pub fn get_suffix_link(&self)->Option<&usize>{
        self.suffix_link.as_ref()
    }

    pub fn add_seq(&mut self, seq_id:usize, start:usize){
        match self.data.get_mut(&seq_id){
            None => {self.data.insert(seq_id, HashSet::from([start]));},
            Some(i) => {
                match i.contains(&start){
                    false => {i.insert(start);},
                    true => {},
                };
            }
        };
    }

    pub fn get_child(&self, child:&T)->Option<&usize>{
        self.children.get(child)
    }

    pub fn get_child_mut(&mut self, child:&T)->Option<&mut usize>{
        self.children.get_mut(child)
    }
    
    pub fn set_child(&mut self, edge:T, child:usize){
        self.children.insert(edge, child);
    }

    pub fn set_edge_length(&mut self, edge_length:usize){
        self.edge_length = edge_length;
    }

    pub fn get_end(&self)->usize{
        self.start + self.edge_length - 1        
    }

    pub fn get_edge_length(&self)-> usize{
        self.edge_length
    }

    pub fn get_string_id(&self)->Option<&usize>{
        self.string_id.as_ref()
    }

    pub fn get_start(&self)->&usize{
        &self.start
    }

    pub fn set_string_id(&mut self, string_id:usize){
        self.string_id = Some(string_id);
    }

    pub fn set_start(&mut self, new_start:usize){
        self.edge_length = self.edge_length-(new_start-self.start);
        self.start = new_start;
    }

    pub fn has_children(&self)->bool{
        !self.children.is_empty()
    }

    pub fn get_children(&self)->&HashMap<T, usize>{
        &self.children
    }

    pub fn get_data(&self)->&HashMap<usize, HashSet<usize>>{
        &self.data
    }
}