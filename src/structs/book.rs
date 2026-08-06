use crate::structs::media::Media;
use crate::traits::{HasTitle, HasID};


#[derive(Debug)]
pub struct Book {
    pub media: Media,
    pub author: String,
    pub page_count: u32,
}

impl Book{
    pub fn new(media: Media, author: String, page_count: u32) -> Self{
        Book{
            media: media,
            author: author,
            page_count: page_count
        }
    }
}

impl HasTitle for Book {
    fn title(&self) -> &str {
        self.media.title.as_str()
    }   
}
impl HasID for Book {
    fn id(&self) -> u32 {
        self.media.id
    }
}