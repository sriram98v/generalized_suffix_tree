pub mod node;

use crate::data::tree_item::Character;
use crate::suffix_node::node::*;

use std::collections::HashMap;
use std::marker::PhantomData;
use std::fmt::{Display, Debug};
use std::fmt;
use std::hash::Hash;
use std::option::Option;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};


#[derive(Debug)]
pub struct Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd,
{
    children: HashMap<Character<T>, usize>,
    string_id: Option<usize>,
    parent: Option<usize>,
    edge_length: usize,
    start: usize,
}

impl<T> Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd,
{
    pub fn new(children: HashMap<Character<T>, usize>,
                string_id: Option<usize>,
                parent: Option<usize>,
                edge_length: usize,
                start: usize)->Self{
                    Self {
                        children,
                        string_id,
                        parent,
                        edge_length,
                        start,
                    }
                }
}

impl<T> SuffixNode<T> for Node<T>
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + PartialOrd,
{

    fn set_parent(&mut self, parent: usize){
        self.parent = Some(parent);
    }

    fn get_parent(&self)->Option<&usize>{
        self.parent.as_ref()
    }


    fn get_child(&self, child:&Character<T>)->Option<&usize>{
        self.children.get(child)
    }

    fn get_child_mut(&mut self, child:&Character<T>)->Option<&mut usize>{
        self.children.get_mut(child)
    }
    
    fn set_child(&mut self, edge:Character<T>, child:usize){
        self.children.insert(edge, child);
    }

    fn set_edge_length(&mut self, edge_length:usize){
        self.edge_length = edge_length;
    }

    fn get_end(&self)->usize{
        self.start + self.edge_length - 1        
    }

    fn get_edge_length(&self)-> usize{
        self.edge_length
    }

    fn get_string_id(&self)->Option<&usize>{
        self.string_id.as_ref()
    }

    fn get_start(&self)->&usize{
        &self.start
    }

    fn set_string_id(&mut self, string_id:usize){
        self.string_id = Some(string_id);
    }

    fn set_start(&mut self, new_start:usize){
        self.edge_length -= new_start-self.start;
        self.start = new_start;
    }

    fn has_children(&self)->bool{
        !self.children.is_empty()
    }

    fn get_children(&self)->&HashMap<Character<T>, usize>{
        &self.children
    }

    fn is_leaf(&self)->bool {
        self.children.is_empty()
    }

}

impl<T> Serialize for Node<T> 
where
    T: Display + Debug + Eq + PartialEq + Hash + Clone + Serialize + PartialOrd,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Node", 5)?;
        state.serialize_field("children", &self.children)?;
        state.serialize_field("string_id", &self.string_id)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("edge_length", &self.edge_length)?;
        state.serialize_field("start", &self.start)?;
        state.end()
    }
}

impl<'de, T> Deserialize<'de> for Node<T>
where
    T: Display + Debug + Eq + PartialEq + PartialOrd + Hash + Clone + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Children, StringID, Parent, EdgeLength, Start }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl Visitor<'_> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`children` or `string_id` or `parent` or `edge_length` or `start`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "children" => Ok(Field::Children),
                            "string_id" => Ok(Field::StringID),
                            "parent" => Ok(Field::Parent),
                            "edge_length" => Ok(Field::EdgeLength),
                            "start" => Ok(Field::Start),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor<K>(PhantomData<K>);

        impl<'de, K> Visitor<'de> for DurationVisitor<K> 
        where
            K: Display + Debug + Eq + PartialEq + PartialOrd + Hash + Clone + Serialize + Deserialize<'de>
        {
            type Value = Node<K>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Node")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Node<K>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let children = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let string_id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let parent = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let edge_length = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let start = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;                
                Ok(Node::new(children, string_id, parent, edge_length, start))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Node<K>, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut children = None;
                let mut string_id = None;
                let mut parent = None;
                let mut edge_length = None;
                let mut start = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Children => {
                            if children.is_some() {
                                return Err(de::Error::duplicate_field("children"));
                            }
                            children = Some(map.next_value()?);
                        }
                        Field::StringID => {
                            if string_id.is_some() {
                                return Err(de::Error::duplicate_field("string_id"));
                            }
                            string_id = Some(map.next_value()?);
                        }
                        Field::Parent => {
                            if parent.is_some() {
                                return Err(de::Error::duplicate_field("parent"));
                            }
                            parent = Some(map.next_value()?);
                        }
                        Field::EdgeLength => {
                            if edge_length.is_some() {
                                return Err(de::Error::duplicate_field("edge_length"));
                            }
                            edge_length = Some(map.next_value()?);
                        }
                        Field::Start => {
                            if start.is_some() {
                                return Err(de::Error::duplicate_field("start"));
                            }
                            start = Some(map.next_value()?);
                        }
                    }
                }
                let children = children.ok_or_else(|| de::Error::missing_field("children"))?;
                let string_id = string_id.ok_or_else(|| de::Error::missing_field("string_id"))?;
                let parent = parent.ok_or_else(|| de::Error::missing_field("parent"))?;
                let edge_length = edge_length.ok_or_else(|| de::Error::missing_field("edge_length"))?;
                let start = start.ok_or_else(|| de::Error::missing_field("start"))?;
                Ok(Node::new(children, string_id, parent, edge_length, start))
            }
        }

        const FIELDS: &[&str] = &["children", "string_id", "parent", "edge_length", "start"];
        deserializer.deserialize_struct("Node", FIELDS, DurationVisitor::<T>(PhantomData::<T>))
    }
}
