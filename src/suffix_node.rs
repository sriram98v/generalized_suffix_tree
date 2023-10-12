pub mod node;

use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::option::Option;
use serde::{Serialize, Deserialize};
use crate::suffix_node::node::*;


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    children: HashMap<T, usize>,
    string_id: Option<usize>,
    parent: Option<usize>,
    edge_length: usize,
    start: usize,
}

impl<T> Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub fn new(children: HashMap<T, usize>,
                string_id: Option<usize>,
                parent: Option<usize>,
                edge_length: usize,
                start: usize)->Self{
                    Self {
                        children,
                        string_id,
                        parent,
                        edge_length,
                        start,
                    }
                }
}

impl<T> SuffixNode<T> for Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{

    fn set_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }

    fn get_parent(&self)->Option<&usize>{
        self.parent.as_ref()
    }


    fn get_child(&self, child:&T)->Option<&usize>{
        self.children.get(child)
    }

    fn get_child_mut(&mut self, child:&T)->Option<&mut usize>{
        self.children.get_mut(child)
    }
    
    fn set_child(&mut self, edge:T, child:usize){
        self.children.insert(edge, child);
    }

    fn set_edge_length(&mut self, edge_length:usize){
        self.edge_length = edge_length;
    }

    fn get_end(&self)->usize{
        self.start + self.edge_length - 1        
    }

    fn get_edge_length(&self)-> usize{
        self.edge_length
    }

    fn get_string_id(&self)->Option<&usize>{
        self.string_id.as_ref()
    }

    fn get_start(&self)->&usize{
        &self.start
    }

    fn set_string_id(&mut self, string_id:usize){
        self.string_id = Some(string_id);
    }

    fn set_start(&mut self, new_start:usize){
        self.edge_length -= new_start-self.start;
        self.start = new_start;
    }

    fn has_children(&self)->bool{
        !self.children.is_empty()
    }

    fn get_children(&self)->&HashMap<T, usize>{
        &self.children
    }

    fn is_leaf(&self)->bool {
        self.children.is_empty()
    }

}