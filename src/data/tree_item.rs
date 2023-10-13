use crate::suffix_node::node::NodeID;
pub trait TreeItem<T, U>{
    fn new(k: U, v: Vec<T>)->Self;
    fn get_string(&self) -> &Vec<T>;
    fn get_id(&self) -> &U;
    fn get_nodes(&self) -> &Vec<NodeID>;
    fn add_data_to_node(&mut self, node_id: &NodeID);
}