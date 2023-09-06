use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Debug};
use std::hash::Hash;
use std::option::Option;
use std::rc::Rc;

use crate::tree_item::TreeItem;

#[derive(PartialEq)]
pub struct Node<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash,
{
    children: HashMap<T, Option<Rc<RefCell<Node<T, U>>>>>,
    suffix_link: Option<Rc<RefCell<Node<T, U>>>>,
    string_id: Option<Rc<TreeItem<T, U>>>,
    data: HashMap<Rc<TreeItem<T, U>>, Vec<(usize, Option<usize>)>>,
    parent: Option<Rc<RefCell<Node<T, U>>>>,
    end: Option<usize>,
    start: usize,
}

impl<T, U> Node<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash,
{
    pub fn new(start:usize, end: Option<usize>)-> Node<T, U>{
        Node{
            children: HashMap::new(),
            suffix_link: None,
            parent:None,
            data: HashMap::new(),
            string_id: None,
            end,
            start,
        }
    }

    pub fn add_parent(&mut self, parent: Rc<RefCell<Node<T, U>>>){
        self.parent = Some(parent);
    }

    pub fn set_suffix_link(&mut self, link_node:Rc<RefCell<Node<T, U>>>){
        self.suffix_link = Some(link_node);
    }

    pub fn get_suffix_link(&self)->Option<Rc<RefCell<Node<T, U>>>>{
        self.suffix_link.clone()
    }

    pub fn add_seq(&mut self, seq_id:Rc<TreeItem<T, U>>, start:usize){
        self.data.entry(seq_id).or_default().push((start, None));
    }

    pub fn get_child(&self, child:&T)->Option<Rc<RefCell<Node<T, U>>>>{
        self.children.get(child).unwrap().clone()
    }
    
    pub fn set_child(&mut self, edge:T, child:Rc<RefCell<Node<T, U>>>){
        self.children.insert(edge, Some(child));
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

    pub fn get_string_id(&self)->Option<Rc<TreeItem<T, U>>>{
        self.string_id.clone()
    }

    pub fn get_start(&self)->&usize{
        &self.start
    }

    pub fn set_string_id(&mut self, string_id:Rc<TreeItem<T, U>>){
        self.string_id = Some(string_id);
    }

    pub fn set_start(&mut self, new_start:usize){
        self.start = new_start;
    }

    pub fn has_children(&self)->bool{
        !self.children.is_empty()
    }

    pub fn get_children(&self)->&HashMap<T, Option<Rc<RefCell<Node<T, U>>>>>{
        &self.children
    }

    pub fn get_data(&self)->&HashMap<Rc<TreeItem<T, U>>, Vec<(usize, Option<usize>)>>{
        &self.data
    }
}

impl<T, U> Drop for Node<T, U>
where
    T: Display + Debug + Eq + PartialEq + Hash,
    U: Display + Debug + Eq + PartialEq + Hash,
{
    fn drop(&mut self) { }
}