use crate::suffix_node::node::*;
use crate::suffix_node::Node;
use super::node_iter::PostOrdNodes;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::{Display, Debug};

pub struct PostOrdEdges<T: PartialEq + Display + Debug>
{
    node_iter: PostOrdNodes<T>,
    s_links: HashMap<NodeID, NodeID>,
    parents: HashMap<NodeID, NodeID>,
    stack: Vec<(NodeID, NodeID)>
}

impl<T> PostOrdEdges<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone
{
    pub fn new(start_node_id: &NodeID, nodes: &HashMap<NodeID, Node<T>>, s_links: HashMap<NodeID, NodeID>, parents: HashMap<NodeID, NodeID>)->Self{
        Self {
            node_iter: PostOrdNodes::new(start_node_id, nodes),
            s_links: s_links.into_iter()
                            .filter(|(_k, v)| v!=&0)
                            .map(|(k, v)| (v, k))
                            .collect(),
            parents,
            stack: Vec::new()
        }
    }

    pub fn len(&self)->usize{
        self.parents.len()+self.s_links.len()-1
    }
}

impl<T> Iterator for PostOrdEdges<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone
{
    type Item = (NodeID, NodeID);

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop(){
            Some(edge) => Some(edge),
            None => {
                match self.node_iter.next(){
                    Some(node_id) => {
                        let node_id_parent = self.parents.get(&node_id).unwrap();
                        if let Some(slink_node_id) = self.s_links.get(&node_id) { self.stack.push((*slink_node_id, node_id)) }
                        Some((*node_id_parent, node_id))
                    },
                    None => None
                }
            }
        }
    }
}