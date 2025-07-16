#[cfg(feature = "non_crypto_hash")]
use fxhash::FxHashMap as HashMap;
#[cfg(not(feature = "non_crypto_hash"))]
use std::collections::HashMap;

use crate::data::tree_item::Alphabet;
use std::{fmt::{Display, Debug}, hash::Hash};

pub trait SuffixNode: Default
where
    Self::Characters: Alphabet,
    Self::NodeID: Display + Debug + Hash + Ord + PartialEq + Eq + Copy
{
    type Characters;
    type NodeID;    

    fn set_parent(&mut self, parent: Self::NodeID);
    fn get_parent(&self)->Option<&Self::NodeID>;
    fn get_child(&self, child:&Self::Characters)->Option<&Self::NodeID>;
    fn get_child_mut(&mut self, child:&Self::Characters)->Option<&mut Self::NodeID>;
    fn set_child(&mut self, edge:Self::Characters, child:Self::NodeID);
    fn has_children(&self)->bool;
    fn get_children(&self)->&HashMap<Self::Characters, Self::NodeID>;
    fn is_leaf(&self)->bool;
}