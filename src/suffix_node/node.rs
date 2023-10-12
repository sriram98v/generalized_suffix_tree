use std::collections::HashMap;

pub type NodeID = usize;
pub type StringID = usize;

pub trait SuffixNode<T> {
    fn set_parent(&mut self, parent: NodeID);
    fn get_parent(&self)->Option<&NodeID>;
    fn get_child(&self, child:&T)->Option<&NodeID>;
    fn get_child_mut(&mut self, child:&T)->Option<&mut NodeID>;
    fn set_child(&mut self, edge:T, child:NodeID);
    fn set_edge_length(&mut self, edge_length:usize);
    fn get_end(&self)->usize;
    fn get_edge_length(&self)-> usize;
    fn get_string_id(&self)->Option<&StringID>;
    fn set_string_id(&mut self, string_id:StringID);
    fn get_start(&self)->&usize;
    fn set_start(&mut self, new_start:usize);
    fn has_children(&self)->bool;
    fn get_children(&self)->&HashMap<T, NodeID>;
    fn is_leaf(&self)->bool;
}