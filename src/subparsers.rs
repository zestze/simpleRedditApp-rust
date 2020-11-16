use crate::mapper;
use crate::models;

pub type List = Vec<Box<dyn SubParser>>;

pub trait SubParser {
    fn update(&mut self, saved_posts: &models::ResponseData);
    fn print(&self);
}

pub fn new(map: bool) -> List {
    let mut v = List::new();
    if map {
        v.push(Box::new(mapper::Mapper::new()));
    }
    v
}
