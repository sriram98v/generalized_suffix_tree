pub mod tree_item;

use std::fmt;
use core::fmt::{Debug, Display};
use std::hash::Hash;
use serde::{Serialize, Deserialize};
use crate::data::tree_item::TreeItem as OtherTreeItem;
use crate::suffix_node::node::NodeID;


#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TreeItem<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash
{
    string: Vec<T>,
    id: U,
    nodes: Vec<NodeID>,
}


impl<T, U> OtherTreeItem<T, U> for TreeItem<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash
{
    fn new(k: U, v: Vec<T>)->Self{
        TreeItem { string: v, id: k , nodes: vec![]}
    }

    fn get_string(&self) -> &Vec<T>{
        &self.string
    }

    fn get_id(&self) -> &U{
        &self.id
    }

    fn get_nodes(&self) -> &Vec<NodeID> {
        &self.nodes
    }

    fn add_data_to_node(&mut self, node_id: &NodeID) {
        self.nodes.push(*node_id)
    }
}


impl<T, U> Display for TreeItem<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "String ID: {}", self.get_id())?;
        write!(f, "String: ")?;
        for v in self.get_string() {
            write!(f, "{}", v)?;
        }
        writeln!(f)?;
        Ok(())
    }

}