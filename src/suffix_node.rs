use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::option::Option;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    pub children: HashMap<T, usize>,
    pub suffix_link: Option<usize>,
    pub string_id: Option<usize>,
    pub data: HashMap<usize, Vec<usize>>,
    pub parent: Option<usize>,
    pub end: Option<usize>,
    pub start: usize,
}

impl<T> Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone,
{
    // pub fn new(start:usize, end: Option<usize>)-> Node<T, U>{
    //     Node{
    //         children: None,
    //         suffix_link: None,
    //         parent:None,
    //         data: None,
    //         string_id: None,
    //         end,
    //         start,
    //     }
    // }

    pub fn set_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }

    pub fn get_parent(&self)->Option<&usize>{
        self.parent.as_ref()
    }

    pub fn set_suffix_link(&mut self, link_node:usize){
        self.suffix_link = Some(link_node);
    }

    pub fn get_suffix_link(&self)->Option<&usize>{
        self.suffix_link.as_ref()
    }

    pub fn add_seq(&mut self, seq_id:usize, start:usize){
        self.data.entry(seq_id).or_default().push(start);
    }

    pub fn get_child(&self, child:&T)->Option<&usize>{
        self.children.get(child)
    }

    pub fn get_child_mut(&mut self, child:&T)->Option<&mut usize>{
        self.children.get_mut(child)
    }
    
    pub fn set_child(&mut self, edge:T, child:usize){
        self.children.insert(edge, child);
    }

    pub fn set_end(&mut self, end:usize){
        self.end = Some(end);
    }

    pub fn get_end(&self, default_end:&usize)->usize{
        match self.end{
            None => default_end.clone(),
            Some(x) => x,
        }
    }

    pub fn edge_length(&self, default_end:&usize)-> usize{
        self.get_end(default_end) + 1 - self.get_start()
    }

    pub fn get_string_id(&self)->Option<&usize>{
        self.string_id.as_ref()
    }

    pub fn get_start(&self)->&usize{
        &self.start
    }

    pub fn set_string_id(&mut self, string_id:usize){
        self.string_id = Some(string_id);
    }

    pub fn set_start(&mut self, new_start:usize){
        self.start = new_start;
    }

    pub fn has_children(&self)->bool{
        !self.children.is_empty()
    }

    pub fn get_children(&self)->&HashMap<T, usize>{
        &self.children
    }

    pub fn get_data(&self)->&HashMap<usize, Vec<usize>>{
        &self.data
    }
}