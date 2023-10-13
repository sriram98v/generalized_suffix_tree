
pub trait TreeItem<T, U>{
    fn new(k: U, v: Vec<T>)->Self;
    fn get_string(&self) -> &Vec<T>;
    fn get_id(&self) -> &U;
}