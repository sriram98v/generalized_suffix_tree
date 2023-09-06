use core::fmt::{Debug, self};
use std::{fmt::Display, hash::Hash};

#[derive(Debug, Hash, Eq, PartialEq)]
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
    pub fn new(id: U, string: Vec<T>)->TreeItem<T, U>{
        TreeItem { string: string, id: id }
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
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }

}