use crate::models;
use crate::subparsers;
use std::collections::HashMap;

//TODO: maybe mapper isn't the best name lol
pub struct Mapper {
    map: HashMap<String, u32>
}

impl Mapper {
    pub fn new() -> Mapper {
        return Mapper{map: HashMap::new()}
    }
}

impl subparsers::SubParser for Mapper {
    fn update(&mut self, saved_posts: &models::ResponseData) {
        for child in saved_posts.children.iter() {
            let subreddit_name = child.data.subreddit.to_string();
            self.map.entry(subreddit_name)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    fn print(&self) {
        let mut list: Vec<(&String, &u32)> = self.map.iter().collect();
        list.sort_by(|a, b| b.1.cmp(a.1));

        for (name, count) in list {
            println!("{:<30}: {}", name, count);
        }
    }
}
