use crate::{data::tree_item::Character, iter::node_iter::*, suffix_node::node::*};
use core::fmt::{Debug, Display};
use std::collections::LinkedList;

pub trait SuffixTree<T>{
    fn root(&self)->&NodeID;
    fn is_leaf(&self, node_id: &NodeID)->bool;
    fn get_node_child(&self, node_id: &NodeID, edge_label: &T)->Option<&NodeID>;
    fn get_node_parent(&self, node_id: &NodeID)->Option<&NodeID>;
    fn get_node_depth(&self, node_id: &NodeID)->usize;
    fn get_suffix_link(&self, node_id: &NodeID) -> &usize;
    fn get_node_label<'a>(&'a self, node_id: &'a NodeID)->Vec<T>;
    fn get_node_path_label(&self, node_id: &NodeID)->&[T];
    fn get_node_path_pre(&self, node_id: &NodeID)->LinkedList<NodeID>;
    fn get_node_path_post(&self, node_id: &NodeID)->LinkedList<NodeID>;
    /// Checks if the input slice is a suffix of any of the strings present in the tree.
    fn is_suffix(&self, s:&[T])->bool;
    }

pub trait Tree<T: PartialEq + Display + Debug + PartialOrd>{
    fn iter_nodes_pre(&self, node_id: &NodeID)->PreOrdNodes<Character<T>>;
    fn mrca(&self, nodes: Vec<NodeID>)->&NodeID;
    fn leaf_pairwise_mrca_matrix(&self)->Vec<Vec<&NodeID>>;
}