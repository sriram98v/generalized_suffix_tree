use std::collections::HashMap;
use std::option::Option;
use serde::{Serialize, Deserialize};
use std::collections::LinkedList;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data{
    string_id: usize,
    start_idx: usize,
}

impl Data{
    pub fn new(string_id: usize, start_idx: usize)->Data{
        Data { string_id: string_id, start_idx: start_idx }
    }
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    children: HashMap<T, usize>,
    suffix_link: Option<usize>,
    string_id: Option<usize>,
    data: LinkedList<Data>,
    parent: Option<usize>,
    end: Option<usize>,
    start: usize,
}

impl<'a, T> Node<T> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone, 
{
    pub fn new(start:usize, end: Option<usize>)-> Node<T>{
        Node{
            children: HashMap::new(),
            suffix_link: None,
            parent:None,
            data: LinkedList::new(),
            string_id: None,
            end,
            start,
        }
    }

    pub fn add_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }

    pub fn set_suffix_link(&mut self, link_node:usize){
        self.suffix_link = Some(link_node);
    }

    pub fn get_suffix_link(&self)->Option<usize>{
        self.suffix_link
    }

    pub fn add_seq(&mut self, seq_id:usize, start:usize){
        self.data.push_back(Data::new(seq_id, start));
    }

    pub fn get_child(&self, child:Option<T>)->Option<usize>{
        match child{
            None => None,
            Some(i) => self.children.get(&i).copied(),
            }
    }
    
    pub fn set_child(&mut self, edge:T, child:usize){
        self.children.insert(edge, child);
    }

    pub fn set_end(&mut self, end:usize){
        self.end = Some(end);
    }

    pub fn get_end(&self, default_end:usize)->usize{
        match self.end{
            None => default_end,
            Some(x) => x,
        }
    }

    pub fn edge_length(&self, default_end:usize)-> usize{
        self.get_end(default_end) + 1 - self.start
    }

    pub fn get_string_id(&self)->Option<usize>{
        self.string_id.clone()
    }

    pub fn get_start(&self)->usize{
        self.start
    }

    pub fn set_string_id(&mut self, string_id:usize){
        self.string_id = Some(string_id);
    }

    pub fn set_start(&mut self, new_start:usize){
        self.start = new_start;
    }

    pub fn has_children(&self)->bool{
        match self.children.is_empty(){
            true => false,
            false => true,
        }
    }

    pub fn get_children(&self)->HashMap<T, usize>{
        self.children.clone()
    }

    pub fn get_data(&self)->Vec<(&usize, &usize)>{
        let mut data = Vec::new();
        for item in self.data.iter(){
            data.push((&item.string_id, &item.start_idx));
        }
        data
    }
}