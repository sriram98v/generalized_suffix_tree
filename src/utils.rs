use serde::{Serialize, Deserialize};
use std::hash::Hash;
use std::fmt::{Display, Debug};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Enode<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
    U: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    children: HashMap<T, usize>,
    data: Vec<U>,
    node_id: usize,
    slink: Option<usize>
}

impl<T, U> Enode<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
    U: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize,
{
    pub fn new(children: HashMap<T, usize>,
                data: Vec<U>,
                slink: Option<usize>,
                node_id: usize) -> Enode<T, U>{
            Enode{
                children,
                data,
                node_id,
                slink
            }
        }
}