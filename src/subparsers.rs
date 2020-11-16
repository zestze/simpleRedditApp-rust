use crate::mapper;
use crate::models;

pub trait SubParser {
    fn update(&mut self, saved_posts: &models::ResponseData);
    fn print(&self);
}

pub fn newSubParsers(map: bool) -> Vec<Box<dyn SubParser>> {
    let mut v = Vec::new();
    if map {
        v.push(Box::new(mapper::Mapper::new()));
    }
    v
}
