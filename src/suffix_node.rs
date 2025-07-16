pub mod node;

use crate::data::tree_item::Alphabet;
use crate::suffix_node::node::*;

#[cfg(feature = "non_crypto_hash")]
use fxhash::FxHashMap as HashMap;
#[cfg(not(feature = "non_crypto_hash"))]
use std::collections::HashMap;

use std::fmt::Debug;
use std::option::Option;

pub type NodeID = usize;

#[derive(Debug, Default)]
pub struct Node<T>
where
    T: Alphabet,
{
    children: HashMap<T, usize>,
    parent: Option<usize>,
}

impl<T> Node<T>
where
    T: Alphabet,
{
    pub fn new(children: HashMap<T, usize>,
                parent: Option<usize>)->Self{
                    Self {
                        children,
                        parent,
                    }
                }
}

impl<T> SuffixNode for Node<T>
where
    T: Alphabet,
{
    type Characters = T;
    type NodeID = NodeID;

    fn set_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }

    fn get_parent(&self)->Option<&usize>{
        self.parent.as_ref()
    }


    fn get_child(&self, child:&T)->Option<&NodeID>{
        self.children.get(child)
    }

    fn get_child_mut(&mut self, child:&T)->Option<&mut usize>{
        self.children.get_mut(child)
    }
    
    fn set_child(&mut self, edge:Self::Characters, child:usize){
        self.children.insert(edge, child);
    }

    fn has_children(&self)->bool{
        !self.children.is_empty()
    }

    fn get_children(&self)->&HashMap<T, usize>{
        &self.children
    }

    fn is_leaf(&self)->bool {
        self.children.is_empty()
    }

}