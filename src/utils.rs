use serde::{Serialize, Deserialize};
use std::hash::Hash;
use std::cmp;
use std::fmt::{Display, Debug};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Enode<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    node_label: Vec<T>,
    children: HashMap<T, usize>,
    data: HashMap<U, HashSet<usize>>,
    edge_length: usize,
}

impl<T, U> Enode<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
    U: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
{
    pub fn new(node_label: Vec<T>,
        children: HashMap<T, usize>,
        data: HashMap<U, HashSet<usize>>,
        edge_length: usize) -> Enode<T, U>{
            Enode{
                node_label: node_label,
                children: children,
                data: data,
                edge_length: edge_length,
            }
        }
}