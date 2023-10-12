use crate::suffix_node::node::*;
use std::collections::LinkedList;

pub trait SuffixTree<T>{
    fn root(&self)->&NodeID;
    fn is_leaf(&self, node_id: &NodeID)->bool;
    fn get_node_child(&self, node_id: &NodeID, edge_label: &T)->Option<&NodeID>;
    fn get_node_parent(&self, node_id: &NodeID)->Option<&NodeID>;
    fn get_node_depth(&self, node_id: &NodeID)->usize;
    fn get_suffix_link(&self, node_id: &NodeID) -> &usize;
    fn get_node_label(&self, node_id: &NodeID)->&[T];
    fn get_node_path_label(&self, node_id: &NodeID)->&[T];
    fn get_node_path(&self, node_id: &NodeID)->LinkedList<NodeID>;
    /// Checks if the input slice is a suffix of any of the strings present in the tree.
    fn is_suffix(&self, s:&[T])->bool;
    }