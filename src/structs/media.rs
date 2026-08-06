use crate::traits::{HasTitle, HasID};

#[derive(Debug)]
pub struct Media {
    pub id: u32,
    pub title: String,
}

impl Media {
    pub fn new(id: u32, title: &str) -> Self{
        Media{
            id: id,
            title: title.to_string()
        }
    }
}

impl HasTitle for Media {
    fn title(&self) -> &str {
        self.title.as_str()
    }   
}
impl HasID for Media {
    fn id(&self) -> u32 {
        self.id
    }
}