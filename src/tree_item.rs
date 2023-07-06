use std::sync::Arc;
use serde::ser::{Serialize, Serializer, SerializeStruct};


#[derive(Debug, Clone)]
pub struct TreeItem<T, U>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
    U: std::cmp::Eq + std::hash::Hash + Clone,
{
    string: Arc<[T]>,
    string_id: U,
}

impl<'a, T, U> TreeItem<T, U> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone, 
    U: std::cmp::Eq + std::hash::Hash + Clone, 
{
    pub fn new(string: Arc<[T]>, string_id: U) -> TreeItem<T, U>{
        TreeItem{
            string: string,
            string_id: string_id,
        }
    }

    pub fn get_string(&self)->&Arc<[T]>{
        &self.string
    }

    pub fn get_id(&self)->&U{
        &self.string_id
    }
}

impl<T, U> Serialize for TreeItem<T, U> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone + Serialize,
    U: std::cmp::Eq + std::hash::Hash + Clone + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("TreeItem", 2)?;
        state.serialize_field("string", &self.string.to_vec())?;
        state.serialize_field("string_id", &self.string_id)?;
        state.end()
    }
}