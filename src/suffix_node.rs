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

#[derive(Debug, Clone, Serialize)]
pub struct Node<T>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    children: HashMap<T, isize>,
    suffix_link: Option<isize>,
    string_id: Option<usize>,
    data: LinkedList<Data>,
    parent: Option<isize>,
    end: Option<isize>,
    start: isize,
}

impl<'a, T> Node<T> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone, 
{
    pub fn new(start:isize, end: Option<isize>)-> Node<T>{
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

    pub fn add_parent(&mut self, parent: isize){
        self.parent = Some(parent);
    }

    pub fn set_suffix_link(&mut self, link_node:isize){
        self.suffix_link = Some(link_node);
    }

    pub fn get_suffix_link(&self)->Option<isize>{
        self.suffix_link
    }

    pub fn add_seq(&mut self, seq_id:usize, start:usize){
        self.data.push_back(Data::new(seq_id, start));
    }

    pub fn get_child(&self, child:Option<T>)->Option<isize>{
        match child{
            None => None,
            Some(i) => self.children.get(&i).copied(),
            }
    }
    
    pub fn set_child(&mut self, edge:T, child:isize){
        self.children.insert(edge, child);
    }

    pub fn set_end(&mut self, end:isize){
        self.end = Some(end);
    }

    pub fn get_end(&self, default_end:isize)->isize{
        match self.end{
            None => default_end,
            Some(x) => x,
        }
    }

    pub fn edge_length(&self, default_end:isize)-> isize{
        self.get_end(default_end) + 1 - self.start
    }

    pub fn get_string_id(&self)->Option<usize>{
        self.string_id.clone()
    }

    pub fn get_start(&self)->isize{
        self.start
    }

    pub fn set_string_id(&mut self, string_id:usize){
        self.string_id = Some(string_id);
    }

    pub fn set_start(&mut self, new_start:isize){
        self.start = new_start;
    }

    pub fn has_children(&self)->bool{
        match self.children.is_empty(){
            true => false,
            false => true,
        }
    }

    pub fn get_children(&self)->HashMap<T, isize>{
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