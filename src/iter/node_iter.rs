use crate::suffix_node::node::*;
use crate::suffix_node::Node;
use std::collections::HashMap;
use std::hash::Hash;
use crate::data::tree_item::Character;
use core::fmt::{Debug, Display};
use itertools::Itertools;

pub struct EulerWalk<T: PartialEq + Display + Debug + PartialOrd>
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashMap<Character<T>, NodeID>>
}

impl<T> EulerWalk<T>
where
    T: Display + Debug + Eq + PartialEq + PartialOrd + Hash + Clone
{
    pub fn new(start_node_id: &NodeID, nodes: &HashMap<NodeID, Node<T>>)->Self{
        Self { stack:vec![*start_node_id], nodes: nodes.iter().map(|(edge_label, child_node)| {
            (*edge_label, child_node.get_children().clone())
        }).collect::<HashMap<NodeID, HashMap<Character<T>, NodeID>>>() }
    }
}

impl<T> Iterator for EulerWalk<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        match self.stack.pop() {
            Some(node_id) => {
                let children_ids:Vec<&NodeID> = self.nodes.get(&node_id).expect("Invalid Node ID!").values().collect();
                for child_node_id in children_ids.into_iter().sorted(){
                    self.stack.push(*child_node_id);
                    self.stack.push(node_id);
            }
            Some(node_id)
            }
            None => None,
        }
    }
}

pub struct PreOrdNodes<T: PartialEq + Display + Debug + PartialOrd>
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashMap<Character<T>, NodeID>>
}

impl<T> PreOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd
{
    pub fn new(start_node_id: &NodeID, nodes: &HashMap<NodeID, Node<T>>)->Self{
        Self { stack:vec![*start_node_id], nodes: nodes.iter().map(|(edge_label, child_node)| {
            (*edge_label, child_node.get_children().clone())
        }).collect::<HashMap<NodeID, HashMap<Character<T>, NodeID>>>() }
    }
}

impl<T> Iterator for PreOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        match self.stack.pop() {
            Some(node_id) => {
                let children_ids:Vec<&NodeID> = self.nodes.get(&node_id).expect("Invalid Node ID!").values().collect();
                for child_node_id in children_ids.into_iter().sorted(){
                    self.stack.push(*child_node_id)
            }
            Some(node_id)
            }
            None => None,
        }
    }
}

pub struct PostOrdNodes<T: PartialEq + Display + Debug + PartialOrd>
{
    stack: Vec<NodeID>,
    nodes: HashMap<NodeID, HashMap<Character<T>, NodeID>>
}

impl<T> PostOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd
{
    pub fn new(start_node_id: &NodeID, nodes: &HashMap<NodeID, Node<T>>)->Self{
        Self { stack:vec![*start_node_id], nodes: nodes.iter().map(|(edge_label, child_node)| {
            (*edge_label, child_node.get_children().clone())
        }).collect::<HashMap<NodeID, HashMap<Character<T>, NodeID>>>() }
    }
}

impl<T> Iterator for PostOrdNodes<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd
{
    type Item = NodeID;

    fn next(&mut self)->Option<Self::Item>{
        while let Some(node_id) = self.stack.pop()  {
            if self.nodes.contains_key(&node_id){
                self.stack.push(node_id);
                let children = self.nodes.remove(&node_id).unwrap();
                for child_id in children.values().sorted(){
                    self.stack.push(*child_id)
                }
            }
            else{
                return Some(node_id)
            }
        }
        None
    }
}