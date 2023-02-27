use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeItem<T, U>
where
    T: std::cmp::Eq + std::hash::Hash + Clone + Serialize,
    U: std::cmp::Eq + std::hash::Hash + Clone + Serialize,
{
    string: Vec<T>,
    string_id: U,
}

impl<'a, T, U> TreeItem<T, U> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone + Serialize + Deserialize<'a>, 
    U: std::cmp::Eq + std::hash::Hash + Clone + Serialize + Deserialize<'a>, 
{
    pub fn new(string: Vec<T>, string_id: U) -> TreeItem<T, U>{
        TreeItem{
            string: string,
            string_id: string_id,
        }
    }

    pub fn get_string(&self)->&Vec<T>{
        &self.string
    }

    pub fn get_id(&self)->&U{
        &self.string_id
    }
}