use std::fmt;
use core::fmt::{Debug, Display};
use std::hash::Hash;
use serde::{Serialize, Deserialize};

#[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct TreeItem<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash
{
    string: Vec<T>,
    id: U,
}


impl<T, U> TreeItem<T, U> 
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash
{
    pub fn new(k: U, v: Vec<T>)->TreeItem<T, U>{
        TreeItem { string: v, id: k }
    }

    pub fn get_string(&self) -> &Vec<T>{
        &self.string
    }

    pub fn get_id(&self) -> &U{
        &self.id
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