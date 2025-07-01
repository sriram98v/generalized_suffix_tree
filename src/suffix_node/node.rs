use std::collections::HashMap;
use crate::data::tree_item::Character;
use core::fmt::{Debug, Display};

pub type NodeID = usize;
pub type StringID = usize;

pub trait SuffixNode<T> 
where
    T: PartialEq + Display + Debug + PartialOrd
{
    fn set_parent(&mut self, parent: NodeID);
    fn get_parent(&self)->Option<&NodeID>;
    fn get_child(&self, child:&Character<T>)->Option<&NodeID>;
    fn get_child_mut(&mut self, child:&Character<T>)->Option<&mut NodeID>;
    fn set_child(&mut self, edge:Character<T>, child:NodeID);
    fn set_edge_length(&mut self, edge_length:usize);
    fn get_end(&self)->usize;
    fn get_edge_length(&self)-> usize;
    fn get_string_id(&self)->Option<&StringID>;
    fn set_string_id(&mut self, string_id:StringID);
    fn get_start(&self)->&usize;
    fn set_start(&mut self, new_start:usize);
    fn has_children(&self)->bool;
    fn get_children(&self)->&HashMap<Character<T>, NodeID>;
    fn is_leaf(&self)->bool;
}