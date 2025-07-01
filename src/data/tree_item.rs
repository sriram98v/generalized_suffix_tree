use crate::suffix_node::node::NodeID;
use serde::{Serialize, Deserialize};
use std::{cmp::Ordering, fmt};
use core::fmt::{Debug, Display};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub enum Character<T: PartialEq + Display + Debug + PartialOrd>{
    Char(T),
    Terminal
}

impl<T> Display for Character<T>
where
    T: PartialEq + Display + Debug + PartialOrd
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
    T: PartialEq + Display + Debug + PartialOrd
{
    pub fn is_terminal(&self)->bool{
        matches!(&self, Character::Terminal)
    }

    pub fn into_inner(&self)->Option<&T>{
        match self {
            Character::Char(x) => Some(x),
            _ => None,
        }
    }
}

impl<T> PartialOrd for Character<T>
where
    T: PartialEq + Display + Debug + PartialOrd
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self{
            Character::Terminal => {
                match other{
                    Character::Terminal => Some(Ordering::Equal),
                    _ => Some(Ordering::Less)
                }
            },
            Character::Char(t) => {
                match other{
                    Character::Terminal => Some(Ordering::Greater),
                    Character::Char(o) => t.partial_cmp(o)
                }
            }
        }
        // return None;
    }
}


pub trait TreeItem<T, U>
where
    T: PartialEq + Display + Debug + PartialOrd
{
    fn new(k: U, v: Vec<T>)->Self;
    fn get_string(&self) -> &[Character<T>];
    fn get_id(&self) -> &U;
    fn get_nodes(&self) -> impl ExactSizeIterator<Item= &NodeID>;
    fn add_data_to_node(&mut self, node_id: &NodeID);
}