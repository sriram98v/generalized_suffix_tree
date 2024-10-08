use crate::suffix_node::node::NodeID;
use serde::{Serialize, Deserialize};
use std::fmt;
use core::fmt::{Debug, Display};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Character<T: PartialEq + Display + Debug>{
    Char(T),
    Terminal
}

impl<T> Display for Character<T>
where
    T: PartialEq + Display + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Character::Char(t) => write!(f, "{}", t),
            Character::Terminal => write!(f, ""),
        }
    }
}

impl<T> Character<T>
where
    T: PartialEq + Display + Debug
{
    pub fn is_terminal(&self)->bool{
        match self{
            Character::Terminal => return true,
            _ => return false,
        }
    }

    pub fn into_inner(&self)->Option<&T>{
        match self {
            Character::Char(x) => return Some(x),
            _ => return None,
        };
    }
}

pub trait TreeItem<T, U>
where
    T: PartialEq + Display + Debug
{
    fn new(k: U, v: Vec<T>)->Self;
    fn get_string<'a>(&'a self) -> &'a [Character<T>];
    fn get_id<'a>(&'a self) -> &'a U;
    fn get_nodes(&self) -> impl ExactSizeIterator<Item= &NodeID>;
    fn add_data_to_node(&mut self, node_id: &NodeID);
}