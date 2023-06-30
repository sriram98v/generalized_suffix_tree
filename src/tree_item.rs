#[derive(Debug, Clone)]
pub struct TreeItem<T, U>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
    U: std::cmp::Eq + std::hash::Hash + Clone,
{
    string: Vec<T>,
    string_id: U,
}

impl<'a, T, U> TreeItem<T, U> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone, 
    U: std::cmp::Eq + std::hash::Hash + Clone, 
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