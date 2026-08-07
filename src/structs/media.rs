use crate::traits::{HasTitle, HasID};

#[derive(Debug)]
pub struct Media {
    pub id: Option<u32>,
    pub title: String,
}

impl Media {
    /// Set id as None unless reading from the db
    pub fn new(id: Option<u32>, title: &str) -> Self{        
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
    fn id(&self) -> Option<u32> {
        self.id
    }
}